use rpgmxp_types::AudioFile;
use rpgmxp_types::Event;
use rpgmxp_types::Table;
use ruby_marshal::ArrayValue;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::SymbolValue;
use ruby_marshal::Value;
use std::collections::HashMap;

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Map {
    pub bgm: AudioFile,
    pub tileset_id: i32,
    pub events: HashMap<i32, Event>,
    pub bgs: AudioFile,
    pub autoplay_bgm: bool,
    pub data: Table,
    pub autoplay_bgs: bool,
    pub height: i32,
    pub encounter_step: i32,
    pub width: i32,
    pub encounter_list: Vec<()>,
}

impl<'a> FromValue<'a> for Map {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let object: &ObjectValue = FromValue::from_value(ctx, value)?;
        let name = object.name();
        let name: &SymbolValue = ctx.from_value(name.into())?;
        let name = name.value();

        if name != OBJECT_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let instance_variables = object.instance_variables();

        let mut bgm_field = None;
        let mut tileset_id_field = None;
        let mut events_field = None;
        let mut bgs_field = None;
        let mut autoplay_bgm_field = None;
        let mut data_field = None;
        let mut autoplay_bgs_field = None;
        let mut height_field = None;
        let mut encounter_step_field = None;
        let mut width_field = None;
        let mut encounter_list_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key: &SymbolValue = ctx.from_value(key.into())?;
            let key = key.value();

            match key {
                BGM_FIELD => {
                    if bgm_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let bgm = ctx.from_value(value)?;
                    bgm_field = Some(bgm);
                }
                TILESET_ID_FIELD => {
                    if tileset_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    tileset_id_field = Some(ctx.from_value(value)?);
                }
                EVENTS_FIELD => {
                    if events_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    events_field = Some(ctx.from_value(value)?);
                }
                BGS_FIELD => {
                    if bgs_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    bgs_field = Some(ctx.from_value(value)?);
                }
                AUTOPLAY_BGM_FIELD => {
                    if autoplay_bgm_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    autoplay_bgm_field = Some(ctx.from_value(value)?);
                }
                DATA_FIELD => {
                    if data_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    data_field = Some(ctx.from_value(value)?);
                }
                AUTOPLAY_BGS_FIELD => {
                    if autoplay_bgs_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    autoplay_bgs_field = Some(ctx.from_value(value)?);
                }
                HEIGHT_FIELD => {
                    if height_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    height_field = Some(ctx.from_value(value)?);
                }
                ENCOUNTER_STEP_FIELD => {
                    if encounter_step_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    encounter_step_field = Some(ctx.from_value(value)?);
                }
                WIDTH_FIELD => {
                    if width_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    width_field = Some(ctx.from_value(value)?);
                }
                ENCOUNTER_LIST_FIELD => {
                    if encounter_list_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let encounter_list: &ArrayValue = ctx.from_value(value)?;

                    if !encounter_list.is_empty() {
                        todo!("ENCOUNTER_LIST");
                    }

                    encounter_list_field = Some(Vec::new());
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
        let events = events_field.ok_or(FromValueError::MissingInstanceVariable {
            name: EVENTS_FIELD.into(),
        })?;
        let bgs = bgs_field.ok_or(FromValueError::MissingInstanceVariable {
            name: BGS_FIELD.into(),
        })?;
        let autoplay_bgm = autoplay_bgm_field.ok_or(FromValueError::MissingInstanceVariable {
            name: AUTOPLAY_BGM_FIELD.into(),
        })?;
        let data = data_field.ok_or(FromValueError::MissingInstanceVariable {
            name: DATA_FIELD.into(),
        })?;
        let autoplay_bgs = autoplay_bgs_field.ok_or(FromValueError::MissingInstanceVariable {
            name: AUTOPLAY_BGS_FIELD.into(),
        })?;
        let height = height_field.ok_or(FromValueError::MissingInstanceVariable {
            name: HEIGHT_FIELD.into(),
        })?;
        let encounter_step =
            encounter_step_field.ok_or(FromValueError::MissingInstanceVariable {
                name: ENCOUNTER_STEP_FIELD.into(),
            })?;
        let width = width_field.ok_or(FromValueError::MissingInstanceVariable {
            name: WIDTH_FIELD.into(),
        })?;
        let encounter_list =
            encounter_list_field.ok_or(FromValueError::MissingInstanceVariable {
                name: ENCOUNTER_LIST_FIELD.into(),
            })?;

        Ok(Self {
            bgm,
            tileset_id,
            events,
            bgs,
            autoplay_bgm,
            data,
            autoplay_bgs,
            height,
            encounter_step,
            width,
            encounter_list,
        })
    }
}
