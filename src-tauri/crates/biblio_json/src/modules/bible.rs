use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{ref_id::RefId, utils};

#[derive(Debug, Serialize, Deserialize)]
pub struct BibleConfig
{
    pub name: String,
    pub description: String,
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

        let bible_path = format!("{}/{}.jsonl", dir_path, name);
        let source = BibleSource::from_file(&bible_path)?;

        Ok(Self { 
            name: config.name, 
            description: config.description,
            source,
        })
    }
}

#[derive(Debug)]
pub struct BibleSource
{
    pub book_infos: HashMap<u32, BookInfo>,
    pub verses: HashMap<RefId, Verse>
}

impl BibleSource
{
    pub fn from_file(path: &str) -> Result<BibleSource, String>
    {
        let verses: Vec<Verse> = utils::load_json_lines(path)?;
    }
}

impl BibleSource
{

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verse
{
    pub id: RefId,
    pub words: Vec<Word>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Word 
{
    pub red: Option<bool>,
    pub italics: Option<bool>,
    pub text: String,
}

#[derive(Debug)]
pub struct BookInfo
{
    pub name: String,
    pub index: u32,
    pub chapters: Vec<u32>
}