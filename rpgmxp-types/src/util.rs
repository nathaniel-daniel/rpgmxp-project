pub(crate) use rpgm_common_types::ruby_string2string;
pub(crate) use rpgm_common_types::string2ruby_string;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::Value;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;

pub(crate) fn ruby_string_array2string_array(
    ctx: &FromValueContext,
    value: &Value,
) -> Result<Vec<String>, FromValueError> {
    struct Wrapper(String);

    impl<'a> FromValue<'a> for Wrapper {
        fn from_value(
            ctx: &FromValueContext<'a>,
            value: &'a Value,
        ) -> Result<Self, FromValueError> {
            let value = ruby_string2string(ctx, value)?;
            Ok(Self(value))
        }
    }

    let value: Vec<Wrapper> = FromValue::from_value(ctx, value)?;
    let value = value.into_iter().map(|value| value.0).collect();

    Ok(value)
}

pub(crate) fn string_array2ruby_string_array(
    string_array: Vec<String>,
    arena: &mut ValueArena,
) -> Result<ValueHandle, ruby_marshal::IntoValueError> {
    let mut value = Vec::with_capacity(string_array.len());
    for string in string_array {
        value.push(string2ruby_string(string, arena)?);
    }
    Ok(arena.create_array(value).into())
}
