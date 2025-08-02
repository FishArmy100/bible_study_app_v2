use serde::{Deserialize, Serialize};

use crate::utils;


#[derive(Debug, Serialize, Deserialize)]
pub struct DictionaryConfig
{
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub language: String,
    pub pub_year: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DictionaryEntry
{
    pub word: String,
    pub aliases: Vec<String>,
    pub description: String,
}

impl DictionaryEntry
{
    pub fn from_file(path: &str) -> Result<Vec<Self>, String>
    {
        utils::load_json_lines(path)?
            .into_iter()
            .map(|(l, _)| l)
            .collect()
    }
}

#[derive(Debug)]
pub struct DictionaryModule
{
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub language: String,
    pub pub_year: Option<u32>,
    pub entries: Vec<DictionaryEntry>,
}

impl DictionaryModule
{
    pub fn load(dir_path: &str, name: &str) -> Result<Self, String>
    {
        let config_path = format!("{}/{}.toml", dir_path, name);
        let config: DictionaryConfig = utils::load_toml(config_path)?;

        let dictionary_path = format!("{}/{}.jsonl", dir_path, name);
        let entries = DictionaryEntry::from_file(&dictionary_path)?;

        Ok(Self { 
            name: config.name, 
            authors: config.authors,
            description: config.description,
            language: config.language,
            pub_year: config.pub_year,
            entries,
        })
    }
}