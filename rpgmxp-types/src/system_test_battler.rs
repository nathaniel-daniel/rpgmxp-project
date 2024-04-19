use ruby_marshal::FromValue;
use ruby_marshal::FromValueContext;
use ruby_marshal::FromValueError;
use ruby_marshal::IntoValue;
use ruby_marshal::IntoValueError;
use ruby_marshal::ObjectValue;
use ruby_marshal::SymbolValue;
use ruby_marshal::Value;
use ruby_marshal::ValueArena;
use ruby_marshal::ValueHandle;

const OBJECT_NAME: &[u8] = b"RPG::System::TestBattler";

const ARMOR4_ID_FIELD: &[u8] = b"@armor4_id";
const ACTOR_ID_FIELD: &[u8] = b"@actor_id";
const WEAPON_ID_FIELD: &[u8] = b"@weapon_id";
const LEVEL_FIELD: &[u8] = b"@level";
const ARMOR3_ID_FIELD: &[u8] = b"@armor3_id";
const ARMOR2_ID_FIELD: &[u8] = b"@armor2_id";
const ARMOR1_ID_FIELD: &[u8] = b"@armor1_id";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SystemTestBattler {
    pub armor4_id: i32,
    pub actor_id: i32,
    pub weapon_id: i32,
    pub level: i32,
    pub armor3_id: i32,
    pub armor2_id: i32,
    pub armor1_id: i32,
}

impl<'a> FromValue<'a> for SystemTestBattler {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let object: &ObjectValue = FromValue::from_value(ctx, value)?;
        let name = object.name();
        let name: &SymbolValue = ctx.from_value(name.into())?;
        let name = name.value();

        if name != OBJECT_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let instance_variables = object.instance_variables();

        let mut armor4_id_field = None;
        let mut actor_id_field = None;
        let mut weapon_id_field = None;
        let mut level_field = None;
        let mut armor3_id_field = None;
        let mut armor2_id_field = None;
        let mut armor1_id_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key: &SymbolValue = ctx.from_value(key.into())?;
            let key = key.value();

            match key {
                ARMOR4_ID_FIELD => {
                    if armor4_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let armor4_id: i32 = ctx.from_value(value)?;
                    armor4_id_field = Some(armor4_id);
                }
                ACTOR_ID_FIELD => {
                    if actor_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let actor_id: i32 = ctx.from_value(value)?;
                    actor_id_field = Some(actor_id);
                }
                WEAPON_ID_FIELD => {
                    if weapon_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let weapon_id: i32 = ctx.from_value(value)?;
                    weapon_id_field = Some(weapon_id);
                }
                LEVEL_FIELD => {
                    if level_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let level: i32 = ctx.from_value(value)?;
                    level_field = Some(level);
                }
                ARMOR3_ID_FIELD => {
                    if armor3_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let armor3_id: i32 = ctx.from_value(value)?;
                    armor3_id_field = Some(armor3_id);
                }
                ARMOR2_ID_FIELD => {
                    if armor2_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let armor2_id: i32 = ctx.from_value(value)?;
                    armor2_id_field = Some(armor2_id);
                }
                ARMOR1_ID_FIELD => {
                    if armor1_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let armor1_id: i32 = ctx.from_value(value)?;
                    armor1_id_field = Some(armor1_id);
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let armor4_id = armor4_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR4_ID_FIELD.into(),
        })?;
        let actor_id = actor_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ACTOR_ID_FIELD.into(),
        })?;
        let weapon_id = weapon_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: WEAPON_ID_FIELD.into(),
        })?;
        let level = level_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: LEVEL_FIELD.into(),
        })?;
        let armor3_id = level_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR3_ID_FIELD.into(),
        })?;
        let armor2_id = level_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR2_ID_FIELD.into(),
        })?;
        let armor1_id = level_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ARMOR1_ID_FIELD.into(),
        })?;

        Ok(Self {
            armor4_id,
            actor_id,
            weapon_id,
            level,
            armor3_id,
            armor2_id,
            armor1_id,
        })
    }
}

impl IntoValue for SystemTestBattler {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let armor4_id_field_key = arena.create_symbol(ARMOR4_ID_FIELD.into());
        let actor_id_field_key = arena.create_symbol(ACTOR_ID_FIELD.into());
        let weapon_id_field_key = arena.create_symbol(WEAPON_ID_FIELD.into());
        let level_field_key = arena.create_symbol(LEVEL_FIELD.into());
        let armor3_id_field_key = arena.create_symbol(ARMOR3_ID_FIELD.into());
        let armor2_id_field_key = arena.create_symbol(ARMOR2_ID_FIELD.into());
        let armor1_id_field_key = arena.create_symbol(ARMOR1_ID_FIELD.into());

        let armor4_id_field_value = self.armor4_id.into_value(arena)?;
        let actor_id_field_value = self.actor_id.into_value(arena)?;
        let weapon_id_field_value = self.weapon_id.into_value(arena)?;
        let level_field_value = self.level.into_value(arena)?;
        let armor3_id_field_value = self.armor3_id.into_value(arena)?;
        let armor2_id_field_value = self.armor2_id.into_value(arena)?;
        let armor1_id_field_value = self.armor1_id.into_value(arena)?;

        let fields = vec![
            (armor4_id_field_key, armor4_id_field_value),
            (actor_id_field_key, actor_id_field_value),
            (weapon_id_field_key, weapon_id_field_value),
            (level_field_key, level_field_value),
            (armor3_id_field_key, armor3_id_field_value),
            (armor2_id_field_key, armor2_id_field_value),
            (armor1_id_field_key, armor1_id_field_value),
        ];

        let object = arena.create_object(object_name, fields);

        Ok(object.into())
    }
}
