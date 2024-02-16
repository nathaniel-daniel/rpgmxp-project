use crate::MoveCommand;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

pub(crate) const OBJECT_NAME: &[u8] = b"RPG::MoveRoute";

const LIST_FIELD: &[u8] = b"@list";
const SKIPPABLE_FIELD: &[u8] = b"@skippable";
const REPEAT_FIELD: &[u8] = b"@repeat";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MoveRoute {
    pub list: Vec<MoveCommand>,
    pub skippable: bool,
    pub repeat: bool,
}

impl<'a> FromValue<'a> for MoveRoute {
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
        if name != OBJECT_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let instance_variables = object.instance_variables();

        let mut list_field = None;
        let mut skippable_field = None;
        let mut repeat_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key = arena
                .get_symbol(key)
                .ok_or(FromValueError::InvalidValueHandle { handle: key.into() })?
                .value();

            match key {
                LIST_FIELD => {
                    if list_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    list_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                SKIPPABLE_FIELD => {
                    if skippable_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    skippable_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                REPEAT_FIELD => {
                    if repeat_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    repeat_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let list = list_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: LIST_FIELD.into(),
        })?;
        let skippable = skippable_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: SKIPPABLE_FIELD.into(),
        })?;
        let repeat = repeat_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: REPEAT_FIELD.into(),
        })?;

        Ok(Self {
            list,
            skippable,
            repeat,
        })
    }
}

impl IntoValue for MoveRoute {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let list_field_key = arena.create_symbol(LIST_FIELD.into());
        let skippable_field_key = arena.create_symbol(SKIPPABLE_FIELD.into());
        let repeat_field_key = arena.create_symbol(REPEAT_FIELD.into());

        let list_field_value = self.list.into_value(arena)?;
        let skippable_field_value = arena.create_bool(self.skippable);
        let repeat_field_value = arena.create_bool(self.repeat);

        let fields = vec![
            (list_field_key, list_field_value),
            (skippable_field_key, skippable_field_value.into()),
            (repeat_field_key, repeat_field_value.into()),
        ];

        let object = arena.create_object(object_name, fields);

        Ok(object.into())
    }
}
