use ruby_marshal::FromValue;
use ruby_marshal::IntoValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

/// Invalid script ruby data
#[derive(Debug)]
pub enum InvalidScriptError {
    /// The array len was invalid.
    InvalidArrayLen { len: usize },

    /// The name was invalid
    InvalidName { error: std::str::Utf8Error },
}

impl std::fmt::Display for InvalidScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArrayLen { len } => {
                write!(f, "invalid script array len of {len}, expected 3")
            }
            Self::InvalidName { .. } => {
                write!(f, "the script name is invalid")
            }
        }
    }
}

impl std::error::Error for InvalidScriptError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidName { error } => Some(error),
            _ => None,
        }
    }
}

impl From<InvalidScriptError> for ruby_marshal::FromValueError {
    fn from(error: InvalidScriptError) -> Self {
        ruby_marshal::FromValueError::Other {
            error: error.into(),
        }
    }
}

/// A Script
#[derive(Debug, Clone)]
pub struct Script {
    /// A randomly-generated script id.
    ///
    /// This should always be positive.
    pub id: i32,

    /// The name of the script.
    pub name: String,

    /// The zlib-compressed script data.
    pub data: Vec<u8>,
}

impl<'a> FromValue<'a> for Script {
    fn from_value(
        arena: &'a ValueArena,
        handle: ValueHandle,
        visited: &mut HashSet<ValueHandle>,
    ) -> Result<Self, ruby_marshal::FromValueError> {
        let script: &ruby_marshal::ArrayValue = FromValue::from_value(arena, handle, visited)?;
        let script = script.value();

        let array_len = script.len();
        if array_len != 3 {
            return Err(InvalidScriptError::InvalidArrayLen { len: array_len }.into());
        }

        let id: i32 = FromValue::from_value(arena, script[0], visited)?;

        let name: &ruby_marshal::StringValue = FromValue::from_value(arena, script[1], visited)?;
        let name = std::str::from_utf8(name.value())
            .map_err(|error| InvalidScriptError::InvalidName { error })?;

        let data: &ruby_marshal::StringValue = FromValue::from_value(arena, script[2], visited)?;
        let data = data.value();

        Ok(Self {
            id,
            name: name.into(),
            data: data.into(),
        })
    }
}

impl IntoValue for Script {
    fn into_value(
        self,
        arena: &mut ValueArena,
    ) -> Result<ValueHandle, ruby_marshal::IntoValueError> {
        let id = self.id.into_value(arena)?;
        let name = arena.create_string(self.name.into()).into_raw();
        let data = arena.create_string(self.data).into_raw();

        let array = arena.create_array(vec![id, name, data]);

        Ok(array.into())
    }
}
