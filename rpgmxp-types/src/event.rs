use crate::EventPage;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::StringValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

const OBJECT_NAME: &[u8] = b"RPG::Event";

const PAGES_FIELD: &[u8] = b"@pages";
const NAME_FIELD: &[u8] = b"@name";
const Y_FIELD: &[u8] = b"@y";
const X_FIELD: &[u8] = b"@x";
const ID_FIELD: &[u8] = b"@id";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Event {
    pub pages: Vec<EventPage>,
    pub name: String,
    pub y: i32,
    pub x: i32,
    pub id: i32,
}

impl<'a> FromValue<'a> for Event {
    fn from_value(
        arena: &ValueArena,
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

        let mut pages_field = None;
        let mut name_field = None;
        let mut y_field = None;
        let mut x_field = None;
        let mut id_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key = arena
                .get_symbol(key)
                .ok_or(FromValueError::InvalidValueHandle { handle: key.into() })?
                .value();
            match key {
                PAGES_FIELD => {
                    if pages_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: PAGES_FIELD.into(),
                        });
                    }

                    pages_field = Some(FromValue::from_value(arena, value, visited)?);
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
                Y_FIELD => {
                    if y_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: Y_FIELD.into(),
                        });
                    }

                    y_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                X_FIELD => {
                    if x_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: X_FIELD.into(),
                        });
                    }

                    x_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                ID_FIELD => {
                    if id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: ID_FIELD.into(),
                        });
                    }

                    id_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let pages = pages_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: PAGES_FIELD.into(),
        })?;
        let name = name_field
            .ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: NAME_FIELD.into(),
            })?
            .to_string();
        let y = y_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: Y_FIELD.into(),
        })?;
        let x = x_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: X_FIELD.into(),
        })?;
        let id = id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ID_FIELD.into(),
        })?;

        Ok(Self {
            pages,
            name,
            y,
            x,
            id,
        })
    }
}

impl IntoValue for Event {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let pages_field_key = arena.create_symbol(PAGES_FIELD.into());
        let name_field_key = arena.create_symbol(NAME_FIELD.into());
        let y_field_key = arena.create_symbol(Y_FIELD.into());
        let x_field_key = arena.create_symbol(X_FIELD.into());
        let id_field_key = arena.create_symbol(ID_FIELD.into());

        let pages_field_value = self.y.into_value(arena)?;
        let name_field_value = arena.create_string(self.name.into()).into();
        let y_field_value = self.y.into_value(arena)?;
        let x_field_value = self.x.into_value(arena)?;
        let id_field_value = self.id.into_value(arena)?;

        let fields = vec![
            (pages_field_key, pages_field_value),
            (name_field_key, name_field_value),
            (y_field_key, y_field_value),
            (x_field_key, x_field_value),
            (id_field_key, id_field_value),
        ];

        let object = arena.create_object(object_name, fields);

        Ok(object.into())
    }
}
