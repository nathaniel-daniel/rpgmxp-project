use crate::Bgm;
use crate::Bgs;
use crate::Event;
use crate::Table;
use std::collections::HashMap;

/// A Map
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Map")]
pub struct Map {
    pub height: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub parallax_name: String,
    pub events: HashMap<i32, Event>,
    pub bgm: Bgm,
    pub parallax_sx: i32,
    pub width: i32,
    pub encounter_step: i32,
    pub bgs: Bgs,
    pub data: Table,
    pub autoplay_bgm: bool,
    pub parallax_loop_y: bool,
    /// I haven't encountered a non-empty field here.
    /// If you see one open an issue.
    pub encounter_list: Vec<i32>,
    pub autoplay_bgs: bool,
    pub parallax_show: bool,
    pub scroll_type: i32,
    pub parallax_loop_x: bool,
    pub disable_dashing: bool,
    pub parallax_sy: i32,
}
