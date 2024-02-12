use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::StringValue;
use ruby_marshal::Value;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use ruby_marshal::ValueKind;
use std::collections::HashSet;

/// An event command parameter
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EventCommandParameter {
    String(String),
    StringArray(Vec<String>),
    Int(i32),
}

impl<'a> FromValue<'a> for EventCommandParameter {
    fn from_value(
        arena: &'a ValueArena,
        handle: ValueHandle,
        visited: &mut HashSet<ValueHandle>,
    ) -> std::result::Result<Self, FromValueError> {
        let value: &Value = FromValue::from_value(arena, handle, visited)?;
        match value {
            Value::String(value) => {
                let value = value.value();
                let value = std::str::from_utf8(value)
                    .map_err(FromValueError::new_other)?
                    .to_string();
                Ok(Self::String(value))
            }
            Value::Array(value) => {
                let value = value.value();
                // TODO: Maybe make the internal array elements polymorphic.
                // If empty, static typing is impossible.
                let first = value.first().expect("array is empty");
                let first_kind = arena
                    .get(*first)
                    .ok_or(FromValueError::InvalidValueHandle { handle: *first })?
                    .kind();

                match first_kind {
                    ValueKind::String => {
                        let mut new_value = Vec::with_capacity(value.len());
                        for value in value.iter().copied() {
                            let value: &StringValue = FromValue::from_value(arena, value, visited)?;
                            let value = std::str::from_utf8(value.value())
                                .map_err(FromValueError::new_other)?
                                .to_string();
                            new_value.push(value);
                        }

                        Ok(Self::StringArray(new_value))
                    }
                    _ => {
                        todo!("unknown array first kind")
                    }
                }
            }
            Value::Fixnum(value) => {
                let value = value.value();

                Ok(Self::Int(value))
            }
            _ => {
                todo!("FromValue is stubbed for EventCommandParameter: {value:?}")
            }
        }
    }
}

impl IntoValue for EventCommandParameter {
    fn into_value(self, _: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        todo!("IntoValue is stubbed for EventCommandParameter")
    }
}
