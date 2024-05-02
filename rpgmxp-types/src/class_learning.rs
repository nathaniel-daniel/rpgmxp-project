#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Class::Learning")]
pub struct ClassLearning {
    pub level: i32,
    pub skill_id: i32,
}
