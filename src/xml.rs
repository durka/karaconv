#[derive(Debug, Deserialize)]
pub struct Karabiner {
    #[serde(rename="item")]
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
    pub appendix: String,
    pub identifier: String,

    #[serde(rename="autogen")]
    pub keys: Vec<String>,
}

