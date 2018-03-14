#[derive(Debug, Serialize, Deserialize)]
pub struct Karabiner {
    pub global: ::serde_json::Value,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub selected: bool,
    pub fn_function_keys: ::serde_json::Value,
    pub devices: ::serde_json::Value,
    pub virtual_hid_keyboard: ::serde_json::Value,
    pub simple_modifications: Vec<SimpleModification>,
    pub complex_modifications: ComplexModifications,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleModification {
    pub from: KeyCode,
    pub to: KeyCode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyCode {
    pub key_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexModifications {
    pub parameters: ::serde_json::Value,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub description: String,
    pub manipulators: Vec<Manipulator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manipulator {
    #[serde(rename="type")]
    pub type_: String,
    pub from: From,
    pub to: Vec<To>,
    #[serde(skip_serializing_if="Vec::is_empty", default)]
    pub to_if_alone: Vec<To>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum From {
    Key {
        key_code: String,
        #[serde(skip_serializing_if="FromModifiers::is_empty", default)]
        modifiers: FromModifiers,
    },
    Button {
        pointing_button: String,
        #[serde(skip_serializing_if="FromModifiers::is_empty", default)]
        modifiers: FromModifiers,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FromModifiers {
    #[serde(skip_serializing_if="Vec::is_empty", default)]
    pub mandatory: Vec<String>,
    #[serde(skip_serializing_if="Vec::is_empty", default)]
    pub optional: Vec<String>,
}

impl FromModifiers {
    fn is_empty(&self) -> bool {
        self.mandatory.is_empty() && self.optional.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum To {
    Key {
        key_code: String,
        #[serde(skip_serializing_if="Vec::is_empty", default)]
        modifiers: Vec<String>,
    },
    Button {
        pointing_button: String,
        #[serde(skip_serializing_if="Vec::is_empty", default)]
        modifiers: Vec<String>,
    },
}

pub enum KeyOrButton {
    Key(String),
    Button(String),
}

pub trait KeyOrButtonConv {
    fn conv(key_or_button: KeyOrButton, mods: Vec<String>) -> Self;
}

impl KeyOrButtonConv for From {
    fn conv(key_or_button: KeyOrButton, mods: Vec<String>) -> Self {
        match key_or_button {
            KeyOrButton::Key(s) =>
                From::Key {
                    key_code: s,
                    modifiers: FromModifiers {
                        mandatory: mods,
                        optional: vec![],
                    },
                },

            KeyOrButton::Button(s) =>
                From::Button {
                    pointing_button: s,
                    modifiers: FromModifiers {
                        mandatory: mods,
                        optional: vec![],
                    },
                },
        }
    }
}

impl KeyOrButtonConv for To {
    fn conv(key_or_button: KeyOrButton, mods: Vec<String>) -> Self {
        match key_or_button {
            KeyOrButton::Key(s) =>
                To::Key {
                    key_code: s,
                    modifiers: mods,
                },

            KeyOrButton::Button(s) =>
                To::Button {
                    pointing_button: s,
                    modifiers: mods,
                },
        }
    }
}

