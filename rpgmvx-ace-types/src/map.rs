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
    pub tileset_id: i32,
    #[ruby_marshal(
        from_value = "rpgm_common_types::util::ruby_string2string",
        into_value = "rpgm_common_types::util::string2ruby_string"
    )]
    pub parallax_name: String,
    pub width: i32,
    pub height: i32,
    pub events: HashMap<i32, Event>,
    pub parallax_sx: i32,
    pub parallax_sy: i32,
    pub bgm: Bgm,
    pub encounter_step: i32,
    pub data: Table,
    pub bgs: Bgs,
    pub parallax_loop_x: bool,
    pub parallax_loop_y: bool,
    /// I haven't encountered a non-empty field here.
    /// If you see one open an issue.
    pub encounter_list: Vec<i32>,
    pub autoplay_bgm: bool,
    pub disable_dashing: bool,
    pub autoplay_bgs: bool,
    pub parallax_show: bool,
    pub scroll_type: i32,
    #[ruby_marshal(
        from_value = "rpgm_common_types::util::ruby_string2string",
        into_value = "rpgm_common_types::util::string2ruby_string"
    )]
    pub display_name: String,
    pub specify_battleback: bool,
    #[ruby_marshal(
        from_value = "rpgm_common_types::util::ruby_string2string",
        into_value = "rpgm_common_types::util::string2ruby_string"
    )]
    pub note: String,
    #[ruby_marshal(
        from_value = "rpgm_common_types::util::ruby_string2string",
        into_value = "rpgm_common_types::util::string2ruby_string"
    )]
    pub battleback1_name: String,
    #[ruby_marshal(
        from_value = "rpgm_common_types::util::ruby_string2string",
        into_value = "rpgm_common_types::util::string2ruby_string"
    )]
    pub battleback2_name: String,
}
