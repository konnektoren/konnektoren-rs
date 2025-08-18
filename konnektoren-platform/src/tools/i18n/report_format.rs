use super::checker::I18nReport;
use crate::i18n::Language;
use crate::tools::i18n::I18nReportError;

pub trait I18nReportFormatter {
    fn format(&self, report: &I18nReport) -> Result<String, I18nReportError>;
}

pub struct I18nYamlFormatter;
pub struct I18nJsonFormatter;
pub struct I18nHumanFormatter;

impl I18nReportFormatter for I18nYamlFormatter {
    fn format(&self, report: &I18nReport) -> Result<String, I18nReportError> {
        use std::collections::BTreeMap;
        let mut i18n_map: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        for (lang, missing_keys) in &report.missing_translations {
            for key in missing_keys {
                i18n_map
                    .entry(key.clone())
                    .or_default()
                    .insert(lang.clone(), "".to_string());
            }
        }
        let mut yaml = String::from("i18n:\n");
        for (key, langs) in i18n_map {
            yaml.push_str(&format!("  \"{}\":\n", key));
            for (lang, _) in langs {
                yaml.push_str(&format!("    \"{}\": \"\"\n", lang));
            }
        }
        Ok(yaml)
    }
}

impl I18nReportFormatter for I18nJsonFormatter {
    fn format(&self, report: &I18nReport) -> Result<String, I18nReportError> {
        let mut map = serde_json::Map::new();
        for (lang, missing_keys) in &report.missing_translations {
            let mut lang_map = serde_json::Map::new();
            for key in missing_keys {
                lang_map.insert(key.clone(), serde_json::json!(""));
            }
            map.insert(lang.clone(), serde_json::json!(lang_map));
        }
        serde_json::to_string_pretty(&serde_json::json!(map)).map_err(I18nReportError::JsonError)
    }
}

impl I18nReportFormatter for I18nHumanFormatter {
    fn format(&self, report: &I18nReport) -> Result<String, I18nReportError> {
        use std::fmt::Write;
        let mut out = String::new();

        writeln!(out, "\nTranslation Report").map_err(I18nReportError::FmtError)?;
        writeln!(out, "=================").map_err(I18nReportError::FmtError)?;
        writeln!(out, "Source keys found: {}", report.source_keys.len())
            .map_err(I18nReportError::FmtError)?;

        writeln!(out, "\nLanguage Statistics:").map_err(I18nReportError::FmtError)?;
        writeln!(out, "-------------------").map_err(I18nReportError::FmtError)?;
        for lang in crate::i18n::Language::builtin() {
            if let Some(stats) = report.language_stats.get(lang.code()) {
                writeln!(
                    out,
                    "{} ({}): {}/{} keys ({:.1}% coverage)",
                    lang.native_name(),
                    lang.code(),
                    stats.total_keys - stats.missing_keys,
                    stats.total_keys,
                    stats.coverage_percentage
                )
                .map_err(I18nReportError::FmtError)?;
            }
        }

        if !report.missing_translations.is_empty() {
            writeln!(out, "\nMissing Translations:").map_err(I18nReportError::FmtError)?;
            writeln!(out, "-------------------").map_err(I18nReportError::FmtError)?;
            for lang in crate::i18n::Language::builtin() {
                if let Some(missing) = report.missing_translations.get(lang.code()) {
                    if !missing.is_empty() {
                        writeln!(
                            out,
                            "{} ({}) - {} missing:",
                            lang.native_name(),
                            lang.code(),
                            missing.len()
                        )
                        .map_err(I18nReportError::FmtError)?;
                        for key in missing {
                            if let Some(en_trans) = report
                                .translations
                                .get("en")
                                .and_then(|t| t.get(key))
                                .and_then(|v| v.as_str())
                            {
                                writeln!(out, "  - {}: \"{}\"", key, en_trans)
                                    .map_err(I18nReportError::FmtError)?;
                            } else {
                                writeln!(out, "  - {}", key).map_err(I18nReportError::FmtError)?;
                            }
                        }
                    }
                }
            }
        }

        if !report.unused_translations.is_empty() {
            writeln!(out, "\nUnused Translations:").map_err(I18nReportError::FmtError)?;
            writeln!(out, "-------------------").map_err(I18nReportError::FmtError)?;
            for key in &report.unused_translations {
                writeln!(out, "  - {}", key).map_err(I18nReportError::FmtError)?;
                for lang in crate::i18n::Language::builtin() {
                    if let Some(trans) = report
                        .translations
                        .get(lang.code())
                        .and_then(|t| t.get(key))
                        .and_then(|v| v.as_str())
                    {
                        writeln!(
                            out,
                            "    {} ({}): \"{}\"",
                            lang.native_name(),
                            lang.code(),
                            trans
                        )
                        .map_err(I18nReportError::FmtError)?;
                    }
                }
            }
        }

        writeln!(out, "\nSummary:").map_err(I18nReportError::FmtError)?;
        writeln!(out, "--------").map_err(I18nReportError::FmtError)?;
        writeln!(out, "Total source keys: {}", report.source_keys.len())
            .map_err(I18nReportError::FmtError)?;
        writeln!(
            out,
            "Languages: {}",
            crate::i18n::Language::builtin()
                .iter()
                .map(|l| l.code())
                .collect::<Vec<_>>()
                .join(", ")
        )
        .map_err(I18nReportError::FmtError)?;

        let total_missing: usize = report.missing_translations.values().map(|v| v.len()).sum();
        writeln!(out, "Total missing translations: {}", total_missing)
            .map_err(I18nReportError::FmtError)?;

        writeln!(
            out,
            "Overall status: {}",
            if report.has_errors {
                "❌ Missing translations"
            } else {
                "✅ All translations complete"
            }
        )
        .map_err(I18nReportError::FmtError)?;

        Ok(out)
    }
}
