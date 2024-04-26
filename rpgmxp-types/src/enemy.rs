use crate::Table;
use crate::EnemyAction;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Enemy")]
pub struct Enemy {
    pub animation1_id: i32,
    pub eva: i32,
    #[ruby_marshal(name = b"@str")]
    pub str_: i32,
    pub weapon_id: i32,
    pub gold: i32,
    pub mdef: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    pub int: i32,
    pub treasure_prob: i32,
    pub maxsp: i32,
    pub exp: i32,
    pub pdef: i32,
    pub agi: i32,
    pub armor_id: i32,
    pub maxhp: i32,
    pub state_ranks: Table,
    pub atk: i32,
    pub battler_hue: i32,
    pub animation2_id: i32,
    pub dex: i32,
    pub item_id: i32,
    pub id: i32,
    pub actions: Vec<EnemyAction>,
    pub element_ranks: Table,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub battler_name: String,
}