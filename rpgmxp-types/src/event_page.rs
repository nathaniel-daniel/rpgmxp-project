use crate::EventCommand;
use crate::EventPageCondition;
use crate::EventPageGraphic;
use crate::MoveRoute;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

const OBJECT_NAME: &[u8] = b"RPG::Event::Page";

const MOVE_TYPE_FIELD: &[u8] = b"@move_type";
const LIST_FIELD: &[u8] = b"@list";
const CONDITION_FIELD: &[u8] = b"@condition";
const DIRECTION_FIX_FIELD: &[u8] = b"@direction_fix";
const MOVE_ROUTE_FIELD: &[u8] = b"@move_route";
const TRIGGER_FIELD: &[u8] = b"@trigger";
const STEP_ANIME_FIELD: &[u8] = b"@step_anime";
const MOVE_FREQUENCY_FIELD: &[u8] = b"@move_frequency";
const GRAPHIC_FIELD: &[u8] = b"@graphic";
const ALWAYS_ON_TOP_FIELD: &[u8] = b"@always_on_top";
const WALK_ANIME_FIELD: &[u8] = b"@walk_anime";
const MOVE_SPEED_FIELD: &[u8] = b"@move_speed";
const THROUGH_FIELD: &[u8] = b"@through";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct EventPage {
    pub move_type: i32,
    pub list: Vec<EventCommand>,
    pub condition: EventPageCondition,
    pub direction_fix: bool,
    pub move_route: MoveRoute,
    pub trigger: i32,
    pub step_anime: bool,
    pub move_frequency: i32,
    pub graphic: EventPageGraphic,
    pub always_on_top: bool,
    pub walk_anime: bool,
    pub move_speed: i32,
    pub through: bool,
}

impl<'a> FromValue<'a> for EventPage {
    fn from_value(
        arena: &ValueArena,
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

        let mut move_type_field = None;
        let mut list_field = None;
        let mut condition_field = None;
        let mut direction_fix_field = None;
        let mut move_route_field = None;
        let mut trigger_field = None;
        let mut step_anime_field = None;
        let mut move_frequency_field = None;
        let mut graphic_field = None;
        let mut always_on_top_field = None;
        let mut walk_anime_field = None;
        let mut move_speed_field = None;
        let mut through_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key = arena
                .get_symbol(key)
                .ok_or(FromValueError::InvalidValueHandle { handle: key.into() })?
                .value();
            match key {
                MOVE_TYPE_FIELD => {
                    if move_type_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: MOVE_TYPE_FIELD.into(),
                        });
                    }

                    move_type_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                LIST_FIELD => {
                    if list_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: LIST_FIELD.into(),
                        });
                    }

                    list_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                CONDITION_FIELD => {
                    if condition_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: CONDITION_FIELD.into(),
                        });
                    }

                    condition_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                DIRECTION_FIX_FIELD => {
                    if direction_fix_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: DIRECTION_FIX_FIELD.into(),
                        });
                    }

                    direction_fix_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                MOVE_ROUTE_FIELD => {
                    if move_route_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: MOVE_ROUTE_FIELD.into(),
                        });
                    }

                    move_route_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                TRIGGER_FIELD => {
                    if trigger_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: TRIGGER_FIELD.into(),
                        });
                    }

                    trigger_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                STEP_ANIME_FIELD => {
                    if step_anime_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: STEP_ANIME_FIELD.into(),
                        });
                    }

                    step_anime_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                MOVE_FREQUENCY_FIELD => {
                    if move_frequency_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: MOVE_FREQUENCY_FIELD.into(),
                        });
                    }

                    move_frequency_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                GRAPHIC_FIELD => {
                    if graphic_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: GRAPHIC_FIELD.into(),
                        });
                    }

                    graphic_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                ALWAYS_ON_TOP_FIELD => {
                    if always_on_top_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: ALWAYS_ON_TOP_FIELD.into(),
                        });
                    }

                    always_on_top_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                WALK_ANIME_FIELD => {
                    if walk_anime_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: WALK_ANIME_FIELD.into(),
                        });
                    }

                    walk_anime_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                MOVE_SPEED_FIELD => {
                    if move_speed_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: MOVE_SPEED_FIELD.into(),
                        });
                    }

                    move_speed_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                THROUGH_FIELD => {
                    if through_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: THROUGH_FIELD.into(),
                        });
                    }

                    through_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let move_type = move_type_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: MOVE_TYPE_FIELD.into(),
        })?;
        let list = list_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: LIST_FIELD.into(),
        })?;
        let condition = condition_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: CONDITION_FIELD.into(),
        })?;
        let direction_fix =
            direction_fix_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: DIRECTION_FIX_FIELD.into(),
            })?;
        let move_route =
            move_route_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: MOVE_ROUTE_FIELD.into(),
            })?;
        let trigger = trigger_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: TRIGGER_FIELD.into(),
        })?;
        let step_anime =
            step_anime_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: STEP_ANIME_FIELD.into(),
            })?;
        let move_frequency =
            move_frequency_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: MOVE_FREQUENCY_FIELD.into(),
            })?;
        let graphic = graphic_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: GRAPHIC_FIELD.into(),
        })?;
        let always_on_top =
            always_on_top_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: ALWAYS_ON_TOP_FIELD.into(),
            })?;
        let walk_anime =
            walk_anime_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: WALK_ANIME_FIELD.into(),
            })?;
        let move_speed =
            move_speed_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: MOVE_SPEED_FIELD.into(),
            })?;
        let through = through_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: THROUGH_FIELD.into(),
        })?;

        Ok(Self {
            move_type,
            list,
            condition,
            direction_fix,
            move_route,
            trigger,
            step_anime,
            move_frequency,
            graphic,
            always_on_top,
            walk_anime,
            move_speed,
            through,
        })
    }
}

impl IntoValue for EventPage {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let move_type_field_key = arena.create_symbol(MOVE_TYPE_FIELD.into());
        let list_field_key = arena.create_symbol(LIST_FIELD.into());
        let condition_field_key = arena.create_symbol(CONDITION_FIELD.into());
        let direction_fix_field_key = arena.create_symbol(DIRECTION_FIX_FIELD.into());
        let move_route_field_key = arena.create_symbol(MOVE_ROUTE_FIELD.into());
        let trigger_field_key = arena.create_symbol(TRIGGER_FIELD.into());
        let step_anime_field_key = arena.create_symbol(STEP_ANIME_FIELD.into());
        let move_frequency_field_key = arena.create_symbol(MOVE_FREQUENCY_FIELD.into());
        let graphic_field_key = arena.create_symbol(GRAPHIC_FIELD.into());
        let always_on_top_field_key = arena.create_symbol(ALWAYS_ON_TOP_FIELD.into());
        let walk_anime_field_key = arena.create_symbol(WALK_ANIME_FIELD.into());
        let move_speed_field_key = arena.create_symbol(MOVE_SPEED_FIELD.into());
        let through_field_key = arena.create_symbol(THROUGH_FIELD.into());

        let move_type_field_value = self.move_type.into_value(arena)?;
        let list_field_value = self.list.into_value(arena)?;
        let condition_field_value = self.condition.into_value(arena)?;
        let direction_fix_field_value = self.direction_fix.into_value(arena)?;
        let move_route_field_value = self.move_route.into_value(arena)?;
        let trigger_field_value = self.trigger.into_value(arena)?;
        let step_anime_field_value = self.step_anime.into_value(arena)?;
        let move_frequency_field_value = self.move_frequency.into_value(arena)?;
        let graphic_field_value = self.graphic.into_value(arena)?;
        let always_on_top_field_value = self.always_on_top.into_value(arena)?;
        let walk_anime_field_value = self.walk_anime.into_value(arena)?;
        let move_speed_field_value = self.move_speed.into_value(arena)?;
        let through_field_value = self.through.into_value(arena)?;

        let fields = vec![
            (move_type_field_key, move_type_field_value),
            (list_field_key, list_field_value),
            (condition_field_key, condition_field_value),
            (direction_fix_field_key, direction_fix_field_value),
            (move_route_field_key, move_route_field_value),
            (trigger_field_key, trigger_field_value),
            (step_anime_field_key, step_anime_field_value),
            (move_frequency_field_key, move_frequency_field_value),
            (graphic_field_key, graphic_field_value),
            (always_on_top_field_key, always_on_top_field_value),
            (walk_anime_field_key, walk_anime_field_value),
            (move_speed_field_key, move_speed_field_value),
            (through_field_key, through_field_value),
        ];

        let object = arena.create_object(object_name, fields);

        Ok(object.into())
    }
}
