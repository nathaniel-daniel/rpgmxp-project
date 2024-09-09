#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Event::Page::Graphic")]
pub struct EventPageGraphic {
    pub direction: i32,
    pub character_index: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub character_name: String,
    pub pattern: i32,
    pub tile_id: i32,
}
