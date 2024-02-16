use crate::EventCommandParameter;
use ruby_marshal::FromValue;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;
use std::collections::HashSet;

pub(crate) const OBJECT_NAME: &[u8] = b"RPG::MoveCommand";

const PARAMETERS_FIELD: &[u8] = b"@parameters";
const CODE_FIELD: &[u8] = b"@code";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MoveCommand {
    pub parameters: Vec<EventCommandParameter>,
    pub code: i32,
}

impl<'a> FromValue<'a> for MoveCommand {
    fn from_value(
        arena: &'a ValueArena,
        handle: ValueHandle,
        visited: &mut HashSet<ValueHandle>,
    ) -> Result<Self, FromValueError> {
        let object: &ObjectValue = FromValue::from_value(arena, handle, visited)?;

        // TODO: Identical MoveCommands appear to be able to be deduped to save space.
        // This breaks our cycle detection, so we need to disable it for this type.
        // Consider a stack-based cycle detector.
        // visited.remove(&handle);
        visited.clear();

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
        let mut code_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key = arena
                .get_symbol(key)
                .ok_or_else(|| FromValueError::InvalidValueHandle { handle: key.into() })?
                .value();

            match key {
                PARAMETERS_FIELD => {
                    if parameters_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    parameters_field = Some(FromValue::from_value(arena, value, visited)?);
                }
                CODE_FIELD => {
                    if code_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    code_field = Some(FromValue::from_value(arena, value, visited)?);
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
        let code = code_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: CODE_FIELD.into(),
        })?;

        Ok(Self { parameters, code })
    }
}

impl IntoValue for MoveCommand {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let parameters_field_key = arena.create_symbol(PARAMETERS_FIELD.into());
        let code_field_key = arena.create_symbol(CODE_FIELD.into());

        let parameters_field_value = self.parameters.into_value(arena)?;
        let code_field_value = self.code.into_value(arena)?;

        let fields = vec![
            (parameters_field_key, parameters_field_value),
            (code_field_key, code_field_value),
        ];

        let object = arena.create_object(object_name, fields);

        Ok(object.into())
    }
}
