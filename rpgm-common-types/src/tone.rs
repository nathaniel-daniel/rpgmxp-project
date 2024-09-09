use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::SymbolValue;
use ruby_marshal::UserDefinedValue;
use ruby_marshal::Value;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;

pub const USER_DEFINED_NAME: &[u8] = b"Tone";

const SIZE: usize = 8 * 4;

#[derive(Debug)]
pub enum ToneFromValueError {
    InvalidSize { size: usize },
}

impl std::fmt::Display for ToneFromValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidSize { size } => {
                write!(f, "invalid Tone data size of {size}, expected {SIZE}")
            }
        }
    }
}

impl std::error::Error for ToneFromValueError {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Tone {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub gray: f64,
}

impl<'a> FromValue<'a> for Tone {
    fn from_value(ctx: &FromValueContext, value: &Value) -> Result<Self, FromValueError> {
        let user_defined: &UserDefinedValue = FromValue::from_value(ctx, value)?;
        let name = user_defined.name();
        let name: &SymbolValue = ctx.from_value(name.into())?;
        let name = name.value();
        if name != USER_DEFINED_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let value = user_defined.value();

        let value_len = value.len();
        if value_len != SIZE {
            return Err(FromValueError::new_other(ToneFromValueError::InvalidSize {
                size: value_len,
            }));
        }

        let (red, value) = value.split_at(8);
        let (green, value) = value.split_at(8);
        let (blue, value) = value.split_at(8);
        let (gray, _value) = value.split_at(8);

        let red = f64::from_le_bytes(red.try_into().unwrap());
        let green = f64::from_le_bytes(green.try_into().unwrap());
        let blue = f64::from_le_bytes(blue.try_into().unwrap());
        let gray = f64::from_le_bytes(gray.try_into().unwrap());

        Ok(Self {
            red,
            green,
            blue,
            gray,
        })
    }
}

impl IntoValue for Tone {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let name = arena.create_symbol(USER_DEFINED_NAME.into());

        let mut value = Vec::with_capacity(32);
        value.extend(self.red.to_le_bytes());
        value.extend(self.green.to_le_bytes());
        value.extend(self.blue.to_le_bytes());
        value.extend(self.gray.to_le_bytes());

        Ok(arena.create_user_defined(name, value).into())
    }
}
