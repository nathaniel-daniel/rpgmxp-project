use crate::Bgm;
use crate::Me;
use crate::Se;
use crate::SystemTerms;
use crate::SystemTestBattler;
use crate::SystemVehicle;
use crate::Table;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::System")]
pub struct System {
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub game_title: String,
    pub version_id: i32,
    pub party_members: Vec<i32>,
    #[ruby_marshal(
        from_value = "crate::util::optional_ruby_string_array2optional_string_array",
        into_value = "crate::util::optional_string_array2optional_ruby_string_array"
    )]
    pub elements: Vec<Option<String>>,
    #[ruby_marshal(
        from_value = "crate::util::optional_ruby_string_array2optional_string_array",
        into_value = "crate::util::optional_string_array2optional_ruby_string_array"
    )]
    pub switches: Vec<Option<String>>,
    #[ruby_marshal(
        from_value = "crate::util::optional_ruby_string_array2optional_string_array",
        into_value = "crate::util::optional_string_array2optional_ruby_string_array"
    )]
    pub variables: Vec<Option<String>>,
    pub passages: Table,
    pub boat: SystemVehicle,
    pub ship: SystemVehicle,
    pub airship: SystemVehicle,
    pub title_bgm: Bgm,
    pub battle_bgm: Bgm,
    pub battle_end_me: Me,
    pub gameover_me: Me,
    pub sounds: Vec<Se>,
    pub test_battlers: Vec<SystemTestBattler>,
    pub test_troop_id: i32,
    pub start_map_id: i32,
    pub start_x: i32,
    pub start_y: i32,
    pub terms: SystemTerms,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub battler_name: String,
    pub battler_hue: i32,
    pub edit_map_id: i32,
    #[ruby_marshal(name = b"@_")]
    pub underscore: i32,
    pub magic_number: i32,
}
