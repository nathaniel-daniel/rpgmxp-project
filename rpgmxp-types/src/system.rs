use crate::AudioFile;
use crate::SystemWords;
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

const OBJECT_NAME: &[u8] = b"RPG::System";

const VARIABLES_FIELD: &[u8] = b"@variables";
const CANCEL_SE_FIELD: &[u8] = b"@cancel_se";
const MAGIC_NUMBER_FIELD: &[u8] = b"@magic_number";
const ESCAPE_SE_FIELD: &[u8] = b"@escape_se";
const BATTLE_END_ME_FIELD: &[u8] = b"@battle_end_me";
const START_MAP_ID_FIELD: &[u8] = b"@start_map_id";
const SHOP_SE_FIELD: &[u8] = b"@shop_se";
const GAMEOVER_NAME_FIELD: &[u8] = b"@gameover_name";
const WORDS_FIELD: &[u8] = b"@words";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct System {
    pub variables: Vec<Option<String>>,
    pub cancel_se: AudioFile,
    pub magic_number: i32,
    pub escape_se: AudioFile,
    pub battle_end_me: AudioFile,
    pub start_map_id: i32,
    pub shop_se: AudioFile,
    pub gameover_name: String,
    pub words: SystemWords,
}

impl<'a> FromValue<'a> for System {
    fn from_value(ctx: &FromValueContext<'a>, value: &Value) -> Result<Self, FromValueError> {
        let object: &ObjectValue = FromValue::from_value(ctx, value)?;
        let name = object.name();
        let name: &SymbolValue = ctx.from_value(name.into())?;
        let name = name.value();

        if name != OBJECT_NAME {
            return Err(FromValueError::UnexpectedObjectName { name: name.into() });
        }

        let instance_variables = object.instance_variables();

        let mut variables_field = None;
        let mut cancel_se_field = None;
        let mut magic_number_field = None;
        let mut escape_se_field = None;
        let mut battle_end_me_field = None;
        let mut start_map_id_field = None;
        let mut shop_se_field = None;
        let mut gameover_name_field = None;
        let mut words_field = None;

        for (key, value) in instance_variables.iter().copied() {
            let key: &SymbolValue = ctx.from_value(key.into())?;
            let key = key.value();

            match key {
                VARIABLES_FIELD => {
                    if variables_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let variables: Vec<Option<&StringValue>> = ctx.from_value(value)?;
                    let variables = variables
                        .into_iter()
                        .map(|value| {
                            value
                                .map(|value| {
                                    std::str::from_utf8(value.value())
                                        .map(|value| value.to_string())
                                })
                                .transpose()
                        })
                        .collect::<Result<_, _>>()
                        .map_err(FromValueError::new_other)?;

                    variables_field = Some(variables);
                }
                CANCEL_SE_FIELD => {
                    if cancel_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let cancel_se: AudioFile = ctx.from_value(value)?;
                    cancel_se_field = Some(cancel_se);
                }
                MAGIC_NUMBER_FIELD => {
                    if magic_number_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let magic_number: i32 = ctx.from_value(value)?;
                    magic_number_field = Some(magic_number);
                }
                ESCAPE_SE_FIELD => {
                    if escape_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let escape_se: AudioFile = ctx.from_value(value)?;
                    escape_se_field = Some(escape_se);
                }
                BATTLE_END_ME_FIELD => {
                    if battle_end_me_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let battle_end_me: AudioFile = ctx.from_value(value)?;
                    battle_end_me_field = Some(battle_end_me);
                }
                START_MAP_ID_FIELD => {
                    if start_map_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let start_map_id: i32 = ctx.from_value(value)?;
                    start_map_id_field = Some(start_map_id);
                }
                SHOP_SE_FIELD => {
                    if shop_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let shop_se: AudioFile = ctx.from_value(value)?;
                    shop_se_field = Some(shop_se);
                }
                GAMEOVER_NAME_FIELD => {
                    if gameover_name_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let gameover_name: &StringValue = ctx.from_value(value)?;
                    let gameover_name = std::str::from_utf8(gameover_name.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    gameover_name_field = Some(gameover_name);
                }
                WORDS_FIELD => {
                    if words_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let words: SystemWords = ctx.from_value(value)?;
                    words_field = Some(words);
                }
                _ => {
                    // return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
                }
            }
        }

        let variables = variables_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: VARIABLES_FIELD.into(),
        })?;
        let cancel_se = cancel_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: CANCEL_SE_FIELD.into(),
        })?;
        let magic_number =
            magic_number_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: MAGIC_NUMBER_FIELD.into(),
            })?;
        let escape_se = escape_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ESCAPE_SE_FIELD.into(),
        })?;
        let battle_end_me =
            battle_end_me_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: BATTLE_END_ME_FIELD.into(),
            })?;
        let start_map_id =
            start_map_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: START_MAP_ID_FIELD.into(),
            })?;
        let shop_se = shop_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: SHOP_SE_FIELD.into(),
        })?;
        let gameover_name =
            gameover_name_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: GAMEOVER_NAME_FIELD.into(),
            })?;
        let words = words_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: WORDS_FIELD.into(),
        })?;

        Ok(Self {
            variables,
            cancel_se,
            magic_number,
            escape_se,
            battle_end_me,
            start_map_id,
            shop_se,
            gameover_name,
            words,
        })
    }
}

impl IntoValue for System {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        todo!()
    }
}
