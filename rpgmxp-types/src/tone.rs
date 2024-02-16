use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::Value;

// pub(crate) const USER_DEFINED_NAME: &[u8] = b"RPG::AudioFile";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Tone {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub gray: f64,
}

impl<'a> FromValue<'a> for Tone {
    fn from_value(_ctx: &FromValueContext, _value: &Value) -> Result<Self, FromValueError> {
        todo!()
    }
}
