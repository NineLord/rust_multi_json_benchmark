// #![allow(unused, dead_code)]

/* #region Imports */
// Standard
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::fs;

// 3rd Party
use home::home_dir;
use once_cell::sync::Lazy;
use structopt::StructOpt;

// Project
use rust_multi_json_benchmark::json_generator::Generator;
/* #endregion */

/* #region Default Values */
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

static DEFAULT_PATH_TO_SAVE_FILE: Lazy<String> = Lazy::new(|| {
    let mut path = match home_dir() {
        Some(path_buffer) => path_buffer,
        None => env::current_dir()
            .expect("Failed to get the home directory and the current working directory"),
    };

    path.push("generatedJson.json");

    path.into_os_string()
        .into_string()
        .expect("Failed to convert the PathBuf of DEFAULT_PATH_TO_SAVE_FILE to String")
});
/* #endregion */

/// Generates JSON file for testing
#[derive(StructOpt, Debug)]
#[structopt(name = "jsonGenerator", rename_all = "kebab-case")]
struct OptionalArguments {
    /// Absolute path to the file location to be saved
    #[structopt(parse(from_os_str), default_value = &DEFAULT_PATH_TO_SAVE_FILE)]
    path_to_save_file: PathBuf,

    /// the total number of letters that each generated node name will have
    #[structopt(short, long, default_value = "7")]
    number_of_letters: u8,

    /// The depth of the JSON tree
    #[structopt(short, long, default_value = "100")]
    depth: u8,

    /// The number of children each node should have
    #[structopt(short = "m", long, default_value = "6")]
    number_of_children: u8,

    /// Print the resulting JSON instead of saving it to a file
    #[structopt(short = "P", long)]
    print: bool,
}

// Example: clear ; cargo run --bin json_generator -- -d10 -m5 -n8 /mnt/c/Users/Shaked/Documents/Mine/IdeaProjects/PreReactivePoc/junk/hugeJson_numberOfLetters8_depth10_children5.json

fn main() -> Result<(), Box<dyn Error>> {
    let options = OptionalArguments::from_args();
    let json = Generator::generate_json(&ALPHABET, options.number_of_letters, options.depth, options.number_of_children)?;

    if options.print {
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        fs::write(options.path_to_save_file.as_path(), serde_json::to_string(&json)?)?;
    }

    Ok(())
}
