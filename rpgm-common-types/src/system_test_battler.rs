#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::System::TestBattler")]
pub struct SystemTestBattler {
    pub actor_id: i32,
    pub level: i32,
    pub weapon_id: i32,
    pub armor1_id: i32,
    pub armor3_id: i32,
    pub armor2_id: i32,
    pub armor4_id: i32,
}
