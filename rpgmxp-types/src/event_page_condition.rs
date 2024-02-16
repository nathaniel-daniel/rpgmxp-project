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

const OBJECT_NAME: &[u8] = b"RPG::Event::Page::Condition";

const SWITCH2_VALID_FIELD: &[u8] = b"@switch2_valid";
const SELF_SWITCH_CH_FIELD: &[u8] = b"@self_switch_ch";
const SWITCH1_ID_FIELD: &[u8] = b"@switch1_id";
const SWITCH1_VALID_FIELD: &[u8] = b"@switch1_valid";
const VARIABLE_VALUE_FIELD: &[u8] = b"@variable_value";
const SELF_SWITCH_VALID_FIELD: &[u8] = b"@self_switch_valid";
const VARIABLE_ID_FIELD: &[u8] = b"@variable_id";
const VARIABLE_VALID_FIELD: &[u8] = b"@variable_valid";
const SWITCH2_ID_FIELD: &[u8] = b"@switch2_id";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct EventPageCondition {
    pub switch2_valid: bool,
    pub self_switch_ch: String,
    pub switch1_id: i32,
    pub switch1_valid: bool,
    pub variable_value: i32,
    pub self_switch_valid: bool,
    pub variable_id: i32,
    pub variable_valid: bool,
    pub switch2_id: i32,
}

impl<'a> FromValue<'a> for EventPageCondition {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let object: &ObjectValue = FromValue::from_value(ctx, value)?;
        let name = object.name();
        let name: &SymbolValue = ctx.from_value(name.into())?;
        let name = name.value();
        if name != OBJECT_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let instance_variables = object.instance_variables();

        let mut switch2_valid_field = None;
        let mut self_switch_ch_field = None;
        let mut switch1_id_field = None;
        let mut switch1_valid_field = None;
        let mut variable_value_field = None;
        let mut self_switch_valid_field = None;
        let mut variable_id_field = None;
        let mut variable_valid_field = None;
        let mut switch2_id_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key: &SymbolValue = ctx.from_value(key.into())?;
            let key = key.value();

            match key {
                SWITCH2_VALID_FIELD => {
                    if switch2_valid_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SWITCH2_VALID_FIELD.into(),
                        });
                    }

                    switch2_valid_field = Some(ctx.from_value(value)?);
                }
                SELF_SWITCH_CH_FIELD => {
                    if self_switch_ch_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SELF_SWITCH_CH_FIELD.into(),
                        });
                    }

                    let value: &StringValue = ctx.from_value(value)?;
                    self_switch_ch_field = Some(
                        std::str::from_utf8(value.value())
                            .map_err(FromValueError::new_other)?
                            .into(),
                    );
                }
                SWITCH1_ID_FIELD => {
                    if switch1_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SWITCH1_ID_FIELD.into(),
                        });
                    }

                    switch1_id_field = Some(ctx.from_value(value)?);
                }
                SWITCH1_VALID_FIELD => {
                    if switch1_valid_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SWITCH1_VALID_FIELD.into(),
                        });
                    }

                    switch1_valid_field = Some(ctx.from_value(value)?);
                }
                VARIABLE_VALUE_FIELD => {
                    if variable_value_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: VARIABLE_VALUE_FIELD.into(),
                        });
                    }

                    variable_value_field = Some(ctx.from_value(value)?);
                }
                SELF_SWITCH_VALID_FIELD => {
                    if self_switch_valid_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SELF_SWITCH_VALID_FIELD.into(),
                        });
                    }

                    self_switch_valid_field = Some(ctx.from_value(value)?);
                }
                VARIABLE_ID_FIELD => {
                    if variable_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: VARIABLE_ID_FIELD.into(),
                        });
                    }

                    variable_id_field = Some(ctx.from_value(value)?);
                }
                VARIABLE_VALID_FIELD => {
                    if variable_valid_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: VARIABLE_VALID_FIELD.into(),
                        });
                    }

                    variable_valid_field = Some(ctx.from_value(value)?);
                }
                SWITCH2_ID_FIELD => {
                    if switch2_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SWITCH2_ID_FIELD.into(),
                        });
                    }

                    switch2_id_field = Some(ctx.from_value(value)?);
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let switch2_valid =
            switch2_valid_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: SWITCH2_VALID_FIELD.into(),
            })?;
        let self_switch_ch =
            self_switch_ch_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: SELF_SWITCH_CH_FIELD.into(),
            })?;
        let switch1_id =
            switch1_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: SWITCH1_ID_FIELD.into(),
            })?;
        let switch1_valid =
            switch1_valid_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: SWITCH1_VALID_FIELD.into(),
            })?;
        let variable_value =
            variable_value_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: VARIABLE_VALUE_FIELD.into(),
            })?;
        let self_switch_valid =
            self_switch_valid_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: SELF_SWITCH_VALID_FIELD.into(),
            })?;
        let variable_id =
            variable_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: VARIABLE_ID_FIELD.into(),
            })?;
        let variable_valid =
            variable_valid_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: VARIABLE_VALID_FIELD.into(),
            })?;
        let switch2_id =
            switch2_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: SWITCH2_ID_FIELD.into(),
            })?;

        Ok(Self {
            switch2_valid,
            self_switch_ch,
            switch1_id,
            switch1_valid,
            variable_value,
            self_switch_valid,
            variable_id,
            variable_valid,
            switch2_id,
        })
    }
}

impl IntoValue for EventPageCondition {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let switch2_valid_field_key = arena.create_symbol(SWITCH2_VALID_FIELD.into());
        let self_switch_ch_field_key = arena.create_symbol(SELF_SWITCH_CH_FIELD.into());
        let switch1_id_field_key = arena.create_symbol(SWITCH1_ID_FIELD.into());
        let switch1_valid_field_key = arena.create_symbol(SWITCH1_VALID_FIELD.into());
        let variable_value_field_key = arena.create_symbol(VARIABLE_VALUE_FIELD.into());
        let self_switch_valid_field_key = arena.create_symbol(SELF_SWITCH_VALID_FIELD.into());
        let variable_id_field_key = arena.create_symbol(VARIABLE_ID_FIELD.into());
        let variable_valid_field_key = arena.create_symbol(VARIABLE_VALID_FIELD.into());
        let switch2_id_field_key = arena.create_symbol(SWITCH2_ID_FIELD.into());

        let switch2_valid_field_value = self.switch2_valid.into_value(arena)?;
        let self_switch_ch_field_value = arena.create_string(self.self_switch_ch.into()).into();
        let switch1_id_field_value = self.switch1_id.into_value(arena)?;
        let switch1_valid_field_value = self.switch1_valid.into_value(arena)?;
        let variable_value_field_value = self.variable_value.into_value(arena)?;
        let self_switch_valid_field_value = self.self_switch_valid.into_value(arena)?;
        let variable_id_field_value = self.variable_id.into_value(arena)?;
        let variable_valid_field_value = self.variable_valid.into_value(arena)?;
        let switch2_id_field_value = self.switch2_id.into_value(arena)?;

        let fields = vec![
            (switch2_valid_field_key, switch2_valid_field_value),
            (self_switch_ch_field_key, self_switch_ch_field_value),
            (switch1_id_field_key, switch1_id_field_value),
            (switch1_valid_field_key, switch1_valid_field_value),
            (variable_value_field_key, variable_value_field_value),
            (self_switch_valid_field_key, self_switch_valid_field_value),
            (variable_id_field_key, variable_id_field_value),
            (variable_valid_field_key, variable_valid_field_value),
            (switch2_id_field_key, switch2_id_field_value),
        ];

        let object = arena.create_object(object_name, fields);

        Ok(object.into())
    }
}
