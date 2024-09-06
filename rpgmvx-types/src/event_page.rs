use crate::EventCommand;
use crate::EventPageCondition;
use crate::EventPageGraphic;
use crate::MoveRoute;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Event::Page")]
pub struct EventPage {
    pub move_route: MoveRoute,
    pub trigger: i32,
    pub step_anime: bool,
    pub move_frequency: i32,
    pub priority_type: i32,
    pub graphic: EventPageGraphic,
    pub walk_anime: bool,
    pub move_speed: i32,
    pub through: bool,
    pub list: Vec<EventCommand>,
    pub move_type: i32,
    pub direction_fix: bool,
    pub condition: EventPageCondition,
}
