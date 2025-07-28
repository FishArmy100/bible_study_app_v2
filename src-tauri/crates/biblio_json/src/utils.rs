use std::{fs, path::Path};

use serde::Deserialize;

pub fn load_file<P>(path: P) -> Result<String, String>
    where P : AsRef<Path>
{
    match fs::read(path)
    {
        Ok(ok) => match String::from_utf8(ok)
        {
            Ok(ok) => Ok(ok),
            Err(err) => return Err(err.to_string()),
        }
        Err(err) => return Err(err.to_string())
    }
}

pub fn load_toml<T, P>(path: P) -> Result<T, String>
    where P : AsRef<Path>,
          T : for<'a> Deserialize<'a>
{
    let src = load_file(path)?;
    toml::from_str(&src)
        .map_err(|e| e.to_string())
}