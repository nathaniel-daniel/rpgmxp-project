use crate::AudioFile;
use crate::SystemTestBattler;
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
const SWITCHES_FIELD: &[u8] = b"@switches";
const DECISION_SE_FIELD: &[u8] = b"@decision_se";
const EDIT_MAP_ID_FIELD: &[u8] = b"@edit_map_id";
const BATTLE_START_SE_FIELD: &[u8] = b"@battle_start_se";
const BATTLE_BGM_FIELD: &[u8] = b"@battle_bgm";
const TEST_TROOP_ID_FIELD: &[u8] = b"@test_troop_id";
const EQUIP_SE_FIELD: &[u8] = b"@equip_se";
const TITLE_NAME_FIELD: &[u8] = b"@title_name";
const ENEMY_COLLAPSE_SE_FIELD: &[u8] = b"@enemy_collapse_se";
const CURSOR_SE_FIELD: &[u8] = b"@cursor_se";
const ELEMENTS_FIELD: &[u8] = b"@elements";
const UNDERSCORE_FIELD: &[u8] = b"@_";
const START_Y_FIELD: &[u8] = b"@start_y";
const BATTLER_HUE_FIELD: &[u8] = b"@battler_hue";
const LOAD_SE_FIELD: &[u8] = b"@load_se";
const TITLE_BGM_FIELD: &[u8] = b"@title_bgm";
const BUZZER_SE_FIELD: &[u8] = b"@buzzer_se";
const WINDOWSKIN_NAME_FIELD: &[u8] = b"@windowskin_name";
const TEST_BATTLERS_FIELD: &[u8] = b"@test_battlers";
const BATTLEBACK_NAME_FIELD: &[u8] = b"@battleback_name";
const PARTY_MEMBERS_FIELD: &[u8] = b"@party_members";
const ACTOR_COLLAPSE_SE_FIELD: &[u8] = b"@actor_collapse_se";
const GAMEOVER_ME_FIELD: &[u8] = b"@gameover_me";
const BATTLER_NAME_FIELD: &[u8] = b"@battler_name";
const SAVE_SE_FIELD: &[u8] = b"@save_se";
const BATTLE_TRANSITION_FIELD: &[u8] = b"@battle_transition";
const START_X_FIELD: &[u8] = b"@start_x";

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
    pub switches: Vec<Option<String>>,
    pub decision_se: AudioFile,
    pub edit_map_id: i32,
    pub battle_start_se: AudioFile,
    pub battle_bgm: AudioFile,
    pub test_troop_id: i32,
    pub equip_se: AudioFile,
    pub title_name: String,
    pub enemy_collapse_se: AudioFile,
    pub cursor_se: AudioFile,
    pub elements: Vec<String>,
    pub underscore: i32,
    pub start_y: i32,
    pub battler_hue: i32,
    pub load_se: AudioFile,
    pub title_bgm: AudioFile,
    pub buzzer_se: AudioFile,
    pub windowskin_name: String,
    pub test_battlers: Vec<SystemTestBattler>,
    pub battleback_name: String,
    pub party_members: Vec<i32>,
    pub actor_collapse_se: AudioFile,
    pub gameover_me: AudioFile,
    pub battler_name: String,
    pub save_se: AudioFile,
    pub battle_transition: String,
    pub start_x: i32,
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
        let mut switches_field = None;
        let mut decision_se_field = None;
        let mut edit_map_id_field = None;
        let mut battle_start_se_field = None;
        let mut battle_bgm_field = None;
        let mut test_troop_id_field = None;
        let mut equip_se_field = None;
        let mut title_name_field = None;
        let mut enemy_collapse_se_field = None;
        let mut cursor_se_field = None;
        let mut elements_field = None;
        let mut underscore_field = None;
        let mut start_y_field = None;
        let mut battler_hue_field = None;
        let mut load_se_field = None;
        let mut title_bgm_field = None;
        let mut buzzer_se_field = None;
        let mut windowskin_name_field = None;
        let mut test_battlers_field = None;
        let mut battleback_name_field = None;
        let mut party_members_field = None;
        let mut actor_collapse_se_field = None;
        let mut gameover_me_field = None;
        let mut battler_name_field = None;
        let mut save_se_field = None;
        let mut battle_transition_field = None;
        let mut start_x_field = None;

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
                SWITCHES_FIELD => {
                    if switches_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let switches: Vec<Option<&StringValue>> = ctx.from_value(value)?;
                    let switches = switches
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

                    switches_field = Some(switches);
                }
                DECISION_SE_FIELD => {
                    if decision_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let decision_se: AudioFile = ctx.from_value(value)?;
                    decision_se_field = Some(decision_se);
                }
                EDIT_MAP_ID_FIELD => {
                    if edit_map_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let edit_map_id: i32 = ctx.from_value(value)?;
                    edit_map_id_field = Some(edit_map_id);
                }
                BATTLE_START_SE_FIELD => {
                    if battle_start_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let battle_start_se: AudioFile = ctx.from_value(value)?;
                    battle_start_se_field = Some(battle_start_se);
                }
                BATTLE_BGM_FIELD => {
                    if battle_bgm_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let battle_bgm: AudioFile = ctx.from_value(value)?;
                    battle_bgm_field = Some(battle_bgm);
                }
                TEST_TROOP_ID_FIELD => {
                    if test_troop_id_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let test_troop_id: i32 = ctx.from_value(value)?;
                    test_troop_id_field = Some(test_troop_id);
                }
                EQUIP_SE_FIELD => {
                    if equip_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let equip_se: AudioFile = ctx.from_value(value)?;
                    equip_se_field = Some(equip_se);
                }
                TITLE_NAME_FIELD => {
                    if title_name_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let title_name: &StringValue = ctx.from_value(value)?;
                    let title_name = std::str::from_utf8(title_name.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    title_name_field = Some(title_name);
                }
                ENEMY_COLLAPSE_SE_FIELD => {
                    if enemy_collapse_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let enemy_collapse_se: AudioFile = ctx.from_value(value)?;
                    enemy_collapse_se_field = Some(enemy_collapse_se);
                }
                CURSOR_SE_FIELD => {
                    if cursor_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let cursor_se: AudioFile = ctx.from_value(value)?;
                    cursor_se_field = Some(cursor_se);
                }
                ELEMENTS_FIELD => {
                    if elements_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let elements: Vec<&StringValue> = ctx.from_value(value)?;
                    let elements = elements
                        .into_iter()
                        .map(|value| {
                            std::str::from_utf8(value.value()).map(|value| value.to_string())
                        })
                        .collect::<Result<_, _>>()
                        .map_err(FromValueError::new_other)?;

                    elements_field = Some(elements);
                }
                UNDERSCORE_FIELD => {
                    if underscore_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let underscore: i32 = ctx.from_value(value)?;
                    underscore_field = Some(underscore);
                }
                START_Y_FIELD => {
                    if start_y_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let start_y: i32 = ctx.from_value(value)?;
                    start_y_field = Some(start_y);
                }
                BATTLER_HUE_FIELD => {
                    if battler_hue_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let battler_hue: i32 = ctx.from_value(value)?;
                    battler_hue_field = Some(battler_hue);
                }
                LOAD_SE_FIELD => {
                    if load_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let load_se: AudioFile = ctx.from_value(value)?;
                    load_se_field = Some(load_se);
                }
                TITLE_BGM_FIELD => {
                    if title_bgm_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let title_bgm: AudioFile = ctx.from_value(value)?;
                    title_bgm_field = Some(title_bgm);
                }
                BUZZER_SE_FIELD => {
                    if buzzer_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let buzzer_se: AudioFile = ctx.from_value(value)?;
                    buzzer_se_field = Some(buzzer_se);
                }
                WINDOWSKIN_NAME_FIELD => {
                    if windowskin_name_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let windowskin_name: &StringValue = ctx.from_value(value)?;
                    let windowskin_name = std::str::from_utf8(windowskin_name.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    windowskin_name_field = Some(windowskin_name);
                }
                TEST_BATTLERS_FIELD => {
                    if test_battlers_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let test_battlers: Vec<SystemTestBattler> = ctx.from_value(value)?;
                    test_battlers_field = Some(test_battlers);
                }
                BATTLEBACK_NAME_FIELD => {
                    if battleback_name_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let battleback_name: &StringValue = ctx.from_value(value)?;
                    let battleback_name = std::str::from_utf8(battleback_name.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    battleback_name_field = Some(battleback_name);
                }
                PARTY_MEMBERS_FIELD => {
                    if party_members_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let party_members: Vec<i32> = ctx.from_value(value)?;
                    party_members_field = Some(party_members);
                }
                ACTOR_COLLAPSE_SE_FIELD => {
                    if actor_collapse_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let actor_collapse_se: AudioFile = ctx.from_value(value)?;
                    actor_collapse_se_field = Some(actor_collapse_se);
                }
                GAMEOVER_ME_FIELD => {
                    if gameover_me_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let gameover_me: AudioFile = ctx.from_value(value)?;
                    gameover_me_field = Some(gameover_me);
                }
                BATTLER_NAME_FIELD => {
                    if battler_name_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let battler_name: &StringValue = ctx.from_value(value)?;
                    let battler_name = std::str::from_utf8(battler_name.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    battler_name_field = Some(battler_name);
                }
                SAVE_SE_FIELD => {
                    if save_se_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let save_se: AudioFile = ctx.from_value(value)?;
                    save_se_field = Some(save_se);
                }
                BATTLE_TRANSITION_FIELD => {
                    if battle_transition_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let battle_transition: &StringValue = ctx.from_value(value)?;
                    let battle_transition = std::str::from_utf8(battle_transition.value())
                        .map_err(FromValueError::new_other)?
                        .to_string();
                    battle_transition_field = Some(battle_transition);
                }
                START_X_FIELD => {
                    if start_x_field.is_some() {
                        return Err(FromValueError::DuplicateInstanceVariable { name: key.into() });
                    }

                    let start_x: i32 = ctx.from_value(value)?;
                    start_x_field = Some(start_x);
                }
                _ => {
                    return Err(FromValueError::UnknownInstanceVariable { name: key.into() });
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
        let switches = switches_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: SWITCHES_FIELD.into(),
        })?;
        let decision_se =
            decision_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: DECISION_SE_FIELD.into(),
            })?;
        let edit_map_id =
            edit_map_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: EDIT_MAP_ID_FIELD.into(),
            })?;
        let battle_start_se =
            battle_start_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: BATTLE_START_SE_FIELD.into(),
            })?;
        let battle_bgm =
            battle_bgm_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: BATTLE_BGM_FIELD.into(),
            })?;
        let test_troop_id =
            test_troop_id_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: TEST_TROOP_ID_FIELD.into(),
            })?;
        let equip_se = equip_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: EQUIP_SE_FIELD.into(),
        })?;
        let title_name =
            title_name_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: TITLE_NAME_FIELD.into(),
            })?;
        let enemy_collapse_se =
            enemy_collapse_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: ENEMY_COLLAPSE_SE_FIELD.into(),
            })?;
        let cursor_se = cursor_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: CURSOR_SE_FIELD.into(),
        })?;
        let elements = elements_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: ELEMENTS_FIELD.into(),
        })?;
        let underscore =
            underscore_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: UNDERSCORE_FIELD.into(),
            })?;
        let start_y = start_y_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: START_Y_FIELD.into(),
        })?;
        let battler_hue =
            battler_hue_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: BATTLER_HUE_FIELD.into(),
            })?;
        let load_se = load_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: LOAD_SE_FIELD.into(),
        })?;
        let title_bgm = title_bgm_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: TITLE_BGM_FIELD.into(),
        })?;
        let buzzer_se = buzzer_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: BUZZER_SE_FIELD.into(),
        })?;
        let windowskin_name =
            windowskin_name_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: WINDOWSKIN_NAME_FIELD.into(),
            })?;
        let test_battlers =
            test_battlers_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: TEST_BATTLERS_FIELD.into(),
            })?;
        let battleback_name =
            battleback_name_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: BATTLEBACK_NAME_FIELD.into(),
            })?;
        let party_members =
            party_members_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: PARTY_MEMBERS_FIELD.into(),
            })?;
        let actor_collapse_se =
            actor_collapse_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: ACTOR_COLLAPSE_SE_FIELD.into(),
            })?;
        let gameover_me =
            gameover_me_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: GAMEOVER_ME_FIELD.into(),
            })?;
        let battler_name =
            battler_name_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: BATTLER_NAME_FIELD.into(),
            })?;
        let save_se = save_se_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: SAVE_SE_FIELD.into(),
        })?;
        let battle_transition =
            battle_transition_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
                name: BATTLE_TRANSITION_FIELD.into(),
            })?;
        let start_x = start_x_field.ok_or_else(|| FromValueError::MissingInstanceVariable {
            name: START_X_FIELD.into(),
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
            switches,
            decision_se,
            edit_map_id,
            battle_start_se,
            battle_bgm,
            test_troop_id,
            equip_se,
            title_name,
            enemy_collapse_se,
            cursor_se,
            elements,
            underscore,
            start_y,
            battler_hue,
            load_se,
            title_bgm,
            buzzer_se,
            windowskin_name,
            test_battlers,
            battleback_name,
            party_members,
            actor_collapse_se,
            gameover_me,
            battler_name,
            save_se,
            battle_transition,
            start_x,
        })
    }
}

impl IntoValue for System {
    fn into_value(self, arena: &mut ValueArena) -> Result<ValueHandle, IntoValueError> {
        let object_name = arena.create_symbol(OBJECT_NAME.into());

        let variables_field_key = arena.create_symbol(VARIABLES_FIELD.into());
        let cancel_se_field_key = arena.create_symbol(CANCEL_SE_FIELD.into());
        let magic_number_field_key = arena.create_symbol(MAGIC_NUMBER_FIELD.into());
        let escape_se_field_key = arena.create_symbol(ESCAPE_SE_FIELD.into());
        let battle_end_me_field_key = arena.create_symbol(BATTLE_END_ME_FIELD.into());
        let start_map_id_field_key = arena.create_symbol(START_MAP_ID_FIELD.into());
        let shop_se_field_key = arena.create_symbol(SHOP_SE_FIELD.into());
        let gameover_name_field_key = arena.create_symbol(GAMEOVER_NAME_FIELD.into());
        let words_field_key = arena.create_symbol(WORDS_FIELD.into());
        let switches_field_key = arena.create_symbol(SWITCHES_FIELD.into());
        let decision_se_field_key = arena.create_symbol(DECISION_SE_FIELD.into());
        let edit_map_id_field_key = arena.create_symbol(EDIT_MAP_ID_FIELD.into());
        let battle_start_se_field_key = arena.create_symbol(BATTLE_START_SE_FIELD.into());
        let battle_bgm_field_key = arena.create_symbol(BATTLE_BGM_FIELD.into());
        let test_troop_id_field_key = arena.create_symbol(TEST_TROOP_ID_FIELD.into());
        let equip_se_field_key = arena.create_symbol(EQUIP_SE_FIELD.into());
        let title_name_field_key = arena.create_symbol(TITLE_NAME_FIELD.into());
        let enemy_collapse_se_field_key = arena.create_symbol(ENEMY_COLLAPSE_SE_FIELD.into());
        let cursor_se_field_key = arena.create_symbol(CURSOR_SE_FIELD.into());
        let elements_field_key = arena.create_symbol(ELEMENTS_FIELD.into());
        let underscore_field_key = arena.create_symbol(UNDERSCORE_FIELD.into());
        let start_y_field_key = arena.create_symbol(START_Y_FIELD.into());
        let battler_hue_field_key = arena.create_symbol(BATTLER_HUE_FIELD.into());
        let load_se_field_key = arena.create_symbol(LOAD_SE_FIELD.into());
        let title_bgm_field_key = arena.create_symbol(TITLE_BGM_FIELD.into());
        let buzzer_se_field_key = arena.create_symbol(BUZZER_SE_FIELD.into());
        let windowskin_name_field_key = arena.create_symbol(WINDOWSKIN_NAME_FIELD.into());
        let test_battlers_field_key = arena.create_symbol(TEST_BATTLERS_FIELD.into());
        let battleback_name_field_key = arena.create_symbol(BATTLEBACK_NAME_FIELD.into());
        let party_members_field_key = arena.create_symbol(PARTY_MEMBERS_FIELD.into());
        let actor_collapse_se_field_key = arena.create_symbol(ACTOR_COLLAPSE_SE_FIELD.into());
        let gameover_me_field_key = arena.create_symbol(GAMEOVER_ME_FIELD.into());
        let battler_name_field_key = arena.create_symbol(BATTLER_NAME_FIELD.into());
        let save_se_field_key = arena.create_symbol(SAVE_SE_FIELD.into());
        let battle_transition_field_key = arena.create_symbol(BATTLE_TRANSITION_FIELD.into());
        let start_x_field_key = arena.create_symbol(START_X_FIELD.into());

        let variables_field_value = {
            let mut variables = Vec::with_capacity(self.variables.len());
            for variable in self.variables {
                let handle = match variable {
                    Some(variable) => arena.create_string(variable.into()).into(),
                    None => arena.create_nil().into(),
                };
                variables.push(handle);
            }

            arena.create_array(variables).into()
        };
        let cancel_se_field_value = self.cancel_se.into_value(arena)?;
        let magic_number_field_value = self.magic_number.into_value(arena)?;
        let escape_se_field_value = self.escape_se.into_value(arena)?;
        let battle_end_me_field_value = self.battle_end_me.into_value(arena)?;
        let start_map_id_field_value = self.start_map_id.into_value(arena)?;
        let shop_se_field_value = self.shop_se.into_value(arena)?;
        let gameover_name_field_value = arena.create_string(self.gameover_name.into()).into();
        let words_field_value = self.words.into_value(arena)?;
        let switches_field_value = {
            let mut switches = Vec::with_capacity(self.switches.len());
            for switch in self.switches {
                let handle = match switch {
                    Some(switch) => arena.create_string(switch.into()).into(),
                    None => arena.create_nil().into(),
                };
                switches.push(handle);
            }

            arena.create_array(switches).into()
        };
        let decision_se_field_value = self.decision_se.into_value(arena)?;
        let edit_map_id_field_value = self.edit_map_id.into_value(arena)?;
        let battle_start_se_field_value = self.battle_start_se.into_value(arena)?;
        let battle_bgm_field_value = self.battle_bgm.into_value(arena)?;
        let test_troop_id_field_value = self.test_troop_id.into_value(arena)?;
        let equip_se_field_value = self.equip_se.into_value(arena)?;
        let title_name_field_value = arena.create_string(self.title_name.into()).into();
        let enemy_collapse_se_field_value = self.enemy_collapse_se.into_value(arena)?;
        let cursor_se_field_value = self.cursor_se.into_value(arena)?;
        let elements_field_value = {
            let mut elements = Vec::with_capacity(self.elements.len());
            for element in self.elements {
                let handle = arena.create_string(element.into()).into();
                elements.push(handle);
            }
            arena.create_array(elements).into()
        };
        let underscore_field_value = self.underscore.into_value(arena)?;
        let start_y_field_value = self.start_y.into_value(arena)?;
        let battler_hue_field_value = self.battler_hue.into_value(arena)?;
        let load_se_field_value = self.load_se.into_value(arena)?;
        let title_bgm_field_value = self.title_bgm.into_value(arena)?;
        let buzzer_se_field_value = self.buzzer_se.into_value(arena)?;
        let windowskin_name_field_value = arena.create_string(self.windowskin_name.into()).into();
        let test_battlers_field_value = self.test_battlers.into_value(arena)?;
        let battleback_name_field_value = arena.create_string(self.battleback_name.into()).into();
        let party_members_field_value = self.party_members.into_value(arena)?;
        let actor_collapse_se_field_value = self.actor_collapse_se.into_value(arena)?;
        let gameover_me_field_value = self.gameover_me.into_value(arena)?;
        let battler_name_field_value = arena.create_string(self.battler_name.into()).into();
        let save_se_field_value = self.save_se.into_value(arena)?;
        let battle_transition_field_value =
            arena.create_string(self.battle_transition.into()).into();
        let start_x_field_value = self.start_x.into_value(arena)?;

        let fields = vec![
            (variables_field_key, variables_field_value),
            (cancel_se_field_key, cancel_se_field_value),
            (magic_number_field_key, magic_number_field_value),
            (escape_se_field_key, escape_se_field_value),
            (battle_end_me_field_key, battle_end_me_field_value),
            (start_map_id_field_key, start_map_id_field_value),
            (shop_se_field_key, shop_se_field_value),
            (gameover_name_field_key, gameover_name_field_value),
            (words_field_key, words_field_value),
            (switches_field_key, switches_field_value),
            (decision_se_field_key, decision_se_field_value),
            (edit_map_id_field_key, edit_map_id_field_value),
            (battle_start_se_field_key, battle_start_se_field_value),
            (battle_bgm_field_key, battle_bgm_field_value),
            (test_troop_id_field_key, test_troop_id_field_value),
            (equip_se_field_key, equip_se_field_value),
            (title_name_field_key, title_name_field_value),
            (enemy_collapse_se_field_key, enemy_collapse_se_field_value),
            (cursor_se_field_key, cursor_se_field_value),
            (elements_field_key, elements_field_value),
            (underscore_field_key, underscore_field_value),
            (start_y_field_key, start_y_field_value),
            (battler_hue_field_key, battler_hue_field_value),
            (load_se_field_key, load_se_field_value),
            (title_bgm_field_key, title_bgm_field_value),
            (buzzer_se_field_key, buzzer_se_field_value),
            (windowskin_name_field_key, windowskin_name_field_value),
            (test_battlers_field_key, test_battlers_field_value),
            (battleback_name_field_key, battleback_name_field_value),
            (party_members_field_key, party_members_field_value),
            (actor_collapse_se_field_key, actor_collapse_se_field_value),
            (gameover_me_field_key, gameover_me_field_value),
            (battler_name_field_key, battler_name_field_value),
            (save_se_field_key, save_se_field_value),
            (battle_transition_field_key, battle_transition_field_value),
            (start_x_field_key, start_x_field_value),
        ];

        let object = arena.create_object(object_name, fields);

        Ok(object.into())
    }
}
