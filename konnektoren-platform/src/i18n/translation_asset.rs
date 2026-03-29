use super::error::AssetLoadError;
use super::language::Language;
use serde_json::{Value, json};
use std::collections::HashMap;

pub trait TranslationAsset {
    fn load_translations(&self) -> HashMap<String, Value>;

    /// Like `load_translations`, but surfaces errors instead of silently returning empty data.
    fn try_load_translations(&self) -> Result<HashMap<String, Value>, AssetLoadError> {
        Ok(self.load_translations())
    }
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

        for file in T::iter() {
            let filename = file.as_ref();

            // Accepts: de.json, level_a1_de.json, wortschatz_a1_begruessung_de.json, etc.
            if let Some(lang) = filename.strip_suffix(".json") {
                let parts: Vec<&str> = lang.split('_').collect();
                let lang_code = if parts.len() == 1 {
                    parts[0]
                } else {
                    parts.last().copied().unwrap_or(lang)
                };

                if Language::builtin().iter().any(|l| l.code() == lang_code)
                    && let Some(content) = T::get(filename)
                {
                    match serde_json::from_slice::<serde_json::Map<String, serde_json::Value>>(
                        &content.data,
                    ) {
                        Ok(json) => {
                            translations
                                .entry(lang_code.to_string())
                                .and_modify(|existing| existing.extend(json.clone()))
                                .or_insert(json);
                        }
                        Err(e) => {
                            log::error!("skipping '{}': JSON parse error: {}", filename, e);
                        }
                    }
                }
            }
        }

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

    fn load_yaml(&self) -> Result<HashMap<String, Value>, AssetLoadError> {
        let file = T::get(&self.filename)
            .ok_or_else(|| AssetLoadError::NotFound(self.filename.clone()))?;

        let content =
            String::from_utf8(file.data.to_vec()).map_err(|e| AssetLoadError::Utf8Error {
                file: self.filename.clone(),
                source: e,
            })?;

        let yaml = serde_yaml::from_str::<serde_yaml::Value>(&content).map_err(|e| {
            AssetLoadError::YamlError {
                file: self.filename.clone(),
                source: e,
            }
        })?;

        let i18n = yaml
            .get("i18n")
            .ok_or_else(|| AssetLoadError::InvalidStructure {
                file: self.filename.clone(),
                detail: "missing top-level 'i18n' key".into(),
            })?;

        let mapping = i18n
            .as_mapping()
            .ok_or_else(|| AssetLoadError::InvalidStructure {
                file: self.filename.clone(),
                detail: "'i18n' value is not a mapping".into(),
            })?;

        let mut translations: HashMap<String, Value> = HashMap::new();
        for (key, translations_map) in mapping {
            let key = key.as_str().unwrap_or_default();
            if let Some(trans_map) = translations_map.as_mapping() {
                for (lang, trans) in trans_map {
                    let lang = lang.as_str().unwrap_or_default();
                    let trans = trans.as_str().unwrap_or_default();
                    if let Some(obj) = translations
                        .entry(lang.to_string())
                        .or_insert_with(|| json!({}))
                        .as_object_mut()
                    {
                        obj.insert(key.to_string(), Value::String(trans.to_string()));
                    }
                }
            }
        }

        Ok(translations)
    }
}

impl<T: rust_embed::RustEmbed> TranslationAsset for YamlTranslationAsset<T> {
    fn load_translations(&self) -> HashMap<String, Value> {
        match self.load_yaml() {
            Ok(t) => t,
            Err(AssetLoadError::NotFound(_)) => HashMap::new(),
            Err(e) => {
                log::warn!("{}", e);
                HashMap::new()
            }
        }
    }

    fn try_load_translations(&self) -> Result<HashMap<String, Value>, AssetLoadError> {
        self.load_yaml()
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

        match self.yaml_loader.load_yaml() {
            Ok(yaml_translations) => {
                for (lang, trans) in yaml_translations {
                    translations
                        .entry(lang)
                        .and_modify(|e| {
                            if let Some(obj) = e.as_object_mut()
                                && let Some(new_obj) = trans.as_object()
                            {
                                obj.extend(new_obj.clone());
                            }
                        })
                        .or_insert(trans);
                }
            }
            Err(AssetLoadError::NotFound(_)) => {}
            Err(e) => {
                log::error!("{}", e);
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
    fn test_yaml_try_load_ok() {
        let asset = YamlTranslationAsset::<I18nAssets>::new("i18n.yml");
        let result = asset.try_load_translations();
        assert!(result.is_ok());
        let translations = result.unwrap();
        assert_eq!(translations["en"]["Description"], "Description");
    }

    #[test]
    fn test_yaml_try_load_missing_file() {
        let asset = YamlTranslationAsset::<I18nAssets>::new("non_existent.yml");
        let result = asset.try_load_translations();
        assert!(matches!(result, Err(AssetLoadError::NotFound(_))));
    }

    #[test]
    fn test_combined_asset_loading() {
        let asset = CombinedTranslationAsset::<I18nAssets>::new("i18n.yml");
        let translations = asset.load_translations();

        assert!(translations.contains_key("en"));
        assert!(translations.contains_key("de"));
        assert_eq!(translations["en"]["Language"], "Language");
        assert_eq!(translations["en"]["Description"], "Description");

        let en = translations["en"].as_object().unwrap();
        assert!(en.contains_key("Language"));
        assert!(en.contains_key("Description"));
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

        assert_eq!(translations["en"]["Language"], "Language");

        let en_trans = translations["en"].as_object().unwrap();
        assert!(en_trans.contains_key("Language")); // From JSON
        assert!(en_trans.contains_key("Description")); // From YAML
    }

    #[test]
    fn test_json_asset_with_invalid_json() {
        #[derive(rust_embed::RustEmbed)]
        #[folder = "$CARGO_MANIFEST_DIR/assets/i18n/"]
        struct BadAssets;

        let asset = JsonTranslationAsset::<BadAssets>::new();
        let translations = asset.load_translations();
        assert!(!translations.contains_key("bad"));
    }

    #[test]
    fn test_yaml_asset_with_invalid_yaml() {
        #[derive(rust_embed::RustEmbed)]
        #[folder = "$CARGO_MANIFEST_DIR/assets/i18n/"]
        struct BadAssets;

        let asset = YamlTranslationAsset::<BadAssets>::new("bad.yml");
        let translations = asset.load_translations();
        assert!(translations.is_empty());
    }
}
