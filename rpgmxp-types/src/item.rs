
use crate::AudioFile;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Item")]
pub struct Item {
    pub animation1_id: i32,
    pub consumable: bool,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub icon_name: String,
    pub minus_state_set: Vec<i32>,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    pub mdef_f: i32,
    pub common_event_id: i32,
    pub recover_hp_rate: i32,
    pub occasion: i32,
    pub price: i32,
    pub recover_sp: i32,
    pub plus_state_set: Vec<i32>,
    pub pdef_f: i32,
    pub menu_se: AudioFile,
    pub parameter_points: i32,
    pub scope: i32,
    pub recover_sp_rate: i32,
    pub element_set: Vec<i32>,
    pub hit: i32,
    pub animation2_id: i32,
    pub parameter_type: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub description: String,
    pub id: i32,
    pub variance: i32,
    pub recover_hp: i32,
}
