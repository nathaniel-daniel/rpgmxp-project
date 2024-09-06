use crate::EventPage;

/// An event
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Event")]
pub struct Event {
    pub y: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    pub x: i32,
    pub pages: Vec<EventPage>,
    pub id: i32,
}
