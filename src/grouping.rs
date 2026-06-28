use std::collections::BTreeMap;

use anyhow::{bail, Result};

use crate::model::ResourceItem;

pub const DEFAULT_GROUP_THRESHOLD: usize = 8;

#[derive(Debug, Default)]
pub struct KeyNode {
    pub segment: String,
    pub prefix: String,
    pub item_index: Option<usize>,
    pub children: BTreeMap<String, KeyNode>,
    pub item_count: usize,
}

pub fn build_key_tree(items: &[ResourceItem]) -> Result<KeyNode> {
    let mut root = KeyNode::default();

    for (idx, item) in items.iter().enumerate() {
        insert_key(&mut root, &item.key, idx)?;
    }

    Ok(root)
}

pub fn insert_key(root: &mut KeyNode, key: &str, item_index: usize) -> Result<()> {
    let key = key.trim();

    if key.is_empty() {
        bail!("x:Key must not be empty");
    }

    let parts: Vec<&str> = key.split('.').collect();

    if parts.iter().any(|part| part.is_empty()) {
        bail!("x:Key contains an empty segment: {key}");
    }

    let mut node = root;
    node.item_count += 1;

    let mut prefix = String::new();

    for part in parts {
        if !prefix.is_empty() {
            prefix.push('.');
        }
        prefix.push_str(part);

        node = node
            .children
            .entry(part.to_string())
            .or_insert_with(|| KeyNode {
                segment: part.to_string(),
                prefix: prefix.clone(),
                ..Default::default()
            });

        node.item_count += 1;
    }

    if node.item_index.is_some() {
        bail!("duplicate x:Key: {key}");
    }

    node.item_index = Some(item_index);

    Ok(())
}

pub fn sorted_top_groups(root: &KeyNode) -> Vec<&KeyNode> {
    let mut groups: Vec<&KeyNode> = root.children.values().collect();

    groups.sort_by(|a, b| match (a.segment.as_str(), b.segment.as_str()) {
        ("Meta", "Meta") => std::cmp::Ordering::Equal,
        ("Meta", _) => std::cmp::Ordering::Less,
        (_, "Meta") => std::cmp::Ordering::Greater,
        _ => a.segment.cmp(&b.segment),
    });

    groups
}

pub fn collect_leaf_items(node: &KeyNode, out: &mut Vec<usize>) {
    if let Some(idx) = node.item_index {
        out.push(idx);
    }

    for child in node.children.values() {
        collect_leaf_items(child, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ResourceItem;

    fn item(key: &str) -> ResourceItem {
        ResourceItem {
            tag_name: "sys:String".to_string(),
            attrs: Vec::new(),
            key: key.to_string(),
            text: String::new(),
        }
    }

    #[test]
    fn meta_is_sorted_first() {
        let items = vec![item("Common.App"), item("Meta.Name"), item("About.Title")];
        let root = build_key_tree(&items).unwrap();
        let names: Vec<&str> = sorted_top_groups(&root)
            .into_iter()
            .map(|node| node.segment.as_str())
            .collect();

        assert_eq!(names, vec!["Meta", "About", "Common"]);
    }

    #[test]
    fn duplicate_keys_are_rejected() {
        let items = vec![item("Common.App"), item("Common.App")];
        let err = build_key_tree(&items).unwrap_err().to_string();

        assert!(err.contains("duplicate x:Key"));
    }
}
