use std::fs::File;
use std::io::BufReader;

use paste::paste;

pub use level_entity_config::LevelEntityConfigData;
pub use misc_data::*;

pub mod node_data;
pub mod pb_components;
pub mod text_map_data;

mod misc_data;

#[derive(thiserror::Error, Debug)]
pub enum LoadDataError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse json: {0}")]
    Json(#[from] serde_json::Error),
}

macro_rules! json_data {
    ($($table_type:ident;)*) => {
        $(paste! {
            mod [<$table_type:snake>];
            pub use [<$table_type:snake>]::[<$table_type Data>];
        })*

        $(paste! {
            pub mod [<$table_type:snake _data>] {
                use std::sync::OnceLock;
                type Data = super::[<$table_type Data>];
                pub(crate) static TABLE: OnceLock<Vec<Data>> = OnceLock::new();

                pub fn iter() -> std::slice::Iter<'static, Data> {
                    TABLE.get().unwrap().iter()
                }
            }
        })*

        fn load_json_data(base_path: &str) -> Result<(), LoadDataError> {
            $(paste! {
                let path = format!("{}/{}.json", base_path, stringify!($table_type));
                tracing::debug!("Loading data started: {path}");
                let file = File::open(&path)?;
                let reader = BufReader::new(file);
                let _ = [<$table_type:snake _data>]::TABLE.set(serde_json::from_reader(reader)?);
                tracing::info!("Loading data finished: {path}");
            })*

            Ok(())
        }
    };
}

macro_rules! json_hash_table_data {
    ($($table_type:ident, $key_param:expr, $key_type:ty;)*) => {
        $(paste! {
            mod [<$table_type:snake>];
            pub use [<$table_type:snake>]::[<$table_type Data>];
        })*

        $(paste! {
            pub mod [<$table_type:snake _data>] {
                use std::collections::HashMap;
                use std::sync::OnceLock;

                pub(crate) type Data = super::[<$table_type Data>];
                pub(crate) static TABLE: OnceLock<HashMap<$key_type, Data>> = OnceLock::new();

                pub fn iter() -> std::collections::hash_map::Iter<'static, $key_type, Data> {
                    TABLE.get().unwrap().iter()
                }

                pub fn get(k: &$key_type) -> Option<&'static Data> {
                    TABLE.get().unwrap().get(k)
                }
            }
        })*

        fn load_json_hash_table_data(base_path: &str) -> Result<(), LoadDataError> {
            $(paste! {
                let path = format!("{}/{}.json", base_path, stringify!($table_type));
                tracing::debug!("Loading data started: {path}");
                let file = File::open(&path)?;
                let reader = BufReader::new(file);
                // Key.clone() is required for String keys, for other types like ints, it doesn't
                // have any effect since clone is *value
                let _ = [<$table_type:snake _data>]::TABLE.set(
                    serde_json::from_reader::<BufReader<File>, Vec<[<$table_type:snake _data>]::Data>>(reader)?
                        .into_iter()
                        .map(|element| (element.$key_param.clone(), element))
                        .collect::<std::collections::HashMap<_, _>>()
                );
                tracing::info!("Loading data finished: {path}");
            })*

            Ok(())
        }
    };
}

pub fn load_all_json_data(base_path: &str) -> Result<(), LoadDataError> {
    load_json_data(base_path)?;
    load_json_hash_table_data(base_path)?;
    load_json_entity_level_config_data(base_path)?;
    Ok(())
}

json_data! {
    Achievement;
    AdventureTask;
    Area;
    BaseProperty;
    CalabashDevelopReward;
    CalabashLevel;
    Damage;
    DungeonDetection;
    ExchangeReward;
    ExchangeShared;
    ExploreProgress;
    ExploreTools;
    FavorGoods;
    FavorLevel;
    FavorStory;
    FavorWord;
    FlySkinConfig;
    ForgeFormula;
    FunctionCondition;
    Gacha;
    GachaPool;
    GachaViewInfo;
    GuideGroup;
    GuideTutorial;
    InstanceDungeon;
    ItemExchangeContent;
    // LevelPlayData;
    LevelPlayNodeData;
    LivenessTask;
    LordGym;
    MonsterDetection;
    MonsterPropertyGrowth;
    Motion;
    QuestNodeData;
    ResonanceAmplification;
    ResonantChain;
    RoleBreach;
    RoleInfo;
    RoleLevelConsume;
    RolePropertyGrowth;
    RoleSkin;
    SilentAreaDetection;
    SynthesisFormula;
    Teleporter;
    WeaponBreach;
    WeaponConf;
    WeaponExpItem;
    WeaponLevel;
    WeaponPropertyGrowth;
    WeaponReson;
    WeaponSkin;
}

json_hash_table_data! {
    AiBase, id, i32;
    AiStateMachineConfig, id, String;
    BlueprintConfig, blueprint_type, String;
    DragonPool, id, i32;
    DropPackage, id, i32;
    RoleExpItem, id, i32;
    TemplateConfig, blueprint_type, String;
}

mod level_entity_config;

pub mod level_entity_config_data {
    use std::collections::HashMap;
    use std::sync::OnceLock;

    pub(crate) type Data = super::LevelEntityConfigData;
    pub(crate) static TABLE: OnceLock<HashMap<String, Data>> = OnceLock::new();

    pub fn iter() -> std::collections::hash_map::Iter<'static, String, Data> {
        TABLE.get().unwrap().iter()
    }

    pub fn get(map_id: i32, entity_id: i64) -> Option<&'static Data> {
        TABLE
            .get()
            .unwrap()
            .get(&create_key_internal(map_id, entity_id))
    }

    #[inline(always)]
    pub fn create_key(element: &Data) -> String {
        create_key_internal(element.map_id, element.entity_id)
    }

    #[inline(always)]
    fn create_key_internal(map_id: i32, entity_id: i64) -> String {
        format!("{}_{}", map_id, entity_id)
    }
}

fn load_json_entity_level_config_data(base_path: &str) -> Result<(), LoadDataError> {
    let path = format!("{}/LevelEntityConfig.json", base_path);
    tracing::debug!("Loading data started: {path}");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let _ = level_entity_config_data::TABLE.set(
        serde_json::from_reader::<BufReader<File>, Vec<level_entity_config_data::Data>>(reader)?
            .into_iter()
            .map(|element| (level_entity_config_data::create_key(&element), element))
            .collect::<std::collections::HashMap<_, _>>(),
    );
    tracing::info!("Loading data finished: {path}");

    Ok(())
}
