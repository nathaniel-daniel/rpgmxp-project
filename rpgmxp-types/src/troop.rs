use crate::TroopPage;
use crate::TroopMember;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Troop")]
pub struct Troop {
    pub pages: Vec<TroopPage>,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    pub id: i32,
    pub members: Vec<TroopMember>,
}