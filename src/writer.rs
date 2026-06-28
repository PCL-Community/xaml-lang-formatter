use anyhow::{Result, bail};

use crate::grouping::{KeyNode, build_key_tree, collect_leaf_items, sorted_top_groups};
use crate::model::{Document, ResourceItem, TOOL_NAME, is_key_attr_name};

pub fn write_document_with_group_threshold(
    doc: &Document,
    timestamp: &str,
    group_threshold: usize,
) -> Result<String> {
    if group_threshold == 0 {
        bail!("group threshold must be greater than 0");
    }

    let root = build_key_tree(&doc.items)?;
    let mut s = String::new();

    if let Some(decl) = &doc.xml_decl {
        s.push_str(decl);
        s.push('\n');
    }

    s.push_str(&format!(
        "<!-- Formatted by {TOOL_NAME} at {timestamp}. -->\n"
    ));

    s.push('<');
    s.push_str(&doc.root_name);

    for attr in &doc.root_attrs {
        s.push_str(&format!(
            "\n    {}=\"{}\"",
            attr.name,
            escape_attr(&attr.value)
        ));
    }

    s.push_str(">\n");

    for group in sorted_top_groups(&root) {
        emit_group_xml(group, &doc.items, &mut s, group_threshold);
    }

    ensure_blank_line(&mut s);
    s.push_str(&format!("</{}>\n", doc.root_name));

    Ok(s)
}

fn emit_group_xml(node: &KeyNode, items: &[ResourceItem], s: &mut String, group_threshold: usize) {
    ensure_blank_line(s);
    s.push_str(&format!(
        "    <!-- {} -->\n\n",
        escape_comment(&node.prefix)
    ));

    let mut direct_items = Vec::new();
    let mut child_groups = Vec::new();

    if let Some(idx) = node.item_index {
        direct_items.push(idx);
    }

    for child in node.children.values() {
        if child.item_count >= group_threshold {
            child_groups.push(child);
        } else {
            collect_leaf_items(child, &mut direct_items);
        }
    }

    direct_items.sort_by_key(|idx| items[*idx].key.clone());

    for idx in direct_items {
        write_item(&items[idx], s);
    }

    for child in child_groups {
        emit_group_xml(child, items, s, group_threshold);
    }
}

fn write_item(item: &ResourceItem, s: &mut String) {
    s.push_str("    <");
    s.push_str(&item.tag_name);

    s.push_str(&format!(" x:Key=\"{}\"", escape_attr(&item.key)));

    for attr in &item.attrs {
        if is_key_attr_name(&attr.name) {
            continue;
        }

        s.push_str(&format!(" {}=\"{}\"", attr.name, escape_attr(&attr.value)));
    }

    s.push('>');
    s.push_str(&escape_text(&item.text));
    s.push_str("</");
    s.push_str(&item.tag_name);
    s.push_str(">\n");
}

fn ensure_blank_line(s: &mut String) {
    if s.ends_with("\n\n") {
        return;
    }

    if s.ends_with('\n') {
        s.push('\n');
    } else {
        s.push_str("\n\n");
    }
}

pub fn escape_attr(value: &str) -> String {
    escape_xml(value, EscapeMode::Attribute)
}

pub fn escape_text(value: &str) -> String {
    escape_xml(value, EscapeMode::Text)
}

#[derive(Debug, Clone, Copy)]
enum EscapeMode {
    Attribute,
    Text,
}

fn escape_xml(value: &str, mode: EscapeMode) -> String {
    let mut escaped = String::with_capacity(value.len());

    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '\n' => escaped.push_str("&#xA;"),
            '\r' => escaped.push_str("&#xD;"),
            '\t' if matches!(mode, EscapeMode::Attribute) => escaped.push_str("&#x9;"),
            '"' if matches!(mode, EscapeMode::Attribute) => escaped.push_str("&quot;"),
            _ => escaped.push(ch),
        }
    }

    escaped
}

fn escape_comment(value: &str) -> String {
    let mut escaped = value.replace("--", "- -");

    if escaped.ends_with('-') {
        escaped.push(' ');
    }

    escaped
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grouping::DEFAULT_GROUP_THRESHOLD;
    use crate::model::{Attr, Document, ResourceItem};

    fn doc(items: Vec<ResourceItem>) -> Document {
        Document {
            xml_decl: None,
            existing_formatter_timestamp: None,
            root_name: "ResourceDictionary".to_string(),
            root_attrs: vec![
                Attr {
                    name: "xmlns".to_string(),
                    value: "http://schemas.microsoft.com/winfx/2006/xaml/presentation".to_string(),
                },
                Attr {
                    name: "xmlns:x".to_string(),
                    value: "http://schemas.microsoft.com/winfx/2006/xaml".to_string(),
                },
            ],
            items,
        }
    }

    pub fn write_document(doc: &Document, timestamp: &str) -> Result<String> {
        write_document_with_group_threshold(doc, timestamp, DEFAULT_GROUP_THRESHOLD)
    }

    fn item(key: &str, text: &str) -> ResourceItem {
        ResourceItem {
            tag_name: "sys:String".to_string(),
            attrs: vec![Attr {
                name: "x:Key".to_string(),
                value: key.to_string(),
            }],
            key: key.to_string(),
            text: text.to_string(),
        }
    }

    #[test]
    fn writes_single_deep_key_under_top_level_group() {
        let output = write_document(
            &doc(vec![item("Common.Settings.Window.Width", "Width")]),
            "2026-06-28T15:21:30",
        )
        .unwrap();

        assert!(output.contains("    <!-- Common -->"));
        assert!(!output.contains("    <!-- Common.Settings -->"));
        assert!(!output.contains("    <!-- Common.Settings.Window -->"));
    }

    #[test]
    fn default_threshold_does_not_split_two_repeated_deep_prefixes() {
        let output = write_document(
            &doc(vec![
                item("Common.Settings.Window.Width", "Width"),
                item("Common.Settings.Window.Height", "Height"),
            ]),
            "2026-06-28T15:21:30",
        )
        .unwrap();

        assert!(output.contains("    <!-- Common -->"));
        assert!(!output.contains("    <!-- Common.Settings -->"));
        assert!(!output.contains("    <!-- Common.Settings.Window -->"));
    }

    #[test]
    fn default_threshold_splits_when_prefix_reaches_five_items() {
        let output = write_document(
            &doc(vec![
                item("Common.Settings.Window.Width", "Width"),
                item("Common.Settings.Window.Height", "Height"),
                item("Common.Settings.Window.Left", "Left"),
                item("Common.Settings.Window.Top", "Top"),
                item("Common.Settings.Window.State", "State"),
            ]),
            "2026-06-28T15:21:30",
        )
        .unwrap();

        assert!(output.contains("    <!-- Common -->"));
        assert!(output.contains("    <!-- Common.Settings -->"));
        assert!(output.contains("    <!-- Common.Settings.Window -->"));
    }

    #[test]
    fn configurable_threshold_splits_repeated_deep_prefixes() {
        let output = write_document_with_group_threshold(
            &doc(vec![
                item("Common.Settings.Window.Width", "Width"),
                item("Common.Settings.Window.Height", "Height"),
            ]),
            "2026-06-28T15:21:30",
            2,
        )
        .unwrap();

        assert!(output.contains("    <!-- Common -->"));
        assert!(output.contains("    <!-- Common.Settings -->"));
        assert!(output.contains("    <!-- Common.Settings.Window -->"));
    }

    #[test]
    fn rejects_zero_group_threshold() {
        let err = write_document_with_group_threshold(
            &doc(vec![item("Common.App", "App")]),
            "2026-06-28T15:21:30",
            0,
        )
        .unwrap_err()
        .to_string();

        assert!(err.contains("group threshold must be greater than 0"));
    }

    #[test]
    fn escapes_attributes_and_text() {
        let output = write_document(
            &doc(vec![item("Common.Save", "Save & Close <Now>")]),
            "2026-06-28T15:21:30",
        )
        .unwrap();

        assert!(output.contains("Save &amp; Close &lt;Now&gt;"));
    }

    #[test]
    fn escapes_text_newlines() {
        let output = write_document(
            &doc(vec![item("Common.Multiline", "Line 1\nLine 2\r\nLine 3")]),
            "2026-06-28T15:21:30",
        )
        .unwrap();

        assert!(output.contains("Line 1&#xA;Line 2&#xD;&#xA;Line 3"));
    }

    #[test]
    fn escapes_attribute_newlines() {
        let mut resource = item("Common.WithAttribute", "Value");
        resource.attrs.push(Attr {
            name: "Comment".to_string(),
            value: "Line 1\nLine 2".to_string(),
        });

        let output = write_document(&doc(vec![resource]), "2026-06-28T15:21:30").unwrap();

        assert!(output.contains("Comment=\"Line 1&#xA;Line 2\""));
    }
}
