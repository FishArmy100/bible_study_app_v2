use biblio_json::Package;
use itertools::Itertools;

fn main()
{
    let package = match Package::load("./res") {
        Ok(ok) => ok,
        Err(e) => return println!("Errors:\n{}\n", e.iter().join("\n"))
    };

    println!("{:#?}", package);
}


