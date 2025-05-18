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
}

impl<T: rust_embed::RustEmbed> TranslationAsset for JsonTranslationAsset<T> {
    fn load_translations(&self) -> HashMap<String, serde_json::Value> {
        let mut translations: HashMap<String, serde_json::Map<String, serde_json::Value>> =
            HashMap::new();

        // Collect all files in the embedded folder
        for file in T::iter() {
            let filename = file.as_ref();

            // Try to extract the language code from the filename
            // Accepts: de.json, level_a1_de.json, wortschatz_a1_begruessung_de.json, etc.
            if let Some(lang) = filename.strip_suffix(".json") {
                // Try to match "de" or "level_a1_de"
                let parts: Vec<&str> = lang.split('_').collect();
                let lang_code = if parts.len() == 1 {
                    // de
                    parts[0]
                } else {
                    // level_a1_de -> de
                    parts.last().unwrap()
                };

                // Only process if it's a known language code
                if Language::builtin().iter().any(|l| l.code() == lang_code) {
                    if let Some(content) = T::get(filename) {
                        if let Ok(json) = serde_json::from_slice::<
                            serde_json::Map<String, serde_json::Value>,
                        >(&content.data)
                        {
                            translations
                                .entry(lang_code.to_string())
                                .and_modify(|existing| existing.extend(json.clone()))
                                .or_insert(json);
                        }
                    }
                }
            }
        }

        // Convert to HashMap<String, Value>
        translations
            .into_iter()
            .map(|(k, v)| (k, serde_json::Value::Object(v)))
            .collect()
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

    fn load_yaml(&self) -> Option<HashMap<String, Value>> {
        T::get(&self.filename).and_then(|file| {
            String::from_utf8(file.data.to_vec())
                .ok()
                .and_then(|content| {
                    serde_yaml::from_str::<serde_yaml::Value>(&content)
                        .ok()
                        .and_then(|yaml| {
                            yaml.get("i18n").map(|i18n| {
                                let mut translations = HashMap::new();
                                if let Some(mapping) = i18n.as_mapping() {
                                    for (key, translations_map) in mapping {
                                        let key = key.as_str().unwrap_or_default();
                                        if let Some(trans_map) = translations_map.as_mapping() {
                                            for (lang, trans) in trans_map {
                                                let lang = lang.as_str().unwrap_or_default();
                                                let trans = trans.as_str().unwrap_or_default();

                                                translations
                                                    .entry(lang.to_string())
                                                    .or_insert_with(|| json!({}))
                                                    .as_object_mut()
                                                    .unwrap()
                                                    .insert(
                                                        key.to_string(),
                                                        Value::String(trans.to_string()),
                                                    );
                                            }
                                        }
                                    }
                                }
                                translations
                            })
                        })
                })
        })
    }
}

impl<T: rust_embed::RustEmbed> TranslationAsset for YamlTranslationAsset<T> {
    fn load_translations(&self) -> HashMap<String, Value> {
        self.load_yaml().unwrap_or_default()
    }
}

// Combined asset loader
pub struct CombinedTranslationAsset<T: rust_embed::RustEmbed> {
    json_loader: JsonTranslationAsset<T>,
    yaml_loader: YamlTranslationAsset<T>,
}

impl<T: rust_embed::RustEmbed> CombinedTranslationAsset<T> {
    pub fn new(yaml_filename: &str) -> Self {
        Self {
            json_loader: JsonTranslationAsset::new(),
            yaml_loader: YamlTranslationAsset::new(yaml_filename),
        }
    }
}

impl<T: rust_embed::RustEmbed> TranslationAsset for CombinedTranslationAsset<T> {
    fn load_translations(&self) -> HashMap<String, Value> {
        let mut translations = self.json_loader.load_translations();

        if let Some(yaml_translations) = self.yaml_loader.load_yaml() {
            for (lang, trans) in yaml_translations {
                translations
                    .entry(lang)
                    .and_modify(|e| {
                        if let Some(obj) = e.as_object_mut() {
                            if let Some(new_obj) = trans.as_object() {
                                obj.extend(new_obj.clone());
                            }
                        }
                    })
                    .or_insert(trans);
            }
        }

        translations
    }
}

// Define the asset structure using rust-embed
#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/assets/i18n/"]
pub struct I18nAssets;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_asset_loading() {
        let asset = JsonTranslationAsset::<I18nAssets>::new();
        let translations = asset.load_translations();
        assert!(translations.contains_key("en"));
        assert!(translations.contains_key("de"));
        assert_eq!(translations["en"]["Language"], "Language");
        assert_eq!(translations["de"]["Language"], "Sprache");
    }

    #[test]
    fn test_yaml_asset_loading() {
        let asset = YamlTranslationAsset::<I18nAssets>::new("i18n.yml");
        let translations = asset.load_translations();
        assert!(translations.contains_key("en"));
        assert!(translations.contains_key("de"));
        assert_eq!(translations["en"]["Description"], "Description");
    }

    #[test]
    fn test_combined_asset_loading() {
        let asset = CombinedTranslationAsset::<I18nAssets>::new("i18n.yml");
        let translations = asset.load_translations();

        // Test JSON translations
        assert!(translations.contains_key("en"));
        assert!(translations.contains_key("de"));
        assert_eq!(translations["en"]["Language"], "Language");

        // Test YAML translations
        assert_eq!(translations["en"]["Description"], "Description");

        // Test that both formats are merged correctly
        assert!(translations["en"]
            .as_object()
            .unwrap()
            .contains_key("Language"));
        assert!(translations["en"]
            .as_object()
            .unwrap()
            .contains_key("Description"));
    }

    #[test]
    fn test_missing_files() {
        let asset = CombinedTranslationAsset::<I18nAssets>::new("non_existent.yml");
        let translations = asset.load_translations();

        // Should still load JSON files even if YAML file is missing
        assert!(!translations.is_empty());
        assert!(translations.contains_key("en"));
    }

    #[test]
    fn test_merge_behavior() {
        let asset = CombinedTranslationAsset::<I18nAssets>::new("i18n.yml");
        let translations = asset.load_translations();

        // Test that YAML translations don't override JSON translations
        assert_eq!(translations["en"]["Language"], "Language");

        // Test that both sources contribute to the final translations
        let en_trans = translations["en"].as_object().unwrap();
        assert!(en_trans.contains_key("Language")); // From JSON
        assert!(en_trans.contains_key("Description")); // From YAML
    }
}
