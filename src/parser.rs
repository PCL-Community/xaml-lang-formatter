use std::collections::HashSet;

use anyhow::{bail, Context, Result};

use crate::model::{is_key_attr_name, Attr, Document, ResourceItem, FORMATTER_COMMENT_PREFIX};

pub fn parse_document(input: &str) -> Result<Document> {
    let input = input.strip_prefix('\u{feff}').unwrap_or(input);
    let mut pos = skip_whitespace(input, 0);

    let xml_decl = if starts_with_at(input, pos, "<?xml") {
        let end = input[pos..]
            .find("?>")
            .map(|idx| pos + idx + 2)
            .context("unterminated XML declaration")?;
        let decl = input[pos..end].trim().to_string();
        pos = end;
        Some(decl)
    } else {
        None
    };

    let mut existing_formatter_timestamp = None;
    skip_ignorable(input, &mut pos, &mut existing_formatter_timestamp)?;

    let (root_name, root_attrs, root_self_closing, next_pos) =
        parse_start_tag(input, pos).context("failed to parse ResourceDictionary root element")?;

    if local_name(&root_name) != "ResourceDictionary" {
        bail!("expected ResourceDictionary root element, found <{root_name}>");
    }

    if root_self_closing {
        bail!("ResourceDictionary must not be self-closing");
    }

    pos = next_pos;
    let mut items = Vec::new();

    loop {
        skip_ignorable(input, &mut pos, &mut existing_formatter_timestamp)?;

        if pos >= input.len() {
            bail!("missing closing </{root_name}> element");
        }

        if starts_with_at(input, pos, "</") {
            let (closing_name, next_pos) = parse_end_tag(input, pos)?;
            if closing_name != root_name {
                bail!("expected closing </{root_name}>, found </{closing_name}>");
            }
            pos = next_pos;
            break;
        }

        if !starts_with_at(input, pos, "<") {
            let rest = &input[pos..];
            if rest.trim().is_empty() {
                break;
            }
            bail!("unexpected non-whitespace content inside <{root_name}>");
        }

        let (tag_name, attrs, self_closing, content_start) = parse_start_tag(input, pos)
            .with_context(|| format!("failed to parse child element at byte offset {pos}"))?;

        let key = attrs
            .iter()
            .find(|attr| is_key_attr_name(&attr.name))
            .map(|attr| attr.value.clone())
            .ok_or_else(|| {
                anyhow::anyhow!("unsupported direct child element <{tag_name}> without x:Key")
            })?;

        let text;
        if self_closing {
            text = String::new();
            pos = content_start;
        } else {
            let closing_pattern = format!("</{tag_name}>");
            let closing_start = input[content_start..]
                .find(&closing_pattern)
                .map(|idx| content_start + idx)
                .with_context(|| format!("missing closing </{tag_name}> element"))?;
            let raw_text = &input[content_start..closing_start];

            if raw_text.contains('<') {
                bail!(
                    "unsupported nested XML or CDATA inside <{tag_name} x:Key=\"{key}\">; \
                     this formatter MVP only supports plain text resources"
                );
            }

            text = unescape_xml(raw_text)
                .with_context(|| format!("failed to decode text for x:Key {key}"))?;
            pos = closing_start + closing_pattern.len();
        }

        items.push(ResourceItem {
            tag_name,
            attrs,
            key,
            text,
        });
    }

    skip_ignorable(input, &mut pos, &mut existing_formatter_timestamp)?;

    if !input[pos..].trim().is_empty() {
        bail!("unexpected content after </{root_name}>");
    }

    validate_unique_keys(&items)?;

    if items.is_empty() {
        bail!("no direct ResourceDictionary child elements with x:Key were found");
    }

    Ok(Document {
        xml_decl,
        existing_formatter_timestamp,
        root_name,
        root_attrs,
        items,
    })
}

pub fn validate_unique_keys(items: &[ResourceItem]) -> Result<()> {
    let mut seen = HashSet::new();

    for item in items {
        if !seen.insert(item.key.clone()) {
            bail!("duplicate x:Key: {}", item.key);
        }
    }

    Ok(())
}

fn parse_start_tag(input: &str, pos: usize) -> Result<(String, Vec<Attr>, bool, usize)> {
    if !starts_with_at(input, pos, "<") {
        bail!("expected start tag");
    }

    if starts_with_at(input, pos, "</")
        || starts_with_at(input, pos, "<!--")
        || starts_with_at(input, pos, "<?")
        || starts_with_at(input, pos, "<!")
    {
        bail!("expected XML element start tag");
    }

    let end = find_tag_end(input, pos)?;
    let mut inner = input[pos + 1..end].trim().to_string();
    let self_closing = inner.ends_with('/');

    if self_closing {
        inner.pop();
        inner = inner.trim_end().to_string();
    }

    let (name, attrs) = parse_tag_inner(&inner)?;

    Ok((name, attrs, self_closing, end + 1))
}

fn parse_end_tag(input: &str, pos: usize) -> Result<(String, usize)> {
    if !starts_with_at(input, pos, "</") {
        bail!("expected end tag");
    }

    let end = find_tag_end(input, pos)?;
    let inner = input[pos + 2..end].trim();

    if inner.is_empty() {
        bail!("empty end tag");
    }

    if inner.chars().any(char::is_whitespace) {
        bail!("invalid end tag: </{inner}>");
    }

    Ok((inner.to_string(), end + 1))
}

fn parse_tag_inner(inner: &str) -> Result<(String, Vec<Attr>)> {
    let bytes = inner.as_bytes();
    let mut i = 0;

    skip_ascii_whitespace(bytes, &mut i);

    let name_start = i;
    while i < bytes.len() && !bytes[i].is_ascii_whitespace() && bytes[i] != b'/' {
        i += 1;
    }

    if name_start == i {
        bail!("missing element name");
    }

    let name = inner[name_start..i].to_string();
    let mut attrs = Vec::new();

    loop {
        skip_ascii_whitespace(bytes, &mut i);

        if i >= bytes.len() {
            break;
        }

        let attr_name_start = i;
        while i < bytes.len()
            && !bytes[i].is_ascii_whitespace()
            && bytes[i] != b'='
            && bytes[i] != b'/'
        {
            i += 1;
        }

        if attr_name_start == i {
            bail!("invalid attribute in <{name}>");
        }

        let attr_name = inner[attr_name_start..i].to_string();

        skip_ascii_whitespace(bytes, &mut i);

        if i >= bytes.len() || bytes[i] != b'=' {
            bail!("attribute {attr_name} in <{name}> is missing '='");
        }
        i += 1;

        skip_ascii_whitespace(bytes, &mut i);

        if i >= bytes.len() || (bytes[i] != b'\"' && bytes[i] != b'\'') {
            bail!("attribute {attr_name} in <{name}> must use quotes");
        }

        let quote = bytes[i];
        i += 1;
        let value_start = i;

        while i < bytes.len() && bytes[i] != quote {
            i += 1;
        }

        if i >= bytes.len() {
            bail!("unterminated attribute {attr_name} in <{name}>");
        }

        let raw_value = &inner[value_start..i];
        i += 1;

        attrs.push(Attr {
            name: attr_name,
            value: unescape_xml(raw_value)?,
        });
    }

    Ok((name, attrs))
}

fn find_tag_end(input: &str, start: usize) -> Result<usize> {
    let mut quote = None;

    for (rel, ch) in input[start + 1..].char_indices() {
        match quote {
            Some(q) if ch == q => quote = None,
            Some(_) => {}
            None if ch == '\"' || ch == '\'' => quote = Some(ch),
            None if ch == '>' => return Ok(start + 1 + rel),
            None => {}
        }
    }

    bail!("unterminated XML tag")
}

fn skip_ignorable(
    input: &str,
    pos: &mut usize,
    existing_formatter_timestamp: &mut Option<String>,
) -> Result<()> {
    loop {
        *pos = skip_whitespace(input, *pos);

        if starts_with_at(input, *pos, "<!--") {
            let end = input[*pos + 4..]
                .find("-->")
                .map(|idx| *pos + 4 + idx)
                .context("unterminated XML comment")?;
            let comment = input[*pos + 4..end].trim();

            if let Some(timestamp) = parse_formatter_timestamp(comment) {
                *existing_formatter_timestamp = Some(timestamp);
            }

            *pos = end + 3;
            continue;
        }

        if starts_with_at(input, *pos, "<?") {
            let end = input[*pos..]
                .find("?>")
                .map(|idx| *pos + idx + 2)
                .context("unterminated processing instruction")?;
            *pos = end;
            continue;
        }

        break;
    }

    Ok(())
}

fn parse_formatter_timestamp(comment: &str) -> Option<String> {
    comment
        .strip_prefix(FORMATTER_COMMENT_PREFIX)
        .and_then(|rest| rest.strip_suffix('.'))
        .map(str::trim)
        .filter(|timestamp| !timestamp.is_empty())
        .map(ToOwned::to_owned)
}

fn skip_whitespace(input: &str, pos: usize) -> usize {
    for (rel, ch) in input[pos..].char_indices() {
        if !ch.is_whitespace() {
            return pos + rel;
        }
    }

    input.len()
}

fn skip_ascii_whitespace(bytes: &[u8], i: &mut usize) {
    while *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
        *i += 1;
    }
}

fn starts_with_at(input: &str, pos: usize, needle: &str) -> bool {
    input
        .get(pos..)
        .map(|rest| rest.starts_with(needle))
        .unwrap_or(false)
}

fn local_name(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}

fn unescape_xml(value: &str) -> Result<String> {
    let mut output = String::with_capacity(value.len());
    let mut rest = value;

    while let Some(amp_rel) = rest.find('&') {
        output.push_str(&rest[..amp_rel]);
        let after_amp = &rest[amp_rel + 1..];
        let semi_rel = after_amp
            .find(';')
            .with_context(|| format!("unterminated XML entity in {value:?}"))?;
        let entity = &after_amp[..semi_rel];

        match entity {
            "amp" => output.push('&'),
            "lt" => output.push('<'),
            "gt" => output.push('>'),
            "quot" => output.push('"'),
            "apos" => output.push('\''),
            _ if entity.starts_with("#x") || entity.starts_with("#X") => {
                let code = u32::from_str_radix(&entity[2..], 16)
                    .with_context(|| format!("invalid XML entity &{entity};"))?;
                let ch = char::from_u32(code)
                    .with_context(|| format!("invalid XML character code &{entity};"))?;
                output.push(ch);
            }
            _ if entity.starts_with('#') => {
                let code = entity[1..]
                    .parse::<u32>()
                    .with_context(|| format!("invalid XML entity &{entity};"))?;
                let ch = char::from_u32(code)
                    .with_context(|| format!("invalid XML character code &{entity};"))?;
                output.push(ch);
            }
            _ => bail!("unsupported XML entity &{entity};"),
        }

        rest = &after_amp[semi_rel + 1..];
    }

    output.push_str(rest);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_resource_dictionary() {
        let input = r#"<?xml version="1.0" encoding="utf-8"?>
<!-- Formatted by xaml-lang-formatter at 2026-06-28T15:21:30. -->
<ResourceDictionary xmlns="x" xmlns:x="y">
    <!-- Common -->
    <sys:String x:Key="Common.App">App &amp; More</sys:String>
</ResourceDictionary>
"#;

        let doc = parse_document(input).unwrap();

        assert_eq!(
            doc.xml_decl.as_deref(),
            Some(r#"<?xml version="1.0" encoding="utf-8"?>"#)
        );
        assert_eq!(
            doc.existing_formatter_timestamp.as_deref(),
            Some("2026-06-28T15:21:30")
        );
        assert_eq!(doc.root_name, "ResourceDictionary");
        assert_eq!(doc.root_attrs.len(), 2);
        assert_eq!(doc.items.len(), 1);
        assert_eq!(doc.items[0].key, "Common.App");
        assert_eq!(doc.items[0].text, "App & More");
    }

    #[test]
    fn rejects_nested_resource_items() {
        let input = r#"<ResourceDictionary>
    <sys:String x:Key="Common.App"><Run>App</Run></sys:String>
</ResourceDictionary>"#;

        let err = parse_document(input).unwrap_err().to_string();

        assert!(err.contains("unsupported nested XML"));
    }
}
