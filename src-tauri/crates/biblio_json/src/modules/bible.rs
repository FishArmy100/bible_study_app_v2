use serde::{Deserialize, Serialize};

use crate::{modules::JsonFormat, utils};

#[derive(Debug, Serialize, Deserialize)]
pub struct BibleConfig
{
    pub name: String,
    pub description: String,
    pub format: JsonFormat,
}

#[derive(Debug)]
pub struct BibleModule
{
    pub name: String,
    pub description: String,
}

impl BibleModule
{
    pub fn load(dir_path: &str, name: &str) -> Result<BibleModule, String>
    {
        let config_path = format!("{}/{}.toml", dir_path, name);
        let config: BibleConfig = utils::load_toml(config_path)?;
        Ok(Self { 
            name: config.name, 
            description: config.description 
        })
    }
}

pub struct BibleSource
{

}

pub struct Verse
{
    
}