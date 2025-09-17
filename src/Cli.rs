// SPDX-FileCopyrightText: 2025 HalfSweet
// SPDX-License-Identifier: Apache-2.0

use clap::{Arg, ArgMatches, ColorChoice, Command, value_parser};
use clap::builder::styling;
use rust_i18n::t;

pub fn cli_command() -> Command {
    let language = Arg::new("language")
        .short('l')
        .long("language")
        .help(t!("root_language_help").to_string())
        .value_parser(["auto", "en", "zh-CN", "ja"])
        .default_value("auto");

    let EDA = Arg::new("EDA")
        .short('e')
        .long("eda")
        .help(t!("root_EDA_help").to_string())
        .value_parser(["auto", "kicad", "jlc", "protel"])
        .default_value("auto");

    let path = Arg::new("path")
        .short('p')
        .long("path")
        .help(t!("root_path_help").to_string())
        .value_parser(value_parser! { String })
        .default_value(".");

    let output_path = Arg::new("output_path")
        .short('o')
        .long("output_path")
        .help(t!("root_output_path_help").to_string())
        .value_parser(value_parser! { String })
        .default_value("./output");

    let zip = Arg::new("zip")
        .short('z')
        .long("zip")
        .help(t!("root_zip_help").to_string())
        .value_parser(value_parser! { bool })
        .default_value("false");

    let zip_name = Arg::new("zip_name")
        .short('n')
        .long("zip_name")
        .help(t!("root_zip_name_help").to_string())
        .value_parser(value_parser! { String })
        .default_value("Gerber");

    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default());

    Command::new("transjlc")
        .about(t!("root_about").to_string())
        .author("HalfSweet")
        .color(ColorChoice::Auto)
        .styles(styles)
        .arg(language)
        .arg(EDA)
        .arg(path)
        .arg(output_path)
        .arg(zip)
        .arg(zip_name)
}

pub struct TransJLC {
    pub language: String,
    pub EDA: String,
    pub path: String,
    pub output_path: String,
    pub zip: bool,
    pub zip_name: String,
}

impl TransJLC {
    pub fn new(matches: &ArgMatches) -> Self {
        Self {
            language: matches.get_one::<String>("language").unwrap().to_string().replace(r"-", r"/"),
            EDA: matches.get_one::<String>("EDA").unwrap().to_string(),
            path: matches.get_one::<String>("path").unwrap().to_string(),
            output_path: matches
                .get_one::<String>("output_path")
                .unwrap()
                .to_string(),
            zip: *matches.get_one::<bool>("zip").unwrap(),
            zip_name: matches.get_one::<String>("zip_name").unwrap().to_string(),
        }
    }
}
