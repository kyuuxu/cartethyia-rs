use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::sync::{LazyLock, Mutex, OnceLock};
use serde::Deserialize;
use crate::LoadDataError;

static FILTERS: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));
static EMPTY: OnceLock<HashMap<String, String>> = OnceLock::new();
static TABLE: OnceLock<HashMap<String, HashMap<String, String>>> = OnceLock::new();

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TextMapData {
    pub id: String,
    pub content: String,
}

pub fn register_filter(filter: String) {
    match TABLE.get() {
        None => { FILTERS.lock().unwrap().insert(filter); }
        Some(_) => {
            // TODO: Implement error if filter is added after textmap was loaded already
        }
    };
}

pub fn load_textmaps(base_path: &str) -> Result<(), LoadDataError> {
    let languages = std::fs::read_dir(base_path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .collect::<Vec<_>>();
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut filters = FILTERS.lock().unwrap();
    for language in languages {
        let lang_id = language.file_name().to_str().unwrap().to_string();
        let file = File::open(&format!("{base_path}/{lang_id}/multi_text/MultiText.json"))?;
        let reader = BufReader::new(file);
        result.insert(
            lang_id,
            serde_json::from_reader::<BufReader<File>, Vec<TextMapData>>(reader)?
                .into_iter()
                .filter(|element| filters.contains(&element.id))
                .map(|element| (element.id, element.content))
                .collect::<HashMap<_, _>>(),
        );
    }
    let _ = TABLE.set(result);
    filters.clear();
    Ok(())
}

pub fn get_textmap(language: i32) -> &'static HashMap<String, String> {
    let (text_code, _audio_code) = get_language_from_i32(language);
    TABLE.get_or_init(|| HashMap::new())
        .get(text_code)
        .unwrap_or(EMPTY.get_or_init(|| HashMap::new()))
}

fn get_language_from_i32(language: i32) -> (&'static str, &'static str) {
    match language {
        0 => ("zh-Hans", "zh"),
        1 => ("en", "en"),
        2 => ("ja", "ja"),
        3 => ("ko", "ko"),
        4 => ("ru", "en"),
        5 => ("zh-Hant", "zh"),
        6 => ("de", "en"),
        7 => ("es", "en"),
        8 => ("pt", "en"),
        9 => ("id", "en"),
        10 => ("fr", "en"),
        11 => ("vi", "en"),
        12 => ("th", "en"),
        _ => ("en", "en"),
    }
}