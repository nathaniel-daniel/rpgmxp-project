#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::State")]
pub struct State {
    pub eva: i32,
    pub hold_turn: i32,
    pub hit_rate: i32,
    pub minus_state_set: Vec<i32>,
    pub atk_rate: i32,
    pub zero_hp: bool,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    pub str_rate: i32,
    pub slip_damage: bool,
    pub battle_only: bool,
    pub plus_state_set: Vec<i32>,
    pub int_rate: i32,
    pub nonresistance: bool,
    pub rating: i32,
    pub shock_release_prob: i32,
    pub maxsp_rate: i32,
    pub mdef_rate: i32,
    pub cant_evade: bool,
    pub agi_rate: i32,
    pub restriction: i32,
    pub auto_release_prob: i32,
    pub maxhp_rate: i32,
    pub guard_element_set: Vec<i32>,
    pub cant_get_exp: bool,
    pub pdef_rate: i32,
    pub id: i32,
    pub animation_id: i32,
    pub dex_rate: i32,
}
