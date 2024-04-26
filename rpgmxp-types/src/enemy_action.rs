#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Enemy::Action")]
pub struct EnemyAction {
    pub basic: i32,
    pub condition_hp: i32,
    pub rating: i32,
    pub condition_turn_b: i32,
    pub kind: i32,
    pub condition_switch_id: i32,
    pub condition_turn_a: i32,
    pub condition_level: i32,
    pub skill_id: i32,
}