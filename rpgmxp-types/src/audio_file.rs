use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::StringValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

const OBJECT_NAME: &str = "RPG::AudioFile";

const VOLUME_FIELD: &str = "@volume";
const NAME_FIELD: &str = "@name";
const PITCH_FIELD: &str = "@pitch";

/// An audio file
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AudioFile {
    /// The volume
    pub volume: i32,

    /// The audio name
    pub name: String,

    /// The audio pitch
    pub pitch: i32,
}

impl<'a> FromValue<'a> for AudioFile {
    fn from_value(
        arena: &'a ValueArena,
        handle: ValueHandle,
        visited: &mut HashSet<ValueHandle>,
    ) -> Result<Self, FromValueError> {
        let object: &ObjectValue = FromValue::from_value(arena, handle, visited)?;
        let name = object.name();
        let name = arena
            .get_symbol(name)
            .ok_or(FromValueError::InvalidValueHandle {
                handle: name.into(),
            })?
            .value();
        let name = std::str::from_utf8(name).map_err(FromValueError::new_other)?;

        let instance_variables = object.instance_variables();

        if name != OBJECT_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let mut volume_field = None;
        let mut name_field = None;
        let mut pitch_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key = arena
                .get_symbol(key)
                .ok_or(FromValueError::InvalidValueHandle { handle: key.into() })?
                .value();
            let key = std::str::from_utf8(key).map_err(FromValueError::new_other)?;

            match key {
                VOLUME_FIELD => {
                    if volume_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: VOLUME_FIELD.into(),
                        });
                    }
                    volume_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                NAME_FIELD => {
                    if name_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: NAME_FIELD.into(),
                        });
                    }
                    let name: &StringValue = FromValue::from_value(arena, value, visited)?;
                    let name =
                        std::str::from_utf8(name.value()).map_err(FromValueError::new_other)?;
                    name_field = Some(name);
                }
                PITCH_FIELD => {
                    if pitch_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: PITCH_FIELD.into(),
                        });
                    }
                    pitch_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let volume = volume_field.ok_or(FromValueError::MissingInstanceVariable {
            name: VOLUME_FIELD.into(),
        })?;
        let name = name_field.ok_or(FromValueError::MissingInstanceVariable {
            name: NAME_FIELD.into(),
        })?;
        let pitch = pitch_field.ok_or(FromValueError::MissingInstanceVariable {
            name: PITCH_FIELD.into(),
        })?;

        Ok(Self {
            volume,
            name: name.into(),
            pitch,
        })
    }
}
