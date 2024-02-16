use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

// pub(crate) const USER_DEFINED_NAME: &[u8] = b"RPG::AudioFile";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Tone {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub gray: f64,
}

impl<'a> FromValue<'a> for Tone {
    fn from_value(
        _arena: &'a ValueArena,
        _handle: ValueHandle,
        _visited: &mut HashSet<ValueHandle>,
    ) -> Result<Self, FromValueError> {
        todo!()
    }
}
