use crate::{challenges::Custom, prelude::ChallengeConfig};

use super::package_metadata::PackageMetadata;
use std::collections::HashMap;

pub struct Package {
    pub metadata: PackageMetadata,
    pub files: HashMap<String, Vec<u8>>,
}

impl Package {
    pub fn new(metadata: PackageMetadata) -> Self {
        Self {
            metadata,
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, path: String, content: Vec<u8>) {
        self.files.insert(path, content);
    }

    pub fn get_file(&self, path: &str) -> Option<&Vec<u8>> {
        self.files.get(path)
    }

    pub fn get_file_as_string(&self, path: &str) -> Option<String> {
        self.files
            .get(path)
            .map(|v| String::from_utf8_lossy(v).to_string())
    }

    /// Get the challenge configuration config.yml from the package
    pub fn get_challenge_config(&self) -> Option<ChallengeConfig> {
        Some(self.metadata.config.clone())
    }

    pub fn get_custom_challenge(&self) -> Option<Custom> {
        Some(self.metadata.custom.clone())
    }

    pub fn get_html_file(&self) -> Option<String> {
        let custom_challenge = self.get_custom_challenge()?;
        let filename = custom_challenge.html;
        self.get_file_as_string(&filename)
    }

    pub fn get_css_file(&self) -> Option<String> {
        let custom_challenge = self.get_custom_challenge()?;
        let filename = custom_challenge.css;
        self.get_file_as_string(&filename)
    }

    pub fn get_js_file(&self) -> Option<String> {
        let custom_challenge = self.get_custom_challenge()?;
        let filename = custom_challenge.js;
        self.get_file_as_string(&filename)
    }

    pub fn get_results_file(&self) -> Option<String> {
        let custom_challenge = self.get_custom_challenge()?;
        let filename = custom_challenge.results_html;
        match filename {
            Some(filename) => self.get_file_as_string(&filename),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::Custom;

    #[test]
    fn test_package() {
        let metadata = PackageMetadata::default();
        let mut package = Package::new(metadata);
        package.add_file("test.txt".to_string(), b"test".to_vec());
        assert_eq!(package.get_file("test.txt"), Some(&b"test".to_vec()));
        assert_eq!(
            package.get_file_as_string("test.txt"),
            Some("test".to_string())
        );
    }

    #[test]
    fn test_challenge_config() {
        let mut metadata = PackageMetadata::default();
        let config = ChallengeConfig {
            tasks: 3,
            ..Default::default()
        };
        metadata.config = config.clone();
        let mut package = Package::new(metadata);

        package.add_file(
            "config.yml".to_string(),
            serde_yaml::to_string(&config).unwrap().into_bytes(),
        );
        assert_eq!(
            package.get_challenge_config(),
            Some(ChallengeConfig {
                tasks: 3,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_custom_challenge() {
        let mut metadata = PackageMetadata::default();
        let custom = Custom {
            id: "custom_challenge".to_string(),
            name: "Custom Challenge".to_string(),
            description: "This is a custom challenge".to_string(),
            ..Default::default()
        };
        metadata.custom = custom.clone();
        let mut package = Package::new(metadata);

        package.add_file(
            "challenge.yml".to_string(),
            serde_yaml::to_string(&custom).unwrap().into_bytes(),
        );
        assert_eq!(
            package.get_custom_challenge(),
            Some(Custom {
                id: "custom_challenge".to_string(),
                name: "Custom Challenge".to_string(),
                description: "This is a custom challenge".to_string(),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_html_file() {
        let mut metadata = PackageMetadata::default();
        let custom = Custom {
            id: "custom_challenge".to_string(),
            name: "Custom Challenge".to_string(),
            description: "This is a custom challenge".to_string(),
            html: "index.html".to_string(),
            ..Default::default()
        };
        metadata.custom = custom.clone();
        let mut package = Package::new(metadata);

        package.add_file(
            "challenge.yml".to_string(),
            serde_yaml::to_string(&custom).unwrap().into_bytes(),
        );
        package.add_file("index.html".to_string(), b"<html></html>".to_vec());
        assert_eq!(package.get_html_file(), Some("<html></html>".to_string()));
    }

    #[test]
    fn test_css_file() {
        let mut metadata = PackageMetadata::default();
        let custom = Custom {
            id: "custom_challenge".to_string(),
            name: "Custom Challenge".to_string(),
            description: "This is a custom challenge".to_string(),
            css: "style.css".to_string(),
            ..Default::default()
        };
        metadata.custom = custom.clone();
        let mut package = Package::new(metadata);

        package.add_file(
            "challenge.yml".to_string(),
            serde_yaml::to_string(&custom).unwrap().into_bytes(),
        );
        package.add_file("style.css".to_string(), b"body { color: red; }".to_vec());
        assert_eq!(
            package.get_css_file(),
            Some("body { color: red; }".to_string())
        );
    }

    #[test]
    fn test_js_file() {
        let mut metadata = PackageMetadata::default();
        let custom = Custom {
            id: "custom_challenge".to_string(),
            name: "Custom Challenge".to_string(),
            description: "This is a custom challenge".to_string(),
            js: "script.js".to_string(),
            ..Default::default()
        };
        metadata.custom = custom.clone();
        let mut package = Package::new(metadata);

        package.add_file(
            "challenge.yml".to_string(),
            serde_yaml::to_string(&custom).unwrap().into_bytes(),
        );
        package.add_file(
            "script.js".to_string(),
            b"console.log('Hello, World!');".to_vec(),
        );
        assert_eq!(
            package.get_js_file(),
            Some("console.log('Hello, World!');".to_string())
        );
    }

    #[test]
    fn test_results_file() {
        let mut metadata = PackageMetadata::default();
        let custom = Custom {
            id: "custom_challenge".to_string(),
            name: "Custom Challenge".to_string(),
            description: "This is a custom challenge".to_string(),
            results_html: Some("results.html".to_string()),
            ..Default::default()
        };
        metadata.custom = custom.clone();
        let mut package = Package::new(metadata);

        package.add_file(
            "challenge.yml".to_string(),
            serde_yaml::to_string(&custom).unwrap().into_bytes(),
        );
        package.add_file("results.html".to_string(), b"<html></html>".to_vec());
        assert_eq!(
            package.get_results_file(),
            Some("<html></html>".to_string())
        );
    }
}
