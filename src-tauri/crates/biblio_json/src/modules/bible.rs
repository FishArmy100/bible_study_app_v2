use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{modules::JsonFormat, ref_id::RefId, utils};

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
    pub source: BibleSource,
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

#[derive(Debug)]
pub struct BibleSource
{
    pub book_infos: HashMap<u32, BookInfo>,
    pub verses: HashMap<RefId, Verse>
}

#[derive(Debug)]
pub struct Verse
{
    pub id: RefId,
    pub words: Vec<Word>
}

#[derive(Debug)]
pub struct Word 
{
    pub red: bool,
    pub italics: bool,
    pub text: String,
}

#[derive(Debug)]
pub struct BookInfo
{
    pub name: String,
    pub index: u32,
    pub chapters: Vec<u32>
}