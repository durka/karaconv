#[macro_use] extern crate structopt;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde;
extern crate chrono;
extern crate regex;
extern crate serde_json;
extern crate serde_xml_rs as serde_xml;

use failure::Error;
use regex::Regex;
use structopt::StructOpt;

use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Input file (Karabiner XML)
    #[structopt(short="i", long="input", parse(from_os_str))]
    infile: PathBuf,

    /// Output file (Karabiner-Elements JSON)
    #[structopt(short="o", long="output", parse(from_os_str))]
    outfile: PathBuf,

    /// Dry run
    #[structopt(short="n")]
    dry_run: bool,
}

mod xml {
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
}

mod json {
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

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Manipulator {
        #[serde(rename="type")]
        pub type_: String,
        pub from: From,
        pub to: Vec<To>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum From {
        Key {
            key_code: String,
            modifiers: FromModifiers,
        },
        Button {
            pointing_button: String,
            modifiers: FromModifiers,
        },
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FromModifiers {
        #[serde(skip_serializing_if="Option::is_none")]
        pub mandatory: Option<Vec<String>>,
        #[serde(skip_serializing_if="Option::is_none")]
        pub optional: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum To {
        Key {
            key_code: String,
            modifiers: Vec<String>,
        },
        Button {
            pointing_button: String,
            modifiers: Vec<String>,
        },
    }

    pub enum KeyOrButton {
        Key(String),
        Button(String),
    }

    pub trait KeyOrButtonConv {
        fn conv(key_or_button: KeyOrButton, mods: Option<Vec<String>>) -> Self;
    }

    impl KeyOrButtonConv for From {
        fn conv(key_or_button: KeyOrButton, mods: Option<Vec<String>>) -> Self {
            match key_or_button {
                KeyOrButton::Key(s) =>
                    From::Key {
                        key_code: s,
                        modifiers: FromModifiers {
                            mandatory: mods,
                            optional: None,
                        }
                    },

                KeyOrButton::Button(s) =>
                    From::Button {
                        pointing_button: s,
                        modifiers: FromModifiers {
                            mandatory: mods,
                            optional: None,
                        }
                    },
            }
        }
    }

    impl KeyOrButtonConv for To {
        fn conv(key_or_button: KeyOrButton, mods: Option<Vec<String>>) -> Self {
            match key_or_button {
                KeyOrButton::Key(s) =>
                    To::Key {
                        key_code: s,
                        modifiers: mods.unwrap_or(vec![]),
                    },

                KeyOrButton::Button(s) =>
                    To::Button {
                        pointing_button: s,
                        modifiers: mods.unwrap_or(vec![]),
                    },
            }
        }
    }
}
use json::KeyOrButtonConv;

fn conv_key(s: &str) -> Result<json::KeyOrButton, Error> {
    use json::KeyOrButton::*;

    let parts = s.split("::").map(str::trim).collect::<Vec<_>>();

    Ok(match parts[0] {
        "KeyCode" => {
            Key(match parts[1] {
                "A" => "a",
                "B" => "b",
                "C" => "c",
                "D" => "d",
                "E" => "e",
                "F" => "f",
                "G" => "g",
                "H" => "h",
                "I" => "i",
                "J" => "j",
                "K" => "k",
                "L" => "l",
                "M" => "m",
                "N" => "n",
                "O" => "o",
                "P" => "p",
                "Q" => "q",
                "R" => "r",
                "S" => "s",
                "T" => "t",
                "U" => "u",
                "V" => "v",
                "W" => "w",
                "X" => "x",
                "Y" => "y",
                "Z" => "z",

                "0" => "0",
                "1" => "1",
                "2" => "2",
                "3" => "3",
                "4" => "4",
                "5" => "5",
                "6" => "6",
                "7" => "7",
                "8" => "8",
                "9" => "9",

                "KEY_0" => "0",
                "KEY_1" => "1",
                "KEY_2" => "2",
                "KEY_3" => "3",
                "KEY_4" => "4",
                "KEY_5" => "5",
                "KEY_6" => "6",
                "KEY_7" => "7",
                "KEY_8" => "8",
                "KEY_9" => "9",

                "KEYPAD_0"        => "keypad_0",
                "KEYPAD_1"        => "keypad_1",
                "KEYPAD_2"        => "keypad_2",
                "KEYPAD_3"        => "keypad_3",
                "KEYPAD_4"        => "keypad_4",
                "KEYPAD_5"        => "keypad_5",
                "KEYPAD_6"        => "keypad_6",
                "KEYPAD_7"        => "keypad_7",
                "KEYPAD_8"        => "keypad_8",
                "KEYPAD_9"        => "keypad_9",
                "KEYPAD_DOT"      => "keypad_period",
                "KEYPAD_EQUAL"    => "keypad_equal_sign",
                "KEYPAD_PLUS"     => "keypad_plus",
                "KEYPAD_SLASH"    => "keypad_slash",
                "KEYPAD_MULTIPLY" => "keypad_asterisk",
                "KEYPAD_MINUS"    => "keypad_hyphen",

                "BACKQUOTE"     => "grave_accent_and_tilde",
                "BACKSLASH"     => "backslash",
                "BRACKET_LEFT"  => "open_bracket",
                "BRACKET_RIGHT" => "close_bracket",
                "COMMA"         => "comma",
                "DOT"           => "period",
                "EQUAL"         => "equal_sign",
                "MINUS"         => "hyphen",
                "QUOTE"         => "quote",
                "SEMICOLON"     => "semicolon",
                "SLASH"          => "slash",

                "DELETE"          => "delete_or_backspace",
                "ENTER"           => "return_or_enter",
                "ESCAPE"          => "escape",
                "RETURN"          => "return_or_enter",
                "SPACE"           => "spacebar",
                "TAB"             => "tab",

                "F1"  => "f1",
                "F2"  => "f2",
                "F3"  => "f3",
                "F4"  => "f4",
                "F5"  => "f5",
                "F6"  => "f6",
                "F7"  => "f7",
                "F8"  => "f8",
                "F9"  => "f9",
                "F10" => "f10",
                "F11" => "f11",
                "F12" => "f12",
                "F13" => "f13",
                "F14" => "f14",
                "F15" => "f15",
                "F16" => "f16",
                "F17" => "f17",
                "F18" => "f18",
                "F19" => "f19",

                "BRIGHTNESS_DOWN" => "display_brightness_decrement",
                "BRIGHTNESS_UP"   => "display_brightness_increment",
                "DASHBOARD"       => "dashboard",
                "LAUNCHPAD"       => "launchpad",
                "MISSION_CONTROL" => "mission_control",

                "PAGEUP"   => "page_up",
                "PAGEDOWN" => "page_down",
                "HOME"     => "home",
                "END"      => "end",

                "CAPSLOCK"  => "caps_lock",
                "COMMAND_L" => "left_command",
                "COMMAND_R" => "right_command",
                "CONTROL_L" => "left_control",
                "CONTROL_R" => "right_control",
                "FN"        => "fn",
                "OPTION_L"  => "left_option",
                "OPTION_R"  => "right_option",
                "SHIFT_L"   => "left_shift",
                "SHIFT_R"   => "right_shift",

                "CURSOR_LEFT"  => "left_arrow",
                "CURSOR_RIGHT" => "right_arrow",
                "CURSOR_UP"    => "up_arrow",
                "CURSOR_DOWN"  => "down_arrow",

                otherwise => bail!("Unknown key code {}", otherwise)
            }.into())
        }

        "PointingButton" => {
            Button(match parts[1] {
                "LEFT"    => "button1",
                "RIGHT"   => "button2",
                "MIDDLE"  => "button3",

                "BUTTON1" => "button1",
                "BUTTON2" => "button2",
                "BUTTON3" => "button3",
                "BUTTON4" => "button4",
                "BUTTON5" => "button5",
                "BUTTON6" => "button6",
                "BUTTON7" => "button7",
                "BUTTON8" => "button8",
                "BUTTON9" => "button9",

                otherwise => bail!("Unknown mouse button {}", otherwise)
            }.into())
        }

        otherwise => bail!("Not a key code: {}", otherwise)
    })
}

fn conv_mod(s: &str) -> Result<Vec<String>, Error> {
    let mut mods = s.split('|').map(str::trim).map(|s| s.split("::").map(str::trim).collect::<Vec<_>>()).collect::<Vec<_>>();

    if mods.is_empty() {
        bail!("Empty modifier");
    } else {
        for m in &mut mods {
            if m[0] != "ModifierFlag" {
                if m[0].starts_with("VK_") {
                    *m = vec!["ModifierFlag", &m[0][3..]];
                } else {
                    bail!("Not a modifier: {}", &m[0]);
                }
            }
        }
    }

    let mut convs = vec![];
    for m in mods {
        convs.push(match m[1] {
            "ZERO"      => "zero",
            "CAPSLOCK"  => "caps_lock",
            "SHIFT"     => "left_shift",
            "SHIFT_L"   => "left_shift",
            "SHIFT_R"   => "right_shift",
            "CONTROL"   => "left_control",
            "CONTROL_L" => "left_control",
            "CONTROL_R" => "right_control",
            "OPTION_L"  => "left_option",
            "OPTION_R"  => "right_option",
            "COMMAND"   => "left_command",
            "COMMAND_L" => "left_command",
            "COMMAND_R" => "right_command",
            "FN"        => "fn",

            otherwise => bail!("Unknown modifier {}", otherwise)
        }.into());
    }
    Ok(convs)
}

fn try_main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let inxml: xml::Karabiner = serde_xml::deserialize(File::open(&opt.infile)?)?;
    let mut outjson: json::Karabiner = serde_json::from_reader(File::open(&opt.outfile)?)?;

    for item in inxml.items {
        println!("Converting {}...", item.name);
        let mut rule = json::Rule {
            description: item.name,
            manipulators: vec![]
        };
        
        for key in item.keys {
            let type_regex = Regex::new(r"--(?P<type>[a-zA-Z]+)-- (?P<contents>.*)")?;
            if let Some(caps) = type_regex.captures(&key) {
                let mut parts = caps["contents"].split(',').map(str::trim).peekable();
                match &caps["type"] {
                    "KeyToKey" => {
                        let fromkey = conv_key(parts.next().unwrap())?;
                        let frommod = if parts.peek().unwrap().starts_with("Modifier") { Some(conv_mod(parts.next().unwrap())?) } else { None };

                        let mut manip = json::Manipulator {
                            type_: "basic".into(),
                            from: json::From::conv(fromkey, frommod),
                            to: vec![]
                        };

                        while parts.peek().is_some() {
                            let tokey = conv_key(parts.next().unwrap())?;
                            let tomod = if parts.peek().is_some() {
                                if !parts.peek().unwrap().starts_with("Key") { Some(conv_mod(parts.next().unwrap())?) } else { None }
                            } else {
                                None
                            };
                            manip.to.push(json::To::conv(tokey, tomod));
                        }

                        rule.manipulators.push(manip);
                    }

                    "KeyOverlaidModifier" => {
                    }

                    otherwise => bail!("Unsupported autogen type: {}", otherwise)
                }
            } else {
                bail!("Unparseable autogen: {}", key);
            }
        }

        outjson.profiles[0].complex_modifications.rules.push(rule);
    }

    if opt.dry_run {
        println!("{}", serde_json::to_string_pretty(&outjson)?);
    } else {
        let ext = format!("{}.bak.{}", opt.outfile.extension().unwrap().to_str().unwrap(), chrono::Local::now().format("%Y%m%d"));
        fs::copy(&opt.outfile, opt.outfile.with_extension(ext))?;
        serde_json::to_writer_pretty(File::create(&opt.outfile)?, &outjson)?;
    }

    Ok(())
}

fn main() {
    try_main().unwrap();
}

