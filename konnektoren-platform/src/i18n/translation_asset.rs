use super::language::Language;
use serde_json::{json, Value};
use std::collections::HashMap;

pub trait TranslationAsset {
    fn load_translations(&self) -> HashMap<String, Value>;
}

// Implementation for embedded JSON files
pub struct JsonTranslationAsset<T: rust_embed::RustEmbed> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: rust_embed::RustEmbed> Default for JsonTranslationAsset<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: rust_embed::RustEmbed> JsonTranslationAsset<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    fn load_json_file(filename: &str) -> Option<Value> {
        T::get(filename).and_then(|file| {
            String::from_utf8(file.data.to_vec())
                .ok()
                .and_then(|content| serde_json::from_str(&content).ok())
        })
    }
}

impl<T: rust_embed::RustEmbed> TranslationAsset for JsonTranslationAsset<T> {
    fn load_translations(&self) -> HashMap<String, Value> {
        let mut translations = HashMap::new();

        for lang in Language::builtin() {
            if let Some(json) = Self::load_json_file(&format!("{}.json", lang.code())) {
                translations.insert(lang.code().to_string(), json);
            }
        }

        translations
    }
}

// Implementation for embedded YAML files
pub struct YamlTranslationAsset<T: rust_embed::RustEmbed> {
    filename: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T: rust_embed::RustEmbed> YamlTranslationAsset<T> {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: rust_embed::RustEmbed> TranslationAsset for YamlTranslationAsset<T> {
    fn load_translations(&self) -> HashMap<String, Value> {
        let mut translations: HashMap<String, Value> = HashMap::new();

        if let Some(file) = T::get(&self.filename) {
            if let Ok(content) = String::from_utf8(file.data.to_vec()) {
                if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                    if let Some(i18n) = yaml.get("i18n") {
                        for (key, translations_map) in i18n.as_mapping().unwrap() {
                            let key = key.as_str().unwrap();
                            for (lang, trans) in translations_map.as_mapping().unwrap() {
                                let lang = lang.as_str().unwrap();
                                let trans = trans.as_str().unwrap();

                                translations
                                    .entry(lang.to_string())
                                    .or_insert_with(|| json!({}))
                                    .as_object_mut()
                                    .unwrap()
                                    .insert(key.to_string(), Value::String(trans.to_string()));
                            }
                        }
                    }
                }
            }
        }

        translations
    }
}

// Define the asset structure using rust-embed
#[derive(rust_embed::RustEmbed)]
#[folder = "assets/i18n/"]
pub struct I18nAssets;
