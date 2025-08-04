use itertools::{EitherOrBoth, Itertools};
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
    pub license: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DictEntry
{
    pub term: String,
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
    pub license: Option<String>,
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
            license: config.license,
            entries,
        })
    }

    pub fn find(&self, term: &str) -> Option<&DictEntry>
    {
        self.entries.iter().find(|entry| {
            let contains_alias = entry.aliases.as_ref().is_some_and(|a| a.iter().find(|t| eq_ignore_punc_and_case(t, term)).is_some());
            eq_ignore_punc_and_case(&entry.term, term) || contains_alias
        })
    }
}

fn eq_ignore_punc_and_case(a: &str, b: &str) -> bool
{
    let a_chars = get_normalized_str_chars(a);
    let b_chars = get_normalized_str_chars(b);

    for pair in a_chars.zip_longest(b_chars)
    {
        let EitherOrBoth::Both(a, b) = pair else {
            return false;
        };

        if a != b 
        {
            return false
        }
    }

    true
}

fn get_normalized_str_chars(s: &str) -> impl Iterator<Item = char>
{
    s.chars().into_iter()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '\'')
        .map(|c| c.to_ascii_lowercase())
}