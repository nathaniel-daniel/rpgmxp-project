use anyhow::bail;
use anyhow::Context;
use rpgmxp_types::Actor;
use rpgmxp_types::Armor;
use rpgmxp_types::Class;
use rpgmxp_types::CommonEvent;
use rpgmxp_types::Enemy;
use rpgmxp_types::Item;
use rpgmxp_types::Skill;
use rpgmxp_types::State;
use rpgmxp_types::Troop;
use rpgmxp_types::Weapon;
use std::fmt::Write;

/// Convert a hex u8 char into a u8 value.
///
/// # Returns
/// Returns `None` if the char is not a hex char.
pub fn decode_hex_u8(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

/// Check if a file name is a map file name.
///
/// # Arguments
/// `file_name`: The file name to check.
/// `expected_extension`: The expected extension.
pub fn is_map_file_name(file_name: &str, expected_extension: &str) -> bool {
    file_name
        .rsplit_once('.')
        .and_then(|(file_name, extension)| {
            if extension == expected_extension {
                Some(file_name)
            } else {
                None
            }
        })
        .and_then(|file_name| file_name.strip_prefix("Map"))
        .map_or(false, |map_n| {
            !map_n.is_empty() && map_n.chars().all(|c| c.is_ascii_digit())
        })
}

/// Percent-escape a file name.
///
/// This will percent-escape the following:
/// * '%'
/// * ':'
/// * '*'
pub fn percent_escape_file_name(file_name: &str) -> String {
    let mut escaped = String::with_capacity(file_name.len());
    for c in file_name.chars() {
        match c {
            '%' | ':' | '*' => {
                let c = u32::from(c);
                write!(&mut escaped, "%{c:02x}").unwrap();
            }
            _ => {
                escaped.push(c);
            }
        }
    }
    escaped
}

/// Percent-unescape a file name.
///
/// # Returns
/// Returns an error if the string cannot be unescaped.
pub fn percent_unescape_file_name(file_name: &str) -> anyhow::Result<String> {
    #[derive(PartialEq)]
    enum State {
        Normal,
        ParsePercentEscape { index: usize, value: u8 },
    }

    let mut unescaped = String::with_capacity(file_name.len());
    let mut state = State::Normal;
    for c in file_name.chars() {
        match (&mut state, c) {
            (State::Normal, '%') => {
                state = State::ParsePercentEscape { index: 0, value: 0 };
            }
            (State::Normal, c) => unescaped.push(c),
            (State::ParsePercentEscape { index, value }, c) => {
                let c = u8::try_from(c).context("invalid percent escape")?;
                let c = crate::util::decode_hex_u8(c).context("invalid hex char")?;

                *value |= c << (4 - (4 * *index));
                *index += 1;

                if *index == 2 {
                    let c = char::from(*value);
                    unescaped.push(c);

                    state = State::Normal;
                }
            }
        }
    }

    if state != State::Normal {
        bail!("incomplete percent escape");
    }

    Ok(unescaped)
}

/// A trait to represent objects stored in *.rxdata files as elements of an array.
pub trait ArrayLikeElement<'a>: serde::Serialize + ruby_marshal::FromValue<'a> {
    /// Get the display name of this type
    fn type_display_name() -> &'static str;

    /// Get the name of this element.
    fn name(&self) -> &str;
}

impl ArrayLikeElement<'_> for CommonEvent {
    fn type_display_name() -> &'static str {
        "common event"
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ArrayLikeElement<'_> for Actor {
    fn type_display_name() -> &'static str {
        "actor"
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ArrayLikeElement<'_> for Weapon {
    fn type_display_name() -> &'static str {
        "weapon"
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ArrayLikeElement<'_> for Armor {
    fn type_display_name() -> &'static str {
        "armor"
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ArrayLikeElement<'_> for Skill {
    fn type_display_name() -> &'static str {
        "skill"
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ArrayLikeElement<'_> for State {
    fn type_display_name() -> &'static str {
        "state"
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ArrayLikeElement<'_> for Item {
    fn type_display_name() -> &'static str {
        "item"
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ArrayLikeElement<'_> for Enemy {
    fn type_display_name() -> &'static str {
        "enemy"
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ArrayLikeElement<'_> for Class {
    fn type_display_name() -> &'static str {
        "class"
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl ArrayLikeElement<'_> for Troop {
    fn type_display_name() -> &'static str {
        "troop"
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_hex_u8_sanity() {
        assert!(decode_hex_u8(b'F') == Some(15));
        assert!(decode_hex_u8(b'G').is_none());
    }

    #[test]
    fn is_map_file_name_sanity() {
        assert!(is_map_file_name("Map001.rxdata", "rxdata"));
        assert!(!is_map_file_name("Map001.json", "rxdata"));
        assert!(is_map_file_name("Map001.json", "json"));
        assert!(!is_map_file_name("Map001.rxdata", "json"));

        assert!(!is_map_file_name("Map.json", "json"));
        assert!(!is_map_file_name("Map", "json"));
    }

    #[test]
    fn percent_escape_round_trip() {
        let tests = ["hello.txt", "%world.json", "foo:bar.rxdata"];

        for test in tests {
            let escaped = percent_escape_file_name(test);
            let unescaped =
                percent_unescape_file_name(escaped.as_str()).expect("failed to percent unescape");

            assert!(test == unescaped, "{test} != {unescaped}");
        }
    }
}
