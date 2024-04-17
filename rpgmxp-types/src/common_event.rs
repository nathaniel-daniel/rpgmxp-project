use crate::EventCommand;
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

const OBJECT_NAME: &[u8] = b"RPG::CommonEvent";

const NAME_FIELD: &[u8] = b"@name";
const LIST_FIELD: &[u8] = b"@list";
const TRIGGER_FIELD: &[u8] = b"@trigger";
const SWITCH_ID_FIELD: &[u8] = b"@switch_id";
const ID_FIELD: &[u8] = b"@id";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CommonEvent {
    pub name: String,
    pub list: Vec<EventCommand>,
    pub trigger: i32,
    pub switch_id: i32,
    pub id: i32,
}

impl<'a> FromValue<'a> for CommonEvent {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let object: &ObjectValue = FromValue::from_value(ctx, value)?;
        let name = object.name();
        let name: &SymbolValue = ctx.from_value(name.into())?;
        let name = name.value();
        if name != OBJECT_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let instance_variables = object.instance_variables();

        let mut name_field = None;
        let mut list_field = None;
        let mut trigger_field = None;
        let mut switch_id_field = None;
        let mut id_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key: &SymbolValue = ctx.from_value(key.into())?;
            let key = key.value();

            match key {
                NAME_FIELD => {
                    if name_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: NAME_FIELD.into(),
                        });
                    }

                    let name: &StringValue = ctx.from_value(value)?;
                    let name =
                        std::str::from_utf8(name.value()).map_err(FromValueError::new_other)?;

                    name_field = Some(name.into());
                }
                LIST_FIELD => {
                    if list_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: LIST_FIELD.into(),
                        });
                    }

                    list_field = Some(ctx.from_value(value)?);
                }
                TRIGGER_FIELD => {
                    if trigger_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: TRIGGER_FIELD.into(),
                        });
                    }

                    trigger_field = Some(ctx.from_value(value)?);
                }
                SWITCH_ID_FIELD => {
                    if switch_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SWITCH_ID_FIELD.into(),
                        });
                    }

                    switch_id_field = Some(ctx.from_value(value)?);
                }
                ID_FIELD => {
                    if id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: ID_FIELD.into(),
                        });
                    }

                    id_field = Some(ctx.from_value(value)?);
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let name = name_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: NAME_FIELD.into(),
        })?;
        let list = list_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: LIST_FIELD.into(),
        })?;
        let trigger = trigger_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: TRIGGER_FIELD.into(),
        })?;
        let switch_id = switch_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: SWITCH_ID_FIELD.into(),
        })?;
        let id = id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ID_FIELD.into(),
        })?;

        Ok(Self {
            name,
            list,
            trigger,
            switch_id,
            id,
        })
    }
}

impl IntoValue for CommonEvent {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let name_field_key = arena.create_symbol(NAME_FIELD.into());
        let list_field_key = arena.create_symbol(LIST_FIELD.into());
        let trigger_field_key = arena.create_symbol(TRIGGER_FIELD.into());
        let switch_id_field_key = arena.create_symbol(SWITCH_ID_FIELD.into());
        let id_field_key = arena.create_symbol(ID_FIELD.into());

        let name_field_value = arena.create_string(self.name.into()).into();
        let list_field_value = self.list.into_value(arena)?;
        let trigger_field_value = self.trigger.into_value(arena)?;
        let switch_id_field_value = self.switch_id.into_value(arena)?;
        let id_field_value = self.id.into_value(arena)?;

        let fields = vec![
            (name_field_key, name_field_value),
            (list_field_key, list_field_value),
            (trigger_field_key, trigger_field_value),
            (switch_id_field_key, switch_id_field_value),
            (id_field_key, id_field_value),
        ];

        let object = arena.create_object(object_name, fields);

        Ok(object.into())
    }
}
