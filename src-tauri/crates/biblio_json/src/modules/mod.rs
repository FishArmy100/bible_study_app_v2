pub mod bible;
pub mod dict;
pub mod xrefs;

use bible::BibleModule;

use crate::modules::dict::DictModule;



#[derive(Debug)]
pub enum Module
{
    Bible(BibleModule),
    Dictionary(DictModule)
}
