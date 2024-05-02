#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Troop::Page::Condition")]
pub struct TroopPageCondition {
    pub actor_id: i32,
    pub turn_a: i32,
    pub turn_valid: bool,
    pub enemy_hp: i32,
    pub switch_valid: bool,
    pub switch_id: i32,
    pub enemy_index: i32,
    pub actor_valid: bool,
    pub actor_hp: i32,
    pub turn_b: i32,
    pub enemy_valid: bool,
}
