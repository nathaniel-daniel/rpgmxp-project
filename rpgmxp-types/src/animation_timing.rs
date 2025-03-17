use crate::AudioFile;
use crate::Color;

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    ruby_marshal_derive::FromValue,
    ruby_marshal_derive::IntoValue,
)]
#[ruby_marshal(object = b"RPG::Animation::Timing")]
pub struct AnimationTiming {
    pub frame: i32,
    pub se: AudioFile,
    pub flash_scope: i32,
    pub flash_color: Color,
    pub flash_duration: i32,
    pub condition: i32,
}
