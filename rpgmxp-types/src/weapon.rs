#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Weapon")]
pub struct Weapon {
    pub int_plus: i32,
    pub animation1_id: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    pub minus_state_set: Vec<i32>,
    pub mdef: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub icon_name: String,
    pub agi_plus: i32,
    pub price: i32,
    pub plus_state_set: Vec<i32>,
    pub pdef: i32,
    pub dex_plus: i32,
    pub element_set: Vec<i32>,
    pub atk: i32,
    pub animation2_id: i32,
    pub id: i32,
    pub str_plus: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub description: String,
}
