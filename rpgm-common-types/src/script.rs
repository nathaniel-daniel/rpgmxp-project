use flate2::bufread::ZlibDecoder;
use flate2::bufread::ZlibEncoder;
use flate2::Compression;
use ruby_marshal::ArrayValue;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::StringValue;
use ruby_marshal::Value;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::io::Read;

const ARRAY_LEN: usize = 3;

/// A list of compressed scripts
#[derive(Debug)]
pub struct CompressedScriptList {
    /// Scripts
    pub scripts: Vec<CompressedScript>,
}

impl<'a> FromValue<'a> for CompressedScriptList {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        Ok(Self {
            scripts: FromValue::from_value(ctx, value)?,
        })
    }
}
impl IntoValue for CompressedScriptList {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        self.scripts.into_value(arena)
    }
}

/// A list of scripts
#[derive(Debug)]
pub struct ScriptList {
    /// Scripts
    pub scripts: Vec<Script>,
}

impl<'a> FromValue<'a> for ScriptList {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        Ok(Self {
            scripts: FromValue::from_value(ctx, value)?,
        })
    }
}
impl IntoValue for ScriptList {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        self.scripts.into_value(arena)
    }
}

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
                write!(f, "invalid script array len of {len}, expected {ARRAY_LEN}")
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
        if array_len != ARRAY_LEN {
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
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
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

impl IntoValue for Script {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let id = self.id.into_value(arena)?;
        let name = arena.create_string(self.name.into()).into_raw();

        let compression = Compression::default();
        let mut encoder = ZlibEncoder::new(self.data.as_bytes(), compression);
        let mut data = Vec::new();
        encoder
            .read_to_end(&mut data)
            .map_err(IntoValueError::new_other)?;

        let data = arena.create_string(data).into_raw();

        let array = arena.create_array(vec![id, name, data]);

        Ok(array.into())
    }
}
