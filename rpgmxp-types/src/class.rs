use crate::ClassLearning;
use crate::Table;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Class")]
pub struct Class {
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    pub armor_set: Vec<i32>,
    pub learnings: Vec<ClassLearning>,
    pub weapon_set: Vec<i32>,
    pub state_ranks: Table,
    pub position: i32,
    pub id: i32,
    pub element_ranks: Table,
}
