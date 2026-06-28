pub const TOOL_NAME: &str = "xaml-lang-formatter";
pub const FORMATTER_COMMENT_PREFIX: &str = "Formatted by xaml-lang-formatter at ";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub xml_decl: Option<String>,
    pub existing_formatter_timestamp: Option<String>,
    pub root_name: String,
    pub root_attrs: Vec<Attr>,
    pub items: Vec<ResourceItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attr {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceItem {
    pub tag_name: String,
    pub attrs: Vec<Attr>,
    pub key: String,
    pub text: String,
}

pub fn is_key_attr_name(name: &str) -> bool {
    name == "x:Key" || name.ends_with(":Key")
}
