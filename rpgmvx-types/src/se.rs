pub(crate) const OBJECT_NAME: &[u8] = b"RPG::SE";

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::SE")]
pub struct Se {
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    pub volume: i32,
    pub pitch: i32,
}