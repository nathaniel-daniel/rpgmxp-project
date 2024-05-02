use crate::Table;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Tileset")]
pub struct Tileset {
    pub panorama_hue: i32,
    pub terrain_tags: Table,
    pub fog_sy: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    pub fog_opacity: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub panorama_name: String,
    pub priorities: Table,
    pub fog_sx: i32,
    pub fog_hue: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string_array2string_array",
        into_value = "crate::util::string_array2ruby_string_array"
    )]
    pub autotile_names: Vec<String>,
    pub passages: Table,
    pub fog_zoom: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub fog_name: String,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub tileset_name: String,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub battleback_name: String,
    pub id: i32,
    pub fog_blend_type: i32,
}
