use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::StringValue;
use ruby_marshal::SymbolValue;
use ruby_marshal::Value;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;

const OBJECT_NAME: &[u8] = b"RPG::Event::Page::Graphic";

const OPACITY_FIELD: &[u8] = b"@opacity";
const CHARACTER_NAME_FIELD: &[u8] = b"@character_name";
const PATTERN_FIELD: &[u8] = b"@pattern";
const TILE_ID_FIELD: &[u8] = b"@tile_id";
const DIRECTION_FIELD: &[u8] = b"@direction";
const BLEND_TYPE_FIELD: &[u8] = b"@blend_type";
const CHARACTER_HUE_FIELD: &[u8] = b"@character_hue";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct EventPageGraphic {
    pub opacity: i32,
    pub character_name: String,
    pub pattern: i32,
    pub tile_id: i32,
    pub direction: i32,
    pub blend_type: i32,
    pub character_hue: i32,
}

impl<'a> FromValue<'a> for EventPageGraphic {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let object: &ObjectValue = FromValue::from_value(ctx, value)?;
        let name = object.name();
        let name: &SymbolValue = ctx.from_value(name.into())?;
        let name = name.value();

        if name != OBJECT_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let instance_variables = object.instance_variables();

        let mut opacity_field = None;
        let mut character_name_field = None;
        let mut pattern_field = None;
        let mut tile_id_field = None;
        let mut direction_field = None;
        let mut blend_type_field = None;
        let mut character_hue_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key: &SymbolValue = ctx.from_value(key.into())?;
            let key = key.value();

            match key {
                OPACITY_FIELD => {
                    if opacity_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: OPACITY_FIELD.into(),
                        });
                    }

                    let opacity: i32 = ctx.from_value(value)?;
                    opacity_field = Some(opacity);
                }
                CHARACTER_NAME_FIELD => {
                    if character_name_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: CHARACTER_NAME_FIELD.into(),
                        });
                    }

                    let character_name: &StringValue = ctx.from_value(value)?;
                    let character_name = std::str::from_utf8(character_name.value())
                        .map_err(FromValueError::new_other)?;

                    character_name_field = Some(character_name.into());
                }
                PATTERN_FIELD => {
                    if pattern_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: PATTERN_FIELD.into(),
                        });
                    }

                    let pattern: i32 = ctx.from_value(value)?;
                    pattern_field = Some(pattern);
                }
                TILE_ID_FIELD => {
                    if tile_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: TILE_ID_FIELD.into(),
                        });
                    }

                    let tile_id: i32 = ctx.from_value(value)?;
                    tile_id_field = Some(tile_id);
                }
                DIRECTION_FIELD => {
                    if direction_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: DIRECTION_FIELD.into(),
                        });
                    }

                    let direction: i32 = ctx.from_value(value)?;
                    direction_field = Some(direction);
                }
                BLEND_TYPE_FIELD => {
                    if blend_type_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: BLEND_TYPE_FIELD.into(),
                        });
                    }

                    let blend_type: i32 = ctx.from_value(value)?;
                    blend_type_field = Some(blend_type);
                }
                CHARACTER_HUE_FIELD => {
                    if character_hue_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: CHARACTER_HUE_FIELD.into(),
                        });
                    }

                    let character_hue: i32 = ctx.from_value(value)?;
                    character_hue_field = Some(character_hue);
                }
                _ => return Err(FromValueError::UnknownInstanceVariable { name: key.into() }),
            }
        }

        let opacity = opacity_field.ok_or(FromValueError::MissingInstanceVariable {
            name: OPACITY_FIELD.into(),
        })?;
        let character_name =
            character_name_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: CHARACTER_NAME_FIELD.into(),
            })?;
        let pattern = pattern_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: PATTERN_FIELD.into(),
        })?;
        let tile_id = tile_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: TILE_ID_FIELD.into(),
        })?;
        let direction = direction_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: DIRECTION_FIELD.into(),
        })?;
        let blend_type =
            blend_type_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: BLEND_TYPE_FIELD.into(),
            })?;
        let character_hue =
            character_hue_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: CHARACTER_HUE_FIELD.into(),
            })?;

        Ok(Self {
            opacity,
            character_name,
            pattern,
            tile_id,
            direction,
            blend_type,
            character_hue,
        })
    }
}

impl IntoValue for EventPageGraphic {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let opacity_field_key = arena.create_symbol(OPACITY_FIELD.into());
        let character_name_field_key = arena.create_symbol(CHARACTER_NAME_FIELD.into());
        let pattern_field_key = arena.create_symbol(PATTERN_FIELD.into());
        let tile_id_field_key = arena.create_symbol(TILE_ID_FIELD.into());
        let direction_field_key = arena.create_symbol(DIRECTION_FIELD.into());
        let blend_type_field_key = arena.create_symbol(BLEND_TYPE_FIELD.into());
        let character_hue_field_key = arena.create_symbol(CHARACTER_HUE_FIELD.into());

        let opacity_field_value = self.opacity.into_value(arena)?;
        let character_name_field_value = arena.create_string(self.character_name.into()).into();
        let pattern_field_value = self.pattern.into_value(arena)?;
        let tile_id_field_value = self.tile_id.into_value(arena)?;
        let direction_field_value = self.direction.into_value(arena)?;
        let blend_type_field_value = self.blend_type.into_value(arena)?;
        let character_hue_field_value = self.character_hue.into_value(arena)?;

        let fields = vec![
            (opacity_field_key, opacity_field_value),
            (pattern_field_key, pattern_field_value),
            (character_name_field_key, character_name_field_value),
            (tile_id_field_key, tile_id_field_value),
            (direction_field_key, direction_field_value),
            (blend_type_field_key, blend_type_field_value),
            (character_hue_field_key, character_hue_field_value),
        ];
        let object = arena.create_object(object_name, fields);

        Ok(object.into())
    }
}
