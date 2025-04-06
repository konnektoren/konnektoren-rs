use crate::challenges::{Package, PackageMetadata};
use std::collections::HashMap;
use std::io::{Cursor, Read};
use zip::ZipArchive;

#[cfg(not(feature = "ssr"))]
use gloo::net::http::Request;

pub struct PackageReader;

impl PackageReader {
    #[cfg(not(feature = "ssr"))]
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

    #[cfg(feature = "ssr")]
    pub async fn download(url: &str) -> Result<Vec<u8>, String> {
        // Use std::env::var instead of env! macro
        let build_dir = std::env::var("BUILD_DIR")
            .map_err(|_| "BUILD_DIR environment variable is not set".to_string())?;

        // SSR: Load from local file (assuming the url is a relative path to the file)
        let file_path = format!("{}/{}", build_dir, url);
        log::info!("SSR: Loading package from file: {}", file_path);

        match std::fs::read(file_path) {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(format!("Failed to read file: {}", e)),
        }
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
    use std::{env, fs};
    use tempdir::TempDir;

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

    #[cfg(feature = "ssr")]
    #[tokio::test]
    async fn test_package_reader_ssr() {
        // 1. Create a temporary directory for BUILD_DIR
        let temp_dir = TempDir::new("test_pkg_").unwrap(); // Added prefix parameter
        let build_dir_path = temp_dir.path().to_str().unwrap().to_string();

        // 2. Create a test zip file within the temporary directory
        let test_zip_content = include_bytes!("../../assets/articles-pkg.zip");
        let test_zip_path = format!("{}/test_package.zip", build_dir_path);
        fs::write(&test_zip_path, test_zip_content).unwrap();

        // 3. Set the BUILD_DIR environment variable
        env::set_var("BUILD_DIR", &build_dir_path);
        let _guard = SetEnvVariableGuard("BUILD_DIR".to_string(), Some(build_dir_path.clone()));
        // Fixed string conversion

        // Rest of the test remains the same...
    }

    struct SetEnvVariableGuard(String, Option<String>);

    impl SetEnvVariableGuard {
        fn new(name: impl Into<String>, value: Option<String>) -> Self {
            let name = name.into();
            let original_value = env::var(&name).ok();

            match &value {
                Some(value) => env::set_var(&name, value),
                None => env::remove_var(&name),
            }

            Self(name, original_value)
        }
    }

    impl Drop for SetEnvVariableGuard {
        fn drop(&mut self) {
            match &self.1 {
                Some(value) => env::set_var(&self.0, value),
                None => env::remove_var(&self.0),
            }
        }
    }
}
