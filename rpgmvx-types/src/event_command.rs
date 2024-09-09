use crate::EventCommandParameter;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::EventCommand")]
pub struct EventCommand {
    pub indent: i32,
    pub code: i32,
    pub parameters: Vec<EventCommandParameter>,
}
