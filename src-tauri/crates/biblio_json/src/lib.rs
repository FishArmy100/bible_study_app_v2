pub(crate) mod utils;
pub mod modules;
pub mod ref_id;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::modules::{bible::BibleModule, dict::DictModule, Module};

pub const PACKAGE_FILE_NAME: &str = "biblio-json.toml";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PackageConfig
{
    pub name: String,
    pub authors: Vec<String>,
    pub license: String,
    pub module_paths: Option<ModulePaths>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ModulePaths
{
    pub bibles: Option<String>,
    pub dictionaries: Option<String>,
}

#[derive(Debug)]
pub struct Package 
{
    pub name: String,
    pub authors: Vec<String>,
    pub license: String,
    pub modules: Vec<Module>
}

impl Package
{
    pub fn load(dir_path: &str) -> Result<Self, Vec<String>>
    {
        let path = Path::new(dir_path);

        if !path.is_dir()
        {
            return Err(vec![format!("Provided path: {dir_path}, must be a directory")]);
        }

        let config_path = path.join(Path::new(PACKAGE_FILE_NAME));
        let file = utils::load_file(config_path).map_err(|e| vec![e])?;
        let config = match toml::from_str::<PackageConfig>(&file) {
            Ok(ok) => ok,
            Err(e) => return Err(vec![e.to_string()])
        };

        let modules = match &config.module_paths {
            Some(paths) => Self::load_modules(dir_path, paths)?,
            None => vec![]
        };

        Ok(Self {
            name: config.name,
            authors: config.authors,
            license: config.license,
            modules
        })
    }

    fn load_modules(root: &str, paths: &ModulePaths) -> Result<Vec<Module>, Vec<String>>
    {
        let mut modules = vec![];
        let mut errors = vec![];
        
        if let Some(bibles_path) = &paths.bibles
        {
            match Self::load_bibles(root, &bibles_path)
            {
                Ok(ok) => modules.extend(ok.into_iter().map(|b| Module::Bible(b))),
                Err(e) => errors.push(e),
            }
        }

        if let Some(dictionary_paths) = &paths.dictionaries
        {
            match Self::load_dictionaries(root, &dictionary_paths)
            {
                Ok(ok) => modules.extend(ok.into_iter().map(|d| Module::Dictionary(d))),
                Err(e) => errors.push(e),
            }
        }

        if errors.len() > 0
        {
            Err(errors)
        }
        else 
        {
            Ok(modules)    
        }
    }

    fn load_bibles(base_dir: &str, pattern: &str) -> Result<Vec<BibleModule>, String>
    {
        let full_path = format!("{}/{}", base_dir, pattern);

        glob::glob(&full_path).map_err(|e| e.to_string())?.filter_map(|entry| -> Option<Result<BibleModule, String>> {
            let entry = match entry {
                Ok(ok) => ok,
                Err(e) => return Some(Err(e.to_string())),
            };

            let path = Path::new(&entry);

            let ext = path.extension().map(|s| s.to_str()).flatten();
            if ext != Some("toml")
            {
                return None;
            }
            
            let dir = match path.parent() {
                Some(s) => s,
                None => return Some(Err(format!("Expected path {} to have a parent", path.display())))
            }.to_str().unwrap();

            let name = match path.file_stem() {
                Some(s) => s,
                None => return Some(Err(format!("Expected path {} to have a stem", path.display())))
            }.to_str().unwrap();

            Some(BibleModule::load(dir, name))
        }).collect()
    }
    
    fn load_dictionaries(base_dir: &str, pattern: &str) -> Result<Vec<DictModule>, String> 
    {
        let full_path = format!("{}/{}", base_dir, pattern);

        glob::glob(&full_path).map_err(|e| e.to_string())?.filter_map(|entry| -> Option<Result<DictModule, String>> {
            let entry = match entry {
                Ok(ok) => ok,
                Err(e) => return Some(Err(e.to_string())),
            };

            let path = Path::new(&entry);

            let ext = path.extension().map(|s| s.to_str()).flatten();
            if ext != Some("toml")
            {
                return None;
            }
            
            let dir = match path.parent() {
                Some(s) => s,
                None => return Some(Err(format!("Expected path {} to have a parent", path.display())))
            }.to_str().unwrap();

            let name = match path.file_stem() {
                Some(s) => s,
                None => return Some(Err(format!("Expected path {} to have a stem", path.display())))
            }.to_str().unwrap();

            Some(DictModule::load(dir, name))
        }).collect()
    }
}
