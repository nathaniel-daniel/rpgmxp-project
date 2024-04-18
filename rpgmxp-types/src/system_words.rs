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

const OBJECT_NAME: &[u8] = b"RPG::System::Words";

const STR_FIELD: &[u8] = b"@str";
const ARMOR3_FIELD: &[u8] = b"@armor3";
const MDEF_FIELD: &[u8] = b"@mdef";
const GOLD_FIELD: &[u8] = b"@gold";
const SP_FIELD: &[u8] = b"@sp";
const SKILL_FIELD: &[u8] = b"@skill";
const INT_FIELD: &[u8] = b"@int";
const ARMOR2_FIELD: &[u8] = b"@armor2";
const EQUIP_FIELD: &[u8] = b"@equip";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SystemWords {
    pub str_: String,
    pub armor3: String,
    pub mdef: String,
    pub gold: String,
    pub sp: String,
    pub skill: String,
    pub int: String,
    pub armor2: String,
}

impl<'a> FromValue<'a> for SystemWords {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let object: &ObjectValue = FromValue::from_value(ctx, value)?;
        let name = object.name();
        let name: &SymbolValue = ctx.from_value(name.into())?;
        let name = name.value();

        if name != OBJECT_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let instance_variables = object.instance_variables();
        dbg!(instance_variables.len());

        let mut str_field = None;
        let mut armor3_field = None;
        let mut mdef_field = None;
        let mut gold_field = None;
        let mut sp_field = None;
        let mut skill_field = None;
        let mut int_field = None;
        let mut armor2_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key: &SymbolValue = ctx.from_value(key.into())?;
            let key = key.value();

            match key {
                STR_FIELD => {
                    if str_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let str_: &StringValue = ctx.from_value(value)?;
                    let str_ = std::str::from_utf8(str_.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    str_field = Some(str_);
                }
                ARMOR3_FIELD => {
                    if armor3_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let armor3: &StringValue = ctx.from_value(value)?;
                    let armor3 = std::str::from_utf8(armor3.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    armor3_field = Some(armor3);
                }
                MDEF_FIELD => {
                    if mdef_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let mdef: &StringValue = ctx.from_value(value)?;
                    let mdef = std::str::from_utf8(mdef.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    mdef_field = Some(mdef);
                }
                GOLD_FIELD => {
                    if gold_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let gold: &StringValue = ctx.from_value(value)?;
                    let gold = std::str::from_utf8(gold.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    gold_field = Some(gold);
                }
                SP_FIELD => {
                    if sp_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let sp: &StringValue = ctx.from_value(value)?;
                    let sp = std::str::from_utf8(sp.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    sp_field = Some(sp);
                }
                SKILL_FIELD => {
                    if skill_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let skill: &StringValue = ctx.from_value(value)?;
                    let skill = std::str::from_utf8(skill.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    skill_field = Some(skill);
                }
                INT_FIELD => {
                    if int_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let int: &StringValue = ctx.from_value(value)?;
                    let int = std::str::from_utf8(int.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    int_field = Some(int);
                }
                ARMOR2_FIELD => {
                    if armor2_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let armor2: &StringValue = ctx.from_value(value)?;
                    let armor2 = std::str::from_utf8(armor2.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    armor2_field = Some(armor2);
                }
                _ => {
                    // return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let str_ = str_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: STR_FIELD.into(),
        })?;
        let armor3 = armor3_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR3_FIELD.into(),
        })?;
        let mdef = mdef_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: MDEF_FIELD.into(),
        })?;
        let gold = gold_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: GOLD_FIELD.into(),
        })?;
        let sp = sp_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: SP_FIELD.into(),
        })?;
        let skill = skill_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: SKILL_FIELD.into(),
        })?;
        let int = int_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: INT_FIELD.into(),
        })?;
        let armor2 = armor2_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR2_FIELD.into(),
        })?;

        Ok(Self {
            str_,
            armor3,
            mdef,
            gold,
            sp,
            skill,
            int,
            armor2,
        })
    }
}

impl IntoValue for SystemWords {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        todo!()
    }
}
