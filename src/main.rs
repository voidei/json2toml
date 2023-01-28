#![feature(file_create_new)]
use std::{
    fs::{
        self,
        File,
    },
    io::{
        prelude::*,
        BufRead,
        BufReader,
    },
    path::Path,
};

use anyhow::Result;
use clap::{
    Arg,
    ArgAction,
    Command,
};

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
        .arg(
            Arg::new("print-output")
                .help("Whether or not to print the outputted file content to stdout.")
                .long("print-output")
                .action(ArgAction::SetTrue),
        )
}
fn main() -> Result<()> {
    let matches = app().get_matches();
    let input_file = matches.get_one::<String>("input").unwrap();
    let input_data = fs::read(input_file)?;

    let file = Path::new(input_file);
    let mut output_file = file.file_stem().unwrap().to_owned();
    output_file.push(".toml");

    #[cfg(test)]
    println!("input file is {}", input_file);
    #[cfg(test)]
    println!("output file is {output_file:?}");

    let json_data: toml::Value = serde_json::from_slice(&input_data)?;
    let toml_output: String = json_data.to_string();

    let output_path = Path::new(&output_file);
    if output_path.exists() {
        println!("{output_file:?} already exists! Exiting program!");
        std::process::exit(0);
    }

    let mut file = match File::create_new(&output_file) {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating {output_file:?}! Exiting program!");
            println!("{e}");
            std::process::exit(1);
        },
    };

    writeln!(&mut file, "{toml_output}")?;

    if *matches.get_one::<bool>("print-output").unwrap() {
        let input = File::open(&output_file)?;
        let buffered = BufReader::new(input);
        for line in buffered.lines() {
            println!("{}", line?);
        }
    }

    Ok(())
}
