use crate::EventCommandParameter;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

const OBJECT_NAME: &[u8] = b"RPG::EventCommand";

const PARAMETERS_FIELD: &[u8] = b"@parameters";
const CODE_FIELD: &[u8] = b"@code";
const INDENT_FIELD: &[u8] = b"@indent";

/// An Event Command
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EventCommand {
    /// Event Command Parameters
    pub parameters: Vec<EventCommandParameter>,
    /// The indent
    pub indent: i32,
    /// The command code
    pub code: i32,
}

impl<'a> FromValue<'a> for EventCommand {
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

        let mut parameters_field = None;
        let mut indent_field = None;
        let mut code_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key = arena
                .get_symbol(key)
                .ok_or(FromValueError::InvalidValueHandle { handle: key.into() })?
                .value();

            match key {
                PARAMETERS_FIELD => {
                    if parameters_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: PARAMETERS_FIELD.into(),
                        });
                    }

                    parameters_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                CODE_FIELD => {
                    if code_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: CODE_FIELD.into(),
                        });
                    }

                    code_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                INDENT_FIELD => {
                    if indent_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable {
                            name: INDENT_FIELD.into(),
                        });
                    }

                    indent_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let parameters =
            parameters_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: PARAMETERS_FIELD.into(),
            })?;
        let indent = indent_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: INDENT_FIELD.into(),
        })?;
        let code = code_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: CODE_FIELD.into(),
        })?;

        Ok(Self {
            parameters,
            indent,
            code,
        })
    }
}

impl IntoValue for EventCommand {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let parameters_field_key = arena.create_symbol(PARAMETERS_FIELD.into());
        let code_field_key = arena.create_symbol(CODE_FIELD.into());
        let indent_field_key = arena.create_symbol(INDENT_FIELD.into());

        let parameters_field_value = self.parameters.into_value(arena)?;
        let code_field_value = self.code.into_value(arena)?;
        let indent_field_value = self.indent.into_value(arena)?;

        let fields = vec![
            (parameters_field_key, parameters_field_value),
            (code_field_key, code_field_value),
            (indent_field_key, indent_field_value),
        ];

        let object = arena.create_object(object_name, fields);

        Ok(object.into())
    }
}
