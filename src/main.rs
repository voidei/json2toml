#![feature(file_create_new)]
use std::{
    fs::{self, File},
    io::{prelude::*, BufRead, BufReader},
};

use anyhow::Result;
use clap::{Arg, Command};

fn app() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg_required_else_help(true)
        .arg(
            Arg::new("input")
                .help("Input JSON to convert to TOML")
                .index(1),
        )
}
fn main() -> Result<()> {
    let output_file = "output.toml";
    let matches = app().get_matches();
    let input_file = matches.get_one::<String>("input").unwrap();
    let input_data = fs::read(input_file)?;

    let json_data: toml::Value = serde_json::from_slice(&input_data)?;
    let toml_output: String = json_data.to_string();

    let mut file = File::create_new(output_file).expect("error while creating file");
    writeln!(&mut file, "{toml_output}")?;
    // file.write_all(toml_output.as_bytes());

    let input = File::open(output_file)?;
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line?);
    }
    Ok(())
}
