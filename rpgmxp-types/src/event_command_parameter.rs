use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

/// An event command parameter
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EventCommandParameter {}

impl<'a> FromValue<'a> for EventCommandParameter {
    fn from_value(
        _: &'a ValueArena,
        _: ValueHandle,
        _: &mut HashSet<ValueHandle>,
    ) -> std::result::Result<Self, FromValueError> {
        todo!("FromValue is stubbed for EventCommandParameter")
    }
}
