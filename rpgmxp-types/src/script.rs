use flate2::bufread::ZlibDecoder;
use ruby_marshal::ArrayValue;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::StringValue;
use ruby_marshal::Value;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::io::Read;

/// Invalid script ruby data
#[derive(Debug)]
pub enum ScriptFromValueError {
    /// The array len was invalid.
    InvalidArrayLen { len: usize },

    /// The name was invalid
    InvalidName { error: std::str::Utf8Error },
}

impl std::fmt::Display for ScriptFromValueError {
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

impl std::error::Error for ScriptFromValueError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidName { error } => Some(error),
            _ => None,
        }
    }
}

impl From<ScriptFromValueError> for FromValueError {
    fn from(error: ScriptFromValueError) -> Self {
        FromValueError::Other {
            error: error.into(),
        }
    }
}

/// A compressed script
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CompressedScript {
    /// A randomly-generated script id.
    ///
    /// This should always be positive.
    pub id: i32,

    /// The name of the script.
    pub name: String,

    /// The zlib-compressed script data.
    pub data: Vec<u8>,
}

impl<'a> FromValue<'a> for CompressedScript {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let script: &ArrayValue = FromValue::from_value(ctx, value)?;
        let script = script.value();

        let array_len = script.len();
        if array_len != 3 {
            return Err(ScriptFromValueError::InvalidArrayLen { len: array_len }.into());
        }

        let id: i32 = ctx.from_value(script[0])?;

        let name: &StringValue = ctx.from_value(script[1])?;
        let name = std::str::from_utf8(name.value())
            .map_err(|error| ScriptFromValueError::InvalidName { error })?;

        let data: &StringValue = ctx.from_value(script[2])?;
        let data = data.value();

        Ok(Self {
            id,
            name: name.into(),
            data: data.into(),
        })
    }
}

impl IntoValue for CompressedScript {
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

/// A decompressed script
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Script {
    /// A randomly-generated script id.
    ///
    /// This should always be positive.
    pub id: i32,

    /// The name of the script.
    pub name: String,

    /// The script data.
    pub data: String,
}

impl<'a> FromValue<'a> for Script {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let script: CompressedScript = FromValue::from_value(ctx, value)?;

        let mut decoder = ZlibDecoder::new(&*script.data);
        let mut data = String::new();
        decoder
            .read_to_string(&mut data)
            .map_err(FromValueError::new_other)?;

        Ok(Self {
            id: script.id,
            name: script.name,
            data,
        })
    }
}
