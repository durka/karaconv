//! Structs necessary for de/serializing Karabiner-Elements JSON format

/// Root element
#[derive(Debug, Serialize, Deserialize)]
pub struct Karabiner {
    /// Some settings
    pub global: ::serde_json::Value,

    /// User profiles (we always operate on the first one)
    pub profiles: Vec<Profile>,
}

/// User profile
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub selected: bool,
    pub fn_function_keys: ::serde_json::Value,
    pub devices: ::serde_json::Value,
    pub virtual_hid_keyboard: ::serde_json::Value,
    pub simple_modifications: Vec<SimpleModification>,

    /// We always convert XML rulesets to complex modifications
    pub complex_modifications: ComplexModifications,
}

/// Simple rule (single keys, no modifiers, etc)
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleModification {
    pub from: KeyCode,
    pub to: KeyCode,
}

/// A key on the keyboard
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyCode {
    pub key_code: String,
}

/// A complex rule (may involve several keys, mouse buttons, modifier keys...)
#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexModifications {
    /// Mysterious blob of parameters
    pub parameters: ::serde_json::Value,

    /// Rulesets
    pub rules: Vec<Rule>,
}

/// Ruleset
#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    /// Short name (copied from XML)
    pub description: String,

    /// Key replacements
    pub manipulators: Vec<Manipulator>,
}

/// Key replacement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manipulator {
    /// Always "basic"
    #[serde(rename="type")]
    pub type_: String,

    /// Origin key/button
    pub from: From,

    /// Destination key(s)/button(s)
    pub to: Vec<To>,

    /// Destination key(s)/button(s) for overlay keys
    #[serde(skip_serializing_if="Vec::is_empty", default)]
    pub to_if_alone: Vec<To>,
}

/// Origin key/button
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

/// Origin modifiers key(s)
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

/// Destination key/button
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

