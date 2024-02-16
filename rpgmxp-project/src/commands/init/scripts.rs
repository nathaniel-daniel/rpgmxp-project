use flate2::bufread::ZlibDecoder;
use ruby_marshal::ArrayValue;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::Value;
use std::io::Read;

#[derive(Debug)]
pub struct ScriptList {
    pub scripts: Vec<Script>,
}

impl<'a> FromValue<'a> for ScriptList {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let array: &ArrayValue = FromValue::from_value(ctx, value)?;
        let array = array.value();

        let mut scripts = Vec::with_capacity(array.len());
        for handle in array {
            let script: Script = ctx.from_value(*handle)?;
            scripts.push(script);
        }

        Ok(Self { scripts })
    }
}

#[derive(Debug)]
pub struct Script {
    #[allow(dead_code)]
    pub id: i32,
    pub name: String,
    pub data: String,
}

impl<'a> ruby_marshal::FromValue<'a> for Script {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let script: rpgmxp_types::Script = FromValue::from_value(ctx, value)?;

        let mut decoder = ZlibDecoder::new(&*script.data);
        let mut data = String::new();
        decoder
            .read_to_string(&mut data)
            .map_err(|error| FromValueError::Other {
                error: error.into(),
            })?;

        Ok(Self {
            id: script.id,
            name: script.name,
            data,
        })
    }
}
