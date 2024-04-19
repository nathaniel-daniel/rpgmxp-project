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
const HP_FIELD: &[u8] = b"@hp";
const PDEF_FIELD: &[u8] = b"@pdef";
const ATTACK_FIELD: &[u8] = b"@attack";
const AGI_FIELD: &[u8] = b"@agi";
const ARMOR1_FIELD: &[u8] = b"@armor1";
const ATK_FIELD: &[u8] = b"@atk";
const ITEM_FIELD: &[u8] = b"@item";
const DEX_FIELD: &[u8] = b"@dex";
const ARMOR4_FIELD: &[u8] = b"@armor4";
const WEAPON_FIELD: &[u8] = b"@weapon";
const GUARD_FIELD: &[u8] = b"@guard";

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
    pub equip: String,
    pub hp: String,
    pub pdef: String,
    pub attack: String,
    pub agi: String,
    pub armor1: String,
    pub atk: String,
    pub item: String,
    pub dex: String,
    pub armor4: String,
    pub weapon: String,
    pub guard: String,
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

        let mut str_field = None;
        let mut armor3_field = None;
        let mut mdef_field = None;
        let mut gold_field = None;
        let mut sp_field = None;
        let mut skill_field = None;
        let mut int_field = None;
        let mut armor2_field = None;
        let mut equip_field = None;
        let mut hp_field = None;
        let mut pdef_field = None;
        let mut attack_field = None;
        let mut agi_field = None;
        let mut armor1_field = None;
        let mut atk_field = None;
        let mut item_field = None;
        let mut dex_field = None;
        let mut armor4_field = None;
        let mut weapon_field = None;
        let mut guard_field = None;

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
                EQUIP_FIELD => {
                    if equip_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let equip: &StringValue = ctx.from_value(value)?;
                    let equip = std::str::from_utf8(equip.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    equip_field = Some(equip);
                }
                HP_FIELD => {
                    if hp_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let hp: &StringValue = ctx.from_value(value)?;
                    let hp = std::str::from_utf8(hp.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    hp_field = Some(hp);
                }
                PDEF_FIELD => {
                    if pdef_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let pdef: &StringValue = ctx.from_value(value)?;
                    let pdef = std::str::from_utf8(pdef.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    pdef_field = Some(pdef);
                }
                ATTACK_FIELD => {
                    if attack_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let attack: &StringValue = ctx.from_value(value)?;
                    let attack = std::str::from_utf8(attack.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    attack_field = Some(attack);
                }
                AGI_FIELD => {
                    if agi_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let agi: &StringValue = ctx.from_value(value)?;
                    let agi = std::str::from_utf8(agi.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    agi_field = Some(agi);
                }
                ARMOR1_FIELD => {
                    if armor1_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let armor1: &StringValue = ctx.from_value(value)?;
                    let armor1 = std::str::from_utf8(armor1.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    armor1_field = Some(armor1);
                }
                ATK_FIELD => {
                    if atk_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let atk: &StringValue = ctx.from_value(value)?;
                    let atk = std::str::from_utf8(atk.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    atk_field = Some(atk);
                }
                ITEM_FIELD => {
                    if item_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let item: &StringValue = ctx.from_value(value)?;
                    let item = std::str::from_utf8(item.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    item_field = Some(item);
                }
                DEX_FIELD => {
                    if dex_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let dex: &StringValue = ctx.from_value(value)?;
                    let dex = std::str::from_utf8(dex.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    dex_field = Some(dex);
                }
                ARMOR4_FIELD => {
                    if armor4_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let armor4: &StringValue = ctx.from_value(value)?;
                    let armor4 = std::str::from_utf8(armor4.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    armor4_field = Some(armor4);
                }
                WEAPON_FIELD => {
                    if weapon_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let weapon: &StringValue = ctx.from_value(value)?;
                    let weapon = std::str::from_utf8(weapon.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    weapon_field = Some(weapon);
                }
                GUARD_FIELD => {
                    if guard_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let guard: &StringValue = ctx.from_value(value)?;
                    let guard = std::str::from_utf8(guard.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    guard_field = Some(guard);
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
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
        let equip = equip_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: EQUIP_FIELD.into(),
        })?;
        let hp = hp_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: HP_FIELD.into(),
        })?;
        let pdef = pdef_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: PDEF_FIELD.into(),
        })?;
        let attack = attack_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ATTACK_FIELD.into(),
        })?;
        let agi = agi_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: AGI_FIELD.into(),
        })?;
        let armor1 = armor1_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR1_FIELD.into(),
        })?;
        let atk = atk_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ATK_FIELD.into(),
        })?;
        let item = item_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ITEM_FIELD.into(),
        })?;
        let dex = dex_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: DEX_FIELD.into(),
        })?;
        let armor4 = armor4_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR4_FIELD.into(),
        })?;
        let weapon = weapon_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: WEAPON_FIELD.into(),
        })?;
        let guard = guard_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: GUARD_FIELD.into(),
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
            equip,
            hp,
            pdef,
            attack,
            agi,
            armor1,
            atk,
            item,
            dex,
            armor4,
            weapon,
            guard,
        })
    }
}

impl IntoValue for SystemWords {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        todo!()
    }
}
