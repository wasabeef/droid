#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate clap;

use std::fs;
use serde::Deserialize;
use prettytable::{Cell, Row, Table};
use prettytable::format;
use clap::{Arg, App, SubCommand};

#[derive(Deserialize, Debug)]
struct Version {
    version: String,
    code_name: String,
    api_level: String,
    release_date: String,
}

fn main() {
    let matches = App::new("droid")
        .about("A command-line tool for checking Android OS version history.")
        .version(crate_version!())
        .author(crate_authors!())
        .after_help("https://github.com/wasabeef/droid")
        .subcommand(SubCommand::with_name("list")
            .about("List all Android version history.")
            .alias("ls")
        )
        .subcommand(SubCommand::with_name("api")
            .alias("a")
            .about("List the specified API level.")
            .arg(Arg::with_name("level").required(true))
        )
        .subcommand(SubCommand::with_name("version")
            .alias("v")
            .about("List the specified version number.")
            .arg(Arg::with_name("number").required(true))
        )
        .subcommand(SubCommand::with_name("code")
            .alias("c")
            .about("List the specified code name.")
            .arg(Arg::with_name("name").required(true))
        )
        .get_matches();

    match matches.subcommand() {
        ("list", _) => all(),
        ("api", Some(sub_m)) => {
            api_levels(sub_m.value_of("level").unwrap().to_string())
        }
        ("version", Some(sub_m)) => {
            version_numbers(sub_m.value_of("number").unwrap().to_string())
        }
        ("code", Some(sub_m)) => {
            code_names(sub_m.value_of("name").unwrap().to_string())
        }
        _ => all()
    }
}

fn all() {
    body(read_versions());
}

fn api_levels(level: String) {
    let array = read_versions();
    body(array.into_iter().filter(|v| v.api_level == level).collect())
}

fn version_numbers(number: String) {
    let array = read_versions();
    body(array.into_iter().filter(|v| v.version.starts_with(&number)).collect())
}

fn code_names(number: String) {
    let array = read_versions();
    body(array.into_iter().filter(|v|
        v.code_name.to_lowercase().split_whitespace().collect::<String>().starts_with(&number.to_lowercase())
    ).collect())
}

fn read_versions() -> Vec<Version> {
    let contents = fs::read_to_string("resources/Android.json").unwrap();
    serde_json::from_str(&contents).unwrap()
}

fn body(versions: Vec<Version>) {
    if versions.is_empty() {
        println!("Not founded.")
    } else {
        // Create the table
        let mut table = Table::new();

        // Add a row per time
        table.set_titles(row!["VERSION", "CODE_NAME", "API_LEVEL", "RELEASE_DATE"]);
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        for version in versions.iter() {
            table.add_row(Row::new(vec![
                Cell::new(&version.version),
                Cell::new(&version.code_name),
                Cell::new(&version.api_level),
                Cell::new(&version.release_date)
            ]));
        }
        print!("\n\n");
        table.printstd();
    }
    println!("https://en.wikipedia.org/wiki/Android_version_history");
    println!("https://developer.android.com/about/dashboards");
    print!("\n\n");
}
