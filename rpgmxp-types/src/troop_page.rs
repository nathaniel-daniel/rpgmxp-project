use crate::EventCommand;
use crate::TroopPageCondition;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Troop::Page")]
pub struct TroopPage {
    pub list: Vec<EventCommand>,
    pub condition: TroopPageCondition,
    pub span: i32,
}
