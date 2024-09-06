use crate::MoveCommand;

pub(crate) const OBJECT_NAME: &[u8] = b"RPG::MoveRoute";

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::MoveRoute")]
pub struct MoveRoute {
    pub wait: bool,
    pub skippable: bool,
    pub repeat: bool,
    pub list: Vec<MoveCommand>,
}
