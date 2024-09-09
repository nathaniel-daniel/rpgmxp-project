use super::Bgm;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::System::Vehicle")]
pub struct SystemVehicle {
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub character_name: String,
    pub character_index: i32,
    pub bgm: Bgm,
    pub start_map_id: i32,
    pub start_x: i32,
    pub start_y: i32,
}
