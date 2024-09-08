use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::StringValue;
use ruby_marshal::Value;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;

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

/*
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
*/

pub(crate) fn optional_ruby_string_array2optional_string_array(
    ctx: &FromValueContext,
    value: &Value,
) -> Result<Vec<Option<String>>, FromValueError> {
    struct Wrapper(Option<String>);

    impl<'a> FromValue<'a> for Wrapper {
        fn from_value(
            ctx: &FromValueContext<'a>,
            value: &'a Value,
        ) -> Result<Self, FromValueError> {
            if matches!(value, Value::Nil(_)) {
                return Ok(Self(None));
            }

            let value = ruby_string2string(ctx, value)?;
            Ok(Self(Some(value)))
        }
    }

    let value: Vec<Wrapper> = FromValue::from_value(ctx, value)?;
    let value = value.into_iter().map(|value| value.0).collect();

    Ok(value)
}

pub(crate) fn optional_string_array2optional_ruby_string_array(
    optional_string_array: Vec<Option<String>>,
    arena: &mut ValueArena,
) -> Result<ValueHandle, ruby_marshal::IntoValueError> {
    let mut value = Vec::with_capacity(optional_string_array.len());
    for optional_string in optional_string_array {
        let handle = match optional_string {
            Some(string) => string2ruby_string(string, arena)?,
            None => arena.create_nil().into(),
        };
        value.push(handle);
    }
    Ok(arena.create_array(value).into())
}
