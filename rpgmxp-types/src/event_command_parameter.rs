use crate::AudioFile;
use crate::MoveCommand;
use crate::MoveRoute;
use crate::Tone;
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
use ruby_marshal::DisplayByteString;

#[derive(Debug)]
pub enum EventCommandParameterFromValueError {
    UnexpectedValueKind { kind: ValueKind },
    EmptyArray,
    UnexpectedArrayValueKind { kind: ValueKind },
    UnexpectedUserDefinedName { name: Vec<u8> },
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
            Self::EmptyArray => write!(f, "the event command parameter array is empty"),
            Self::UnexpectedArrayValueKind { kind } => {
                write!(
                    f,
                    "unexpected event command parameter array value kind: {kind:?}"
                )
            }
            Self::UnexpectedUserDefinedName { name } => {
                let name = DisplayByteString(name);
                write!(
                    f,
                    "unexpected event command parameter user defined name: {name}"
                )
            }
        }
    }
}

impl std::error::Error for EventCommandParameterFromValueError {}

/// An event command parameter
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EventCommandParameter {
    String(String),
    StringArray(Vec<String>),
    Int(i32),
    MoveRoute(MoveRoute),
    MoveCommand(MoveCommand),
    AudioFile(AudioFile),
    Tone(Tone),
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
            Value::Fixnum(value) => {
                let value = value.value();
                Ok(Self::Int(value))
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
                    crate::audio_file::OBJECT_NAME => {
                        let value = FromValue::from_value(ctx, value)?;
                        Ok(Self::AudioFile(value))
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
    fn into_value(self, _: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        todo!("IntoValue is stubbed for EventCommandParameter")
    }
}
