#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Armor")]
pub struct Armor {
    pub eva: i32,
    pub int_plus: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub icon_name: String,
    pub mdef: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    pub auto_state_id: i32,
    pub price: i32,
    pub agi_plus: i32,
    pub pdef: i32,
    pub guard_state_set: Vec<i32>,
    pub kind: i32,
    pub dex_plus: i32,
    pub guard_element_set: Vec<i32>,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub description: String,
    pub str_plus: i32,
    pub id: i32,
}
