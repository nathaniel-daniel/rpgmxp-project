use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::StringValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

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
            let key = arena
                .get_symbol(key)
                .ok_or(FromValueError::InvalidValueHandle { handle: key.into() })?
                .value();
            match key {
                SWITCH2_VALID_FIELD => {
                    if switch2_valid_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SWITCH2_VALID_FIELD.into(),
                        });
                    }

                    switch2_valid_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                SELF_SWITCH_CH_FIELD => {
                    if self_switch_ch_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SELF_SWITCH_CH_FIELD.into(),
                        });
                    }

                    let value: &StringValue = FromValue::from_value(arena, value, visited)?;
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

                    switch1_id_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                SWITCH1_VALID_FIELD => {
                    if switch1_valid_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SWITCH1_VALID_FIELD.into(),
                        });
                    }

                    switch1_valid_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                VARIABLE_VALUE_FIELD => {
                    if variable_value_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: VARIABLE_VALUE_FIELD.into(),
                        });
                    }

                    variable_value_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                SELF_SWITCH_VALID_FIELD => {
                    if self_switch_valid_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SELF_SWITCH_VALID_FIELD.into(),
                        });
                    }

                    self_switch_valid_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                VARIABLE_ID_FIELD => {
                    if variable_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: VARIABLE_ID_FIELD.into(),
                        });
                    }

                    variable_id_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                VARIABLE_VALID_FIELD => {
                    if variable_valid_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: VARIABLE_VALID_FIELD.into(),
                        });
                    }

                    variable_valid_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                SWITCH2_ID_FIELD => {
                    if switch2_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: SWITCH2_ID_FIELD.into(),
                        });
                    }

                    switch2_id_field = Some(FromValue::from_value(arena, value, visited)?);
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
