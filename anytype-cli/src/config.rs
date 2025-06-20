use anyhow::Result;
use std::path::PathBuf;

/// Get the path to the configuration directory
pub fn config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
        .join("anytype-cli");
    
    std::fs::create_dir_all(&config_dir)?;
    Ok(config_dir)
}

/// Get the path to the API key file
pub fn api_key_file() -> Result<PathBuf> {
    Ok(config_dir()?.join("api_key"))
}

/// Save API key to file
pub fn save_api_key(api_key: &str) -> Result<()> {
    let key_file = api_key_file()?;
    std::fs::write(key_file, api_key)?;
    Ok(())
}

/// Load API key from file
pub fn load_api_key() -> Result<Option<String>> {
    let key_file = api_key_file()?;
    
    if key_file.exists() {
        let api_key = std::fs::read_to_string(key_file)?.trim().to_string();
        if api_key.is_empty() {
            Ok(None)
        } else {
            Ok(Some(api_key))
        }
    } else {
        Ok(None)
    }
}

/// Remove stored API key
pub fn remove_api_key() -> Result<()> {
    let key_file = api_key_file()?;
    if key_file.exists() {
        std::fs::remove_file(key_file)?;
    }
    Ok(())
}
