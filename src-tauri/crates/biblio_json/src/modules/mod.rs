pub mod bible;

use bible::BibleModule;



#[derive(Debug)]
pub enum Module
{
    Bible(BibleModule)
}
