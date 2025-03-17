use crate::Table;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Animation::Frame")]
pub struct AnimationFrame {
    pub cell_max: i32,
    pub cell_data: Table,
}