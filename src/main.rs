#![allow(non_snake_case)]
use rust_i18n::t;
use whoami::Language;

use TransJLC::{JLC, JlcTrait};

rust_i18n::i18n!("i18n");

mod Cli;

fn set_language(language: &str) -> Result<(), Box<dyn std::error::Error>> {
    if language == "auto" {
        return Ok(());
    }
    let i18n_list = rust_i18n::available_locales!();
    // 不支持的语言默认使用英语
    if !i18n_list.contains(&language) {
        println!("{}", "Language not supported");
        rust_i18n::set_locale("en");
    } else {
        rust_i18n::set_locale(&language);
    }
    Ok(())
}

fn default_language() -> Result<(), Box<dyn std::error::Error>> {
    let language: Vec<_> = whoami::langs()?
        .map(|lang: Language| lang.to_string())
        .collect();
    let language = language[0].as_str().to_owned();
    set_language(&language)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    default_language()?;

    let matches = Cli::cli_command().get_matches();
    let trans_jlc = Cli::TransJLC::new(&matches);
    set_language(trans_jlc.language.as_str())?;

    let path = trans_jlc.path.clone();
    let output = trans_jlc.output_path.clone();
    let eda = match trans_jlc.EDA.to_lowercase().as_str() {
        "auto" => TransJLC::EDA::Auto,
        "protel" => TransJLC::EDA::Protel,
        "kciad" => TransJLC::EDA::Kicad,
        _ => TransJLC::EDA::Custom(trans_jlc.EDA.clone()),
    };

    let mut jlc = JLC::new(path, output, eda);
    jlc.copy_file()?;

    if trans_jlc.zip {
        jlc.zip_file(trans_jlc.zip_name.as_str())?;
    }
    
    println!("{}", t!("success_log"));
    println!("Hash aperture functionality has been integrated into the file processing.");

    Ok(())
}
