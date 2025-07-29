use std::{fs, path::Path};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
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

pub fn load_json<T, P>(path: P) -> Result<T, String> 
    where P : AsRef<Path>,
          T : for<'a> Deserialize<'a>
{
    let src = load_file(path)?;
    serde_json::from_str(&src)
        .map_err(|e| e.to_string())
}

pub fn load_json_lines<P, T>(path: P) -> Result<Vec<T>, String>
    where P : AsRef<Path>,
          T : for<'a> Deserialize<'a> + Send + Sync + 'static
{
    let src = load_file(path)?;
    src.lines().collect_vec().into_par_iter().map(|json| {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }).collect()
}