use crate::Table;
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

const OBJECT_NAME: &[u8] = b"RPG::Actor";

const INITIAL_LEVEL_FIELD: &[u8] = b"@initial_level";
const ARMOR4_ID_FIELD: &[u8] = b"@armor4_id";
const WEAPON_ID_FIELD: &[u8] = b"@weapon_id";
const CHARACTER_NAME_FIELD: &[u8] = b"@character_name";
const NAME_FIELD: &[u8] = b"@name";
const EXP_INFLATION_FIELD: &[u8] = b"@exp_inflation";
const PARAMETERS_FIELD: &[u8] = b"@parameters";
const ARMOR2_FIX_FIELD: &[u8] = b"@armor2_fix";
const CLASS_ID_FIELD: &[u8] = b"@class_id";
const ARMOR3_ID_FIELD: &[u8] = b"@armor3_id";
const ARMOR1_FIX_FIELD: &[u8] = b"@armor1_fix";
const EXP_BASIS_FIELD: &[u8] = b"@exp_basis";
const ARMOR2_ID_FIELD: &[u8] = b"@armor2_id";
const BATTLER_HUE_FIELD: &[u8] = b"@battler_hue";
const ARMOR4_FIX_FIELD: &[u8] = b"@armor4_fix";
const FINAL_LEVEL_FIELD: &[u8] = b"@final_level";
const WEAPON_FIX_FIELD: &[u8] = b"@weapon_fix";
const ARMOR1_ID_FIELD: &[u8] = b"@armor1_id";
const ID_FIELD: &[u8] = b"@id";
const CHARACTER_HUE_FIELD: &[u8] = b"@character_hue";
const BATTLER_NAME_FIELD: &[u8] = b"@battler_name";
const ARMOR3_FIX_FIELD: &[u8] = b"@armor3_fix";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Actor {
    pub initial_level: i32,
    pub armor4_id: i32,
    pub weapon_id: i32,
    pub character_name: String,
    pub name: String,
    pub exp_inflation: i32,
    pub parameters: Table,
    pub armor2_fix: bool,
    pub class_id: i32,
    pub armor3_id: i32,
    pub armor1_fix: bool,
    pub exp_basis: i32,
    pub armor2_id: i32,
    pub battler_hue: i32,
    pub armor4_fix: bool,
    pub final_level: i32,
    pub weapon_fix: bool,
    pub armor1_id: i32,
    pub id: i32,
    pub character_hue: i32,
    pub battler_name: String,
    pub armor3_fix: bool,
}

impl<'a> FromValue<'a> for Actor {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let object: &ObjectValue = FromValue::from_value(ctx, value)?;
        let name = object.name();
        let name: &SymbolValue = ctx.from_value(name.into())?;
        let name = name.value();

        if name != OBJECT_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let instance_variables = object.instance_variables();

        let mut initial_level_field = None;
        let mut armor4_id_field = None;
        let mut weapon_id_field = None;
        let mut character_name_field = None;
        let mut name_field = None;
        let mut exp_inflation_field = None;
        let mut parameters_field = None;
        let mut armor2_fix_field = None;
        let mut class_id_field = None;
        let mut armor3_id_field = None;
        let mut armor1_fix_field = None;
        let mut exp_basis_field = None;
        let mut armor2_id_field = None;
        let mut battler_hue_field = None;
        let mut armor4_fix_field = None;
        let mut final_level_field = None;
        let mut weapon_fix_field = None;
        let mut armor1_id_field = None;
        let mut id_field = None;
        let mut character_hue_field = None;
        let mut battler_name_field = None;
        let mut armor3_fix_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key: &SymbolValue = ctx.from_value(key.into())?;
            let key = key.value();

            match key {
                INITIAL_LEVEL_FIELD => {
                    if initial_level_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let value: i32 = ctx.from_value(value)?;
                    initial_level_field = Some(value);
                }
                ARMOR4_ID_FIELD => {
                    if armor4_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    armor4_id_field = Some(value);
                }
                WEAPON_ID_FIELD => {
                    if weapon_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    weapon_id_field = Some(value);
                }
                CHARACTER_NAME_FIELD => {
                    if character_name_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: &StringValue = ctx.from_value(value)?;
                    let value = std::str::from_utf8(value.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    character_name_field = Some(value);
                }
                NAME_FIELD => {
                    if name_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: &StringValue = ctx.from_value(value)?;
                    let value = std::str::from_utf8(value.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    name_field = Some(value);
                }
                EXP_INFLATION_FIELD => {
                    if exp_inflation_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    exp_inflation_field = Some(value);
                }
                PARAMETERS_FIELD => {
                    if parameters_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: Table = ctx.from_value(value)?;
                    parameters_field = Some(value);
                }
                ARMOR2_FIX_FIELD => {
                    if armor2_fix_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: bool = ctx.from_value(value)?;
                    armor2_fix_field = Some(value);
                }
                CLASS_ID_FIELD => {
                    if class_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    class_id_field = Some(value);
                }
                ARMOR3_ID_FIELD => {
                    if armor3_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    armor3_id_field = Some(value);
                }
                ARMOR1_FIX_FIELD => {
                    if armor1_fix_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: bool = ctx.from_value(value)?;
                    armor1_fix_field = Some(value);
                }
                EXP_BASIS_FIELD => {
                    if exp_basis_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    exp_basis_field = Some(value);
                }
                ARMOR2_ID_FIELD => {
                    if armor2_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    armor2_id_field = Some(value);
                }
                BATTLER_HUE_FIELD => {
                    if battler_hue_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    battler_hue_field = Some(value);
                }
                ARMOR4_FIX_FIELD => {
                    if armor4_fix_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: bool = ctx.from_value(value)?;
                    armor4_fix_field = Some(value);
                }
                FINAL_LEVEL_FIELD => {
                    if final_level_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    final_level_field = Some(value);
                }
                WEAPON_FIX_FIELD => {
                    if weapon_fix_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: bool = ctx.from_value(value)?;
                    weapon_fix_field = Some(value);
                }
                ARMOR1_ID_FIELD => {
                    if armor1_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    armor1_id_field = Some(value);
                }
                ID_FIELD => {
                    if id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    id_field = Some(value);
                }
                CHARACTER_HUE_FIELD => {
                    if character_hue_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: i32 = ctx.from_value(value)?;
                    character_hue_field = Some(value);
                }
                BATTLER_NAME_FIELD => {
                    if battler_name_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: &StringValue = ctx.from_value(value)?;
                    let value = std::str::from_utf8(value.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    battler_name_field = Some(value);
                }
                ARMOR3_FIX_FIELD => {
                    if armor3_fix_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }
                    let value: bool = ctx.from_value(value)?;
                    armor3_fix_field = Some(value);
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let initial_level =
            initial_level_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: INITIAL_LEVEL_FIELD.into(),
            })?;
        let armor4_id = armor4_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR4_ID_FIELD.into(),
        })?;
        let weapon_id = weapon_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: WEAPON_ID_FIELD.into(),
        })?;
        let character_name =
            character_name_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: CHARACTER_NAME_FIELD.into(),
            })?;
        let name = name_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: NAME_FIELD.into(),
        })?;
        let exp_inflation =
            exp_inflation_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: EXP_INFLATION_FIELD.into(),
            })?;
        let parameters =
            parameters_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: PARAMETERS_FIELD.into(),
            })?;
        let armor2_fix =
            armor2_fix_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: ARMOR2_FIX_FIELD.into(),
            })?;
        let class_id = class_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: CLASS_ID_FIELD.into(),
        })?;
        let armor3_id = armor3_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR3_ID_FIELD.into(),
        })?;
        let armor1_fix =
            armor1_fix_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: ARMOR1_FIX_FIELD.into(),
            })?;
        let exp_basis = exp_basis_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: EXP_BASIS_FIELD.into(),
        })?;
        let armor2_id = armor2_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR2_ID_FIELD.into(),
        })?;
        let battler_hue =
            battler_hue_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: BATTLER_HUE_FIELD.into(),
            })?;
        let armor4_fix =
            armor4_fix_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: ARMOR4_FIX_FIELD.into(),
            })?;
        let final_level =
            final_level_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: FINAL_LEVEL_FIELD.into(),
            })?;
        let weapon_fix =
            weapon_fix_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: WEAPON_FIX_FIELD.into(),
            })?;
        let armor1_id = armor1_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR1_ID_FIELD.into(),
        })?;
        let id = id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ID_FIELD.into(),
        })?;
        let character_hue =
            character_hue_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: CHARACTER_HUE_FIELD.into(),
            })?;
        let battler_name =
            battler_name_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: BATTLER_NAME_FIELD.into(),
            })?;
        let armor3_fix =
            armor3_fix_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: ARMOR3_FIX_FIELD.into(),
            })?;

        Ok(Self {
            initial_level,
            armor4_id,
            weapon_id,
            character_name,
            name,
            exp_inflation,
            parameters,
            armor2_fix,
            class_id,
            armor3_id,
            armor1_fix,
            exp_basis,
            armor2_id,
            battler_hue,
            armor4_fix,
            final_level,
            weapon_fix,
            armor1_id,
            id,
            character_hue,
            battler_name,
            armor3_fix,
        })
    }
}

impl IntoValue for Actor {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        todo!()
    }
}
