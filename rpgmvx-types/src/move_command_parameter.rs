use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::Value;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use ruby_marshal::ValueKind;

#[derive(Debug)]
pub enum MoveCommandParameterFromValueError {
    UnexpectedValueKind { kind: ValueKind },
}

impl std::fmt::Display for MoveCommandParameterFromValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedValueKind { kind } => {
                write!(
                    f,
                    "unexpected value kind for move command parameter: {kind:?}"
                )
            }
        }
    }
}

impl std::error::Error for MoveCommandParameterFromValueError {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum MoveCommandParameter {
    Int(i32),
}

impl<'a> FromValue<'a> for MoveCommandParameter {
    fn from_value(_ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        match value {
            Value::Fixnum(value) => {
                let value = value.value();
                Ok(Self::Int(value))
            }
            _ => Err(FromValueError::new_other(
                MoveCommandParameterFromValueError::UnexpectedValueKind { kind: value.kind() },
            )),
        }
    }
}

impl IntoValue for MoveCommandParameter {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        match self {
            Self::Int(value) => value.into_value(arena),
        }
    }
}
