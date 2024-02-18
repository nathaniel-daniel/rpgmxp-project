use crate::AudioFile;
use crate::Event;
use crate::Table;
use ruby_marshal::ArrayValue;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::SymbolValue;
use ruby_marshal::Value;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
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

/// An error that may occur while creating a map from a ruby value.
#[derive(Debug)]
pub enum MapFromValueError {
    /// We currently don't support a non-empty encounter list.
    NonEmptyEncounterList,
}

impl std::fmt::Display for MapFromValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NonEmptyEncounterList => {
                write!(f, "a non-empty map encounter_list was encountered")
            }
        }
    }
}

impl std::error::Error for MapFromValueError {}

/// An error that may occur when transforming a map into a ruby value
#[derive(Debug)]
pub enum MapIntoValueError {
    /// We currently don't support a non-empty encounter list.
    NonEmptyEncounterList,
}

impl std::fmt::Display for MapIntoValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NonEmptyEncounterList => {
                write!(f, "a non-empty map encounter_list was provided")
            }
        }
    }
}

impl std::error::Error for MapIntoValueError {}

/// A Map
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

    /// I haven't encountered a non-empty field here.
    /// If you see one open an issue.
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
                        return Err(FromValueError::new_other(
                            MapFromValueError::NonEmptyEncounterList,
                        ));
                    }

                    encounter_list_field = Some(Vec::new());
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let bgm = bgm_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: BGM_FIELD.into(),
        })?;
        let tileset_id =
            tileset_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: TILESET_ID_FIELD.into(),
            })?;
        let events = events_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: EVENTS_FIELD.into(),
        })?;
        let bgs = bgs_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: BGS_FIELD.into(),
        })?;
        let autoplay_bgm =
            autoplay_bgm_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: AUTOPLAY_BGM_FIELD.into(),
            })?;
        let data = data_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: DATA_FIELD.into(),
        })?;
        let autoplay_bgs =
            autoplay_bgs_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: AUTOPLAY_BGS_FIELD.into(),
            })?;
        let height = height_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: HEIGHT_FIELD.into(),
        })?;
        let encounter_step =
            encounter_step_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: ENCOUNTER_STEP_FIELD.into(),
            })?;
        let width = width_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: WIDTH_FIELD.into(),
        })?;
        let encounter_list =
            encounter_list_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
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

impl IntoValue for Map {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let bgm_field_key = arena.create_symbol(BGM_FIELD.into());
        let tileset_id_field_key = arena.create_symbol(TILESET_ID_FIELD.into());
        let events_field_key = arena.create_symbol(EVENTS_FIELD.into());
        let bgs_field_key = arena.create_symbol(BGS_FIELD.into());
        let autoplay_bgm_field_key = arena.create_symbol(AUTOPLAY_BGM_FIELD.into());
        let data_field_key = arena.create_symbol(DATA_FIELD.into());
        let autoplay_bgs_field_key = arena.create_symbol(AUTOPLAY_BGS_FIELD.into());
        let height_field_key = arena.create_symbol(HEIGHT_FIELD.into());
        let encounter_step_field_key = arena.create_symbol(ENCOUNTER_STEP_FIELD.into());
        let width_field_key = arena.create_symbol(WIDTH_FIELD.into());
        let encounter_list_field_key = arena.create_symbol(ENCOUNTER_LIST_FIELD.into());

        let bgm_field_value = self.bgm.into_value(arena)?;
        let tileset_id_field_value = self.tileset_id.into_value(arena)?;
        let events_field_value = self.events.into_value(arena)?;
        let bgs_field_value = self.bgs.into_value(arena)?;
        let autoplay_bgm_field_value = self.autoplay_bgm.into_value(arena)?;
        let data_field_value = self.data.into_value(arena)?;
        let autoplay_bgs_field_value = self.autoplay_bgs.into_value(arena)?;
        let height_field_value = self.height.into_value(arena)?;
        let encounter_step_field_value = self.encounter_step.into_value(arena)?;
        let width_field_value = self.width.into_value(arena)?;
        if !self.encounter_list.is_empty() {
            return Err(IntoValueError::new_other(
                MapIntoValueError::NonEmptyEncounterList,
            ));
        }
        let encounter_list_field_value = arena.create_array(Vec::new()).into();

        let fields = vec![
            (bgm_field_key, bgm_field_value),
            (tileset_id_field_key, tileset_id_field_value),
            (events_field_key, events_field_value),
            (bgs_field_key, bgs_field_value),
            (autoplay_bgm_field_key, autoplay_bgm_field_value),
            (data_field_key, data_field_value),
            (autoplay_bgs_field_key, autoplay_bgs_field_value),
            (height_field_key, height_field_value),
            (encounter_step_field_key, encounter_step_field_value),
            (width_field_key, width_field_value),
            (encounter_list_field_key, encounter_list_field_value),
        ];

        let object = arena.create_object(object_name, fields);

        Ok(object.into())
    }
}
