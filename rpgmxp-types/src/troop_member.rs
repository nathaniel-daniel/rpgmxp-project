#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Troop::Member")]
pub struct TroopMember {
    pub hidden: bool,
    pub y: i32,
    pub enemy_id: i32,
    pub x: i32,
    pub immortal: bool,
}