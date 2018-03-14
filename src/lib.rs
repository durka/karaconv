#[macro_use] extern crate failure;
#[macro_use] extern crate serde;
extern crate serde_json;

pub mod xml;
pub mod json;

use failure::Error;

pub fn conv_key(s: &str) -> Result<json::KeyOrButton, Error> {
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

pub fn conv_mod(s: &str) -> Result<Vec<String>, Error> {
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
            "NONE"      => continue,
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

