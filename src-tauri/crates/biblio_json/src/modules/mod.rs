pub mod bible;
pub mod dict;

use bible::BibleModule;

use crate::modules::dict::DictionaryModule;



#[derive(Debug)]
pub enum Module
{
    Bible(BibleModule),
    Dictionary(DictionaryModule)
}
