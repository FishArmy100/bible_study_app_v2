use std::{fs, path::Path};

use biblio_json::Package;
use itertools::Itertools;

fn main()
{
    let package = match Package::load("./res") {
        Ok(ok) => ok,
        Err(e) => return println!("Errors:\n{}\n", e.iter().join("\n"))
    };

    write_file("./out/out.txt", &format!("{:#?}", package)).unwrap();
}

pub fn write_file<P>(path: P, src: &str) -> Result<(), String>
    where P : AsRef<Path>
{
    fs::write(path, src).map_err(|e| e.to_string())
}

