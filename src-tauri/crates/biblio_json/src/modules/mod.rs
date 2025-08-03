pub mod bible;
pub mod dict;

use bible::BibleModule;

use crate::modules::dict::DictModule;



#[derive(Debug)]
pub enum Module
{
    Bible(BibleModule),
    Dictionary(DictModule)
}
