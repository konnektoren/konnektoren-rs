use crate::challenges::{ChallengeConfig, Custom, Package, PackageMetadata};
use gloo::net::http::Request;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use zip::ZipArchive;

pub struct PackageReader;

impl PackageReader {
    pub async fn download(url: &str) -> Result<Vec<u8>, String> {
        let response = Request::get(url)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if response.status() != 200 {
            return Err(format!("Failed to download package: {}", response.status()));
        }

        let bytes = response
            .binary()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        Ok(bytes)
    }

    pub fn read(package_data: &[u8]) -> Result<Package, String> {
        let reader = Cursor::new(package_data);
        let mut archive =
            ZipArchive::new(reader).map_err(|e| format!("Failed to create ZIP archive: {}", e))?;

        let mut files = HashMap::new();
        let mut metadata = None;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| format!("Failed to read file from archive: {}", e))?;
            let file_name = file.name().to_string();

            let mut contents = Vec::new();
            file.read_to_end(&mut contents)
                .map_err(|e| format!("Failed to read file contents: {}", e))?;

            if file_name == "config.yml" {
                let config: ChallengeConfig = serde_yaml::from_slice(&contents)
                    .map_err(|e| format!("Failed to parse config.yml: {}", e))?;
                metadata.get_or_insert(PackageMetadata {
                    config,
                    custom: Custom::default(),
                });
            } else if file_name == "challenge.yml" {
                let custom: Custom = serde_yaml::from_slice(&contents)
                    .map_err(|e| format!("Failed to parse challenge.yml: {}", e))?;
                if let Some(meta) = &mut metadata {
                    meta.custom = custom;
                }
            } else {
                files.insert(file_name, contents);
            }
        }

        let metadata = metadata.ok_or_else(|| "Missing metadata files in package".to_string())?;
        Ok(Package { metadata, files })
    }
}
