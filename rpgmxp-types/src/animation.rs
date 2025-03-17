use crate::AnimationFrame;
use crate::AnimationTiming;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Animation")]
pub struct Animation {
    pub id: i32,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub name: String,
    #[ruby_marshal(
        from_value = "crate::util::ruby_string2string",
        into_value = "crate::util::string2ruby_string"
    )]
    pub animation_name: String,
    pub animation_hue: i32,
    pub position: i32,
    pub frame_max: i32,
    pub frames: Vec<AnimationFrame>,
    pub timings: Vec<AnimationTiming>,
}
