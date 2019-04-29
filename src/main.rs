extern crate clap;
extern crate toml;

#[macro_use]
extern crate serde_derive;

use clap::App;
// use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
struct Config {
    package: Package,
}

#[derive(Debug, Deserialize)]
struct Package {
    version: String,
}

fn main() -> io::Result<()> {
    let mut file = File::open("Cargo.toml")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let config: Config = toml::from_str(&buffer).unwrap();

    App::new("pbnj")
        .version(config.package.version.as_str())
        .author("Peter Benjamin <https://pbnj.dev>")
        .about("A CLI to help you get in touch with me")
        .get_matches();

    Ok(())
}
