use crate::Color;
use crate::MoveCommand;
use crate::MoveRoute;
use crate::Se;
use crate::Tone;
use ruby_marshal::DisplayByteString;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::StringValue;
use ruby_marshal::SymbolValue;
use ruby_marshal::Value;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use ruby_marshal::ValueKind;

#[derive(Debug)]
pub enum EventCommandParameterFromValueError {
    UnexpectedValueKind { kind: ValueKind },
    UnexpectedUserDefinedName { name: Vec<u8> },
    EmptyArray,
    UnexpectedArrayValueKind { kind: ValueKind },
}

impl std::fmt::Display for EventCommandParameterFromValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedValueKind { kind } => {
                write!(
                    f,
                    "unexpected value kind for event command parameter: {kind:?}"
                )
            }
            Self::UnexpectedUserDefinedName { name } => {
                let name = DisplayByteString(name);
                write!(
                    f,
                    "unexpected event command parameter user defined name: {name}"
                )
            }
            Self::EmptyArray => write!(f, "the event command parameter array is empty"),
            Self::UnexpectedArrayValueKind { kind } => {
                write!(
                    f,
                    "unexpected event command parameter array value kind: {kind:?}"
                )
            }
        }
    }
}

impl std::error::Error for EventCommandParameterFromValueError {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum EventCommandParameter {
    String(String),
    Int(i32),
    Bool(bool),
    Nil,
    StringArray(Vec<String>),
    MoveRoute(MoveRoute),
    MoveCommand(MoveCommand),
    Se(Se),
    Tone(Tone),
    Color(Color),
}

impl<'a> FromValue<'a> for EventCommandParameter {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        match value {
            Value::String(value) => {
                let value = value.value();
                let value = std::str::from_utf8(value)
                    .map_err(FromValueError::new_other)?
                    .to_string();
                Ok(Self::String(value))
            }
            Value::Fixnum(value) => {
                let value = value.value();
                Ok(Self::Int(value))
            }
            Value::Bool(value) => {
                let value = value.value();
                Ok(Self::Bool(value))
            }
            Value::Nil(_) => Ok(Self::Nil),
            Value::Array(value) => {
                let value = value.value();
                // TODO: Maybe make the internal array elements polymorphic.
                // If empty, static typing is impossible.
                let first = value.first().ok_or(FromValueError::new_other(
                    EventCommandParameterFromValueError::EmptyArray,
                ))?;
                let first: &Value = ctx.from_value(*first)?;
                let first_kind = first.kind();

                match first_kind {
                    ValueKind::String => {
                        let mut new_value = Vec::with_capacity(value.len());
                        for value in value.iter().copied() {
                            let value: &StringValue = ctx.from_value(value)?;
                            let value = std::str::from_utf8(value.value())
                                .map_err(FromValueError::new_other)?
                                .to_string();
                            new_value.push(value);
                        }

                        Ok(Self::StringArray(new_value))
                    }
                    _ => Err(FromValueError::new_other(
                        EventCommandParameterFromValueError::UnexpectedArrayValueKind {
                            kind: first_kind,
                        },
                    )),
                }
            }
            Value::Object(object_value) => {
                let name = object_value.name();
                let name: &SymbolValue = ctx.from_value(name.into())?;
                let name = name.value();

                match name {
                    crate::move_route::OBJECT_NAME => {
                        let value = FromValue::from_value(ctx, value)?;
                        Ok(Self::MoveRoute(value))
                    }
                    crate::move_command::OBJECT_NAME => {
                        let value = FromValue::from_value(ctx, value)?;
                        Ok(Self::MoveCommand(value))
                    }
                    crate::se::OBJECT_NAME => {
                        let value = FromValue::from_value(ctx, value)?;
                        Ok(Self::Se(value))
                    }
                    _ => Err(FromValueError::UnexpectedObjectName { name: name.into() }),
                }
            }
            Value::UserDefined(user_defined_value) => {
                let name = user_defined_value.name();
                let name: &SymbolValue = ctx.from_value(name.into())?;
                let name = name.value();

                match name {
                    crate::tone::USER_DEFINED_NAME => {
                        let value = FromValue::from_value(ctx, value)?;
                        Ok(Self::Tone(value))
                    }
                    crate::color::USER_DEFINED_NAME => {
                        let value = FromValue::from_value(ctx, value)?;
                        Ok(Self::Color(value))
                    }
                    _ => Err(FromValueError::new_other(
                        EventCommandParameterFromValueError::UnexpectedUserDefinedName {
                            name: name.into(),
                        },
                    )),
                }
            }
            _ => Err(FromValueError::new_other(
                EventCommandParameterFromValueError::UnexpectedValueKind { kind: value.kind() },
            )),
        }
    }
}

impl IntoValue for EventCommandParameter {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        match self {
            Self::String(value) => Ok(arena.create_string(value.into()).into()),
            Self::Int(value) => value.into_value(arena),
            Self::Bool(value) => value.into_value(arena),
            Self::Nil => Ok(arena.create_nil().into()),
            Self::StringArray(value) => {
                let values: Vec<_> = value
                    .into_iter()
                    .map(|value| arena.create_string(value.into()).into())
                    .collect();
                Ok(arena.create_array(values).into())
            }
            Self::MoveRoute(value) => value.into_value(arena),
            Self::MoveCommand(value) => value.into_value(arena),
            Self::Se(value) => value.into_value(arena),
            Self::Tone(value) => value.into_value(arena),
            Self::Color(value) => value.into_value(arena),
        }
    }
}
