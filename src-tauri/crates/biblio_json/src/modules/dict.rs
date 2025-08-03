use serde::{Deserialize, Serialize};

use crate::utils;


#[derive(Debug, Serialize, Deserialize)]
pub struct DictConfig
{
    pub name: String,
    pub authors: Vec<String>,
    pub language: String,
    pub description: Option<String>,
    pub data_source: Option<String>,
    pub pub_year: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DictEntry
{
    pub word: String,
    pub aliases: Option<Vec<String>>,
    pub definitions: Vec<String>,
}

impl DictEntry
{
    pub fn from_file(path: &str) -> Result<Vec<Self>, String>
    {
        let ret = utils::load_json_lines(path)?
            .into_iter()
            .map(|(l, _)| l)
            .collect();

        Ok(ret)
    }
}

#[derive(Debug)]
pub struct DictModule
{
    pub name: String,
    pub authors: Vec<String>,
    pub language: String,
    pub description: Option<String>,
    pub pub_year: Option<u32>,
    pub entries: Vec<DictEntry>,
}

impl DictModule
{
    pub fn load(dir_path: &str, name: &str) -> Result<Self, String>
    {
        let config_path = format!("{}/{}.toml", dir_path, name);
        let config: DictConfig = utils::load_toml(config_path)?;

        let dictionary_path = format!("{}/{}.jsonl", dir_path, name);
        let entries = DictEntry::from_file(&dictionary_path)?;

        Ok(Self { 
            name: config.name, 
            authors: config.authors,
            description: config.description,
            language: config.language,
            pub_year: config.pub_year,
            entries,
        })
    }

    pub fn find(&self, name: &str) -> Option<&DictEntry>
    {
        self.entries.iter().find(|entry| {
            let contains_alias = entry.aliases.as_ref().is_some_and(|a| a.iter().find(|n| n.eq_ignore_ascii_case(name)).is_some());
            entry.word.eq_ignore_ascii_case(name) || contains_alias
        })
    }
}