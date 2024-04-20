use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::StringValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;

use ruby_marshal::FromValueContext;
use ruby_marshal::Value;

pub(crate) fn ruby_string2string(
    ctx: &FromValueContext,
    value: &Value,
) -> Result<String, FromValueError> {
    let value: &StringValue = FromValue::from_value(ctx, value)?;
    let value = value.value();
    let value = std::str::from_utf8(value)
        .map_err(FromValueError::new_other)?
        .to_string();

    Ok(value)
}

pub(crate) fn string2ruby_string(
    s: String,
    arena: &mut ValueArena,
) -> Result<ValueHandle, ruby_marshal::IntoValueError> {
    Ok(arena.create_string(s.into()).into())
}
