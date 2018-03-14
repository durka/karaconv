//! Structs necessary for deserializing Karabiner XML format

/// Root element
#[derive(Debug, Deserialize)]
pub struct Karabiner {
    #[serde(rename="item")]
    pub items: Vec<Item>,
}

/// Ruleset
#[derive(Debug, Deserialize)]
pub struct Item {
    /// Short name (transferred to JSON)
    pub name: String,

    /// Longer description (ignored)
    pub appendix: String,

    /// Slug identifier (ignored)
    pub identifier: String,

    /// Key replacements
    #[serde(rename="autogen")]
    pub keys: Vec<String>,
}

