#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::MapInfo")]
pub struct MapInfo {
    pub scroll_x: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    pub expanded: bool,
    pub order: i32,
    pub scroll_y: i32,
    pub parent_id: i32,
}
