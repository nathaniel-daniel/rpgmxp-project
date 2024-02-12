use rpgmxp_types::AudioFile;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
// use rpgmxp_types::Event;
use std::collections::HashSet;

const OBJECT_NAME: &[u8] = b"RPG::Map";

const BGM_FIELD: &[u8] = b"@bgm";
const TILESET_ID_FIELD: &[u8] = b"@tileset_id";
const EVENTS_FIELD: &[u8] = b"@events";
const BGS_FIELD: &[u8] = b"@bgs";
const AUTOPLAY_BGM_FIELD: &[u8] = b"@autoplay_bgm";
const DATA_FIELD: &[u8] = b"@data";
const AUTOPLAY_BGS_FIELD: &[u8] = b"@autoplay_bgs";
const HEIGHT_FIELD: &[u8] = b"@height";
const ENCOUNTER_STEP_FIELD: &[u8] = b"@encounter_step";
const WIDTH_FIELD: &[u8] = b"@width";
const ENCOUNTER_LIST_FIELD: &[u8] = b"@encounter_list";

#[derive(Debug)]
pub struct Map {
    pub bgm: AudioFile,
    pub tileset_id: i32,
    //pub events: Vec<(i32, Event)>,
}

impl<'a> FromValue<'a> for Map {
    fn from_value(
        arena: &'a ValueArena,
        handle: ValueHandle,
        visited: &mut HashSet<ValueHandle>,
    ) -> Result<Self, FromValueError> {
        let object: &ObjectValue = FromValue::from_value(arena, handle, visited)?;
        let name = object.name();
        let name = arena
            .get_symbol(name)
            .ok_or(FromValueError::InvalidValueHandle {
                handle: name.into(),
            })?
            .value();

        if name != OBJECT_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let instance_variables = object.instance_variables();

        let mut bgm_field = None;
        let mut tileset_id_field = None;
        //let mut events_field = None;
        // let mut bgs_field = None;
        // let mut autoplay_bgm_field = None;
        // let mut data_field = None;
        // let mut autoplay_bgs_field = None;
        // let mut height_field = None;
        // let mut encounter_step_field = None;
        // let mut width_field = None;
        //let mut encounter_list_field = None;

        for (key, value) in instance_variables.iter() {
            let key = arena
                .get_symbol(*key)
                .ok_or(FromValueError::InvalidValueHandle {
                    handle: (*key).into(),
                })?
                .value();

            match key {
                BGM_FIELD => {
                    if bgm_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let bgm = AudioFile::from_value(arena, *value, visited)?;
                    bgm_field = Some(bgm);
                }
                TILESET_ID_FIELD => {
                    if tileset_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    tileset_id_field = Some(FromValue::from_value(arena, *value, visited)?);
                }
                EVENTS_FIELD => {
                    /*
                    if events_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    */
                    todo!("Events");

                    // events_field = Some(FromValue::from_value(arena, *value, visited)?);
                }
                _ => {
                    todo!("{:#?}", std::str::from_utf8(key))
                }
            }
        }

        let bgm = bgm_field.ok_or(FromValueError::MissingInstanceVariable {
            name: BGM_FIELD.into(),
        })?;
        let tileset_id = tileset_id_field.ok_or(FromValueError::MissingInstanceVariable {
            name: TILESET_ID_FIELD.into(),
        })?;
        /*
        let events = events_field.ok_or(FromValueError::MissingInstanceVariable {
            name: EVENTS_FIELD.into(),
        })?;
        */

        Ok(Self {
            bgm,
            tileset_id,
            //events,
        })
    }
}
