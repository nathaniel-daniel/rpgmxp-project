use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::StringValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

pub(crate) const OBJECT_NAME: &[u8] = b"RPG::AudioFile";

const VOLUME_FIELD: &[u8] = b"@volume";
const NAME_FIELD: &[u8] = b"@name";
const PITCH_FIELD: &[u8] = b"@pitch";

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

        let volume = volume_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: VOLUME_FIELD.into(),
        })?;
        let name = name_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: NAME_FIELD.into(),
        })?;
        let pitch = pitch_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: PITCH_FIELD.into(),
        })?;

        Ok(Self {
            volume,
            name: name.into(),
            pitch,
        })
    }
}

impl IntoValue for AudioFile {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let volume_field_key = arena.create_symbol(VOLUME_FIELD.into());
        let name_field_key = arena.create_symbol(NAME_FIELD.into());
        let pitch_field_key = arena.create_symbol(PITCH_FIELD.into());

        let volume_field_value = self.volume.into_value(arena)?;
        let name_field_value = arena.create_string(self.name.into());
        let pitch_field_value = self.pitch.into_value(arena)?;

        let instance_variables = vec![
            (volume_field_key, volume_field_value),
            (name_field_key, name_field_value.into()),
            (pitch_field_key, pitch_field_value),
        ];
        let object = arena.create_object(object_name, instance_variables);

        Ok(object.into())
    }
}
