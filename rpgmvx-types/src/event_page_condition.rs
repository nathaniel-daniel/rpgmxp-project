#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Event::Page::Condition")]
pub struct EventPageCondition {
    pub switch1_valid: bool,
    pub switch2_valid: bool,
    pub variable_valid: bool,
    pub self_switch_valid: bool,
    pub item_valid: bool,
    pub actor_valid: bool,
    pub switch1_id: i32,
    pub switch2_id: i32,
    pub variable_id: i32,
    pub variable_value: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub self_switch_ch: String,
    pub item_id: i32,
    pub actor_id: i32,
}
