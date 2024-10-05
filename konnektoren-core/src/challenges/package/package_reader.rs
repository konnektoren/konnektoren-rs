use crate::challenges::{Package, PackageMetadata};
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
        let mut config = None;
        let mut custom = None;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| format!("Failed to read file from archive: {}", e))?;
            let file_name = file.name().to_string();

            let mut contents = Vec::new();
            file.read_to_end(&mut contents)
                .map_err(|e| format!("Failed to read file contents: {}", e))?;

            match file_name.as_str() {
                "config.yml" => {
                    config = Some(
                        serde_yaml::from_slice(&contents)
                            .map_err(|e| format!("Failed to parse config.yml: {}", e))?,
                    );
                }
                "challenge.yml" => {
                    custom = Some(
                        serde_yaml::from_slice(&contents)
                            .map_err(|e| format!("Failed to parse challenge.yml: {}", e))?,
                    );
                }
                _ => {
                    files.insert(file_name, contents);
                }
            }
        }

        let config = config.ok_or_else(|| "Missing config.yml in package".to_string())?;
        let custom = custom.unwrap_or_default();
        let metadata = PackageMetadata { config, custom };

        Ok(Package { metadata, files })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    #[test]
    fn test_package_reader() {
        env_logger::init();
        let package_data = include_bytes!("../../assets/articles-pkg.zip");
        let package = PackageReader::read(package_data).unwrap();

        assert_eq!(package.files.len(), 5);
        log::debug!("files {:?}", package.files.keys());
        assert!(package.get_html_file().is_some());
        assert!(package.get_css_file().is_some());
        assert!(package.get_js_file().is_some());
        assert!(package.get_results_file().is_some());
    }
}
