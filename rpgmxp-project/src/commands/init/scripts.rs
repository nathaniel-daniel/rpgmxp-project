use ruby_marshal::ArrayValue;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::Value;

#[derive(Debug)]
pub struct ScriptList {
    pub scripts: Vec<rpgmxp_types::Script>,
}

impl<'a> FromValue<'a> for ScriptList {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let array: &ArrayValue = FromValue::from_value(ctx, value)?;
        let array = array.value();

        let mut scripts = Vec::with_capacity(array.len());
        for handle in array {
            let script: rpgmxp_types::Script = ctx.from_value(*handle)?;
            scripts.push(script);
        }

        Ok(Self { scripts })
    }
}
