use std::path::PathBuf;

#[cfg(feature = "csr")]
use gloo::net::http::Request;

#[derive(Debug, Clone)]
pub enum AssetLoader {
    /// Load assets from URLs (used in CSR)
    Url { base_url: String },
    /// Load assets from the filesystem (used in SSR)
    File { base_dirs: Vec<PathBuf> },
}

impl Default for AssetLoader {
    fn default() -> Self {
        #[cfg(feature = "csr")]
        {
            // Default for CSR is to use URL loader with /assets/ base
            AssetLoader::Url {
                base_url: "/assets/".to_string(),
            }
        }

        #[cfg(all(feature = "ssr", not(feature = "csr")))]
        {
            // Default for SSR is to use File loader with BUILD_DIR or current directory
            let mut base_dirs = Vec::new();

            // Try BUILD_DIR environment variable if set
            if let Ok(build_dir) = std::env::var("BUILD_DIR") {
                base_dirs.push(PathBuf::from(build_dir));
            }

            // Add current directory and public directory as fallbacks
            base_dirs.push(PathBuf::from("./"));
            base_dirs.push(PathBuf::from("assets"));

            AssetLoader::File { base_dirs }
        }

        #[cfg(not(any(feature = "csr", feature = "ssr")))]
        {
            // Default when neither feature is enabled
            AssetLoader::File {
                base_dirs: vec![PathBuf::from("./")],
            }
        }
    }
}

impl AssetLoader {
    /// Create a URL-based asset loader
    #[cfg(feature = "csr")]
    pub fn new_url(base_url: impl Into<String>) -> Self {
        AssetLoader::Url {
            base_url: base_url.into(),
        }
    }

    /// Create a file-based asset loader
    pub fn new_file(base_dirs: Vec<PathBuf>) -> Self {
        AssetLoader::File { base_dirs }
    }

    /// Create a file-based asset loader from a single directory
    pub fn from_dir(dir: impl Into<PathBuf>) -> Self {
        AssetLoader::File {
            base_dirs: vec![dir.into()],
        }
    }

    /// Load a binary asset (like a zip file)
    pub async fn load_binary(&self, path: &str) -> Result<Vec<u8>, String> {
        match self {
            #[cfg(feature = "csr")]
            AssetLoader::Url { base_url } => {
                // Normalize URL by ensuring base_url ends with / and path doesn't start with /
                let normalized_path = path.trim_start_matches('/');
                let base_url = if base_url.ends_with('/') {
                    base_url.to_string()
                } else {
                    format!("{}/", base_url)
                };

                let url = format!("{}{}", base_url, normalized_path);

                let response = Request::get(&url)
                    .send()
                    .await
                    .map_err(|e| format!("Failed to send request to {}: {}", url, e))?;

                if response.status() != 200 {
                    return Err(format!(
                        "Failed to load asset {}: status {}",
                        url,
                        response.status()
                    ));
                }

                response
                    .binary()
                    .await
                    .map_err(|e| format!("Failed to read response from {}: {}", url, e))
            }

            AssetLoader::File { base_dirs } => {
                // Try each base directory
                for base_dir in base_dirs {
                    let file_path = base_dir.join(path);
                    if file_path.exists() {
                        return std::fs::read(&file_path).map_err(|e| {
                            format!("Failed to read file {}: {}", file_path.display(), e)
                        });
                    }
                }

                // If path is absolute, try it directly
                let path_buf = PathBuf::from(path);
                if path_buf.is_absolute() && path_buf.exists() {
                    return std::fs::read(&path_buf)
                        .map_err(|e| format!("Failed to read file {}: {}", path_buf.display(), e));
                }

                Err(format!("File not found: {}", path))
            }

            #[allow(unreachable_patterns)]
            _ => Err("Asset loading is not available in this configuration".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "csr")]
    use wasm_bindgen_test::*;

    #[cfg(feature = "csr")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[cfg(all(test, feature = "ssr", not(feature = "csr")))]
    const KONNEKTOREN_YAML_CONTENT: &str = include_str!("../assets/konnektoren.yml");

    // Test for SSR environment
    #[cfg(all(test, feature = "ssr", not(feature = "csr")))]
    #[tokio::test]
    async fn test_load_konnektoren_yaml_ssr() {
        use crate::asset_loader::AssetLoader;
        use std::io::Write;
        use tempfile::TempDir;
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();

        // Create the assets directory inside the temp dir
        let assets_dir = temp_dir.path().join("assets");
        std::fs::create_dir_all(&assets_dir).unwrap();

        // Write the konnektoren.yml file to the assets directory
        let yaml_path = assets_dir.join("konnektoren.yml");
        let mut file = std::fs::File::create(&yaml_path).unwrap();
        file.write_all(KONNEKTOREN_YAML_CONTENT.as_bytes()).unwrap();

        // Set the BUILD_DIR environment variable to the temp directory
        std::env::set_var("BUILD_DIR", temp_dir.path().to_str().unwrap());

        // Create asset loader
        let asset_loader = AssetLoader::default();

        // Load the file
        let content = asset_loader
            .load_binary("assets/konnektoren.yml")
            .await
            .unwrap();

        // Verify content
        let content_str = String::from_utf8(content).unwrap();
        assert!(content_str.contains("id: \"konnektoren\""));
        assert!(content_str.contains("name: \"Konnektoren\""));
        assert!(content_str.contains("lang: \"de\""));
    }

    // Test with explicit file loader
    #[cfg(all(test, feature = "ssr", not(feature = "csr")))]
    #[tokio::test]
    async fn test_load_konnektoren_yaml_file_loader() {
        use crate::asset_loader::AssetLoader;
        use std::io::Write;
        use tempfile::TempDir;

        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();

        // Create the assets directory inside the temp dir
        let assets_dir = temp_dir.path().join("assets");
        std::fs::create_dir_all(&assets_dir).unwrap();

        // Write the konnektoren.yml file to the assets directory
        let yaml_path = assets_dir.join("konnektoren.yml");
        let mut file = std::fs::File::create(&yaml_path).unwrap();
        file.write_all(KONNEKTOREN_YAML_CONTENT.as_bytes()).unwrap();

        // Create asset loader with specific directory
        let asset_loader = AssetLoader::from_dir(temp_dir.path());

        // Load the file
        let content = asset_loader
            .load_binary("assets/konnektoren.yml")
            .await
            .unwrap();

        // Verify content
        let content_str = String::from_utf8(content).unwrap();
        assert!(content_str.contains("id: \"konnektoren\""));
        assert!(content_str.contains("name: \"Konnektoren\""));
        assert!(content_str.contains("lang: \"de\""));
    }

    // Test for CSR environment - we need to mock fetch API in a browser environment
    #[cfg(feature = "csr")]
    #[wasm_bindgen_test]
    async fn test_load_konnektoren_yaml_csr() {
        // In a real browser environment with proper server setup,
        // we would test against a real file, but for a unit test
        // we'll just check that the code doesn't panic
        let asset_loader = AssetLoader::default();

        // This will typically fail in a test environment as there's no server,
        // but we want to make sure the URL is constructed correctly
        let result = asset_loader.load_binary("assets/konnektoren.yml").await;

        // In an actual integration test, you'd set up a mock server and assert:
        // assert!(result.is_ok());
        // For unit test, we'll just ensure our code runs without panicking
        match result {
            Ok(_) => {
                println!("Successfully loaded the file (integration environment)");
            }
            Err(e) => {
                println!("Expected error in test environment: {}", e);
                // Make sure it tried to load from the correct URL
                assert!(e.contains("assets/konnektoren.yml"));
            }
        }
    }

    // Test for custom URL
    #[cfg(feature = "csr")]
    #[wasm_bindgen_test]
    async fn test_custom_url_base_csr() {
        let asset_loader = AssetLoader::new_url("https://example.com/static");

        // Check that the URL is constructed correctly with the custom base
        let result = asset_loader.load_binary("assets/konnektoren.yml").await;

        match result {
            Ok(_) => {
                println!("Successfully loaded the file (integration environment)");
            }
            Err(e) => {
                println!("Expected error in test environment: {}", e);
                // Make sure it tried to load from the correct URL with custom base
                assert!(e.contains("https://example.com/static/assets/konnektoren.yml"));
            }
        }
    }
}
