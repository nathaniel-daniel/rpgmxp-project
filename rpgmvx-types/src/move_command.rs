use crate::MoveCommandParameter;

pub(crate) const OBJECT_NAME: &[u8] = b"RPG::MoveCommand";

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::MoveCommand")]
pub struct MoveCommand {
    pub code: i32,
    pub parameters: Vec<MoveCommandParameter>,
}
