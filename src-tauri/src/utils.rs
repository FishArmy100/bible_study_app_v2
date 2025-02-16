use std::{error::Error, hash::{DefaultHasher, Hash, Hasher}, path::Path};

use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_hex(hex: &str) -> Option<Self> {
        if !hex.starts_with('#') {
            return None;
        }
        let Ok(r) = u8::from_str_radix(&hex[1..=2], 16) else {
            return None;
        };
        let Ok(g) = u8::from_str_radix(&hex[3..=4], 16) else {
            return None;
        };
        let Ok(b) = u8::from_str_radix(&hex[5..=6], 16) else {
            return None;
        };

        Some(Self { r, g, b })
    }
}

pub fn get_hash_code<T>(value: &T) -> u64
where
    T: Hash,
{
    let mut s = DefaultHasher::new();
    value.hash(&mut s);
    s.finish()
}

#[macro_export]
macro_rules! debug_release_val 
{
    (debug: $debug_val:expr, release: $release_val:expr $(,)?) => {
        if cfg!(debug_assertions)
        {
            $debug_val
        }
        else 
        {
            $release_val 
        }
    };
}

pub fn open_file_explorer(path: &str) -> Result<(), Box<dyn Error>> 
{
    let path = Path::new(path);
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()).into());
    }

    if !path.is_dir() {
        return Err(format!("Path must be a directory: {}", path.display()).into())
    }

    // Use the `open` crate to open the path in the system's file explorer
    open::that(path)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo
{
    pub version: String,
    pub bibles: Vec<String>,
    pub save_version: String,
}
