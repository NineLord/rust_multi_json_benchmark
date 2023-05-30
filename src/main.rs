#![allow(unused, dead_code)] // Shaked-TODO: delete this
/* #region Imports */
// Standard
use std::{
    env,
    fs,
    thread,
    time::Duration,
    error::Error,
    path::{Path, PathBuf},
    sync::mpsc,
    num::ParseIntError,
};

// 3rd Party
use home::home_dir;
use once_cell::sync::Lazy;
use structopt::StructOpt;
use serde_json::{ Value, json };
use tokio::runtime::Builder;

// Project
use rust_multi_json_benchmark::json_generator;
use rust_multi_json_benchmark::search_tree::{ breadth_first_search, depth_first_search };
use rust_multi_json_benchmark::test_json::{
    config::{Config, Configs},
    reporter::Report,
    excel_generator::ExcelGenerator
};
/* #endregion */

/* #region Default values */

static DEFAULT_PATH_TO_SAVE_FILE: Lazy<String> = Lazy::new(|| {
    let mut path = match home_dir() {
        Some(path_buffer) => path_buffer,
        None => env::current_dir()
            .expect("Failed to get the home directory and the current working directory"),
    };

    path.push("report_rust.xlsx");

    path.into_os_string()
        .into_string()
        .expect("Failed to convert the PathBuf of DEFAULT_PATH_TO_SAVE_FILE to String")
});

#[allow(unused)]
static DEFAULT_PATH_TO_DEBUG_DIRECTORY: Lazy<String> = Lazy::new(|| {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("junk");

    path.into_os_string()
        .into_string()
        .expect("Couldn't get DEFAULT_PATH_TO_DEBUG_DIRECTORY")
});

static CHARACTER_POLL: &str = "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz!@#$%&";
/* #endregion */

/* #region CLI Arguments */
fn parse_config(source: &str) -> Result<Configs, Box<dyn Error>> {
    let raw_config_file = fs::read_to_string(Path::new(source))?;
    let configs: Configs = serde_json::from_str(&raw_config_file)?;
    Ok(configs)
}

fn parse_none_zero_u8(source: &str) -> Result<u8, Box<dyn Error>> {
    let num: u8 = source.parse()?;
    if (num == 0) {
        Err("The number has to be none zero".into())
    } else {
        Ok(num)
    }
}

/// Tests JSON manipulations
#[derive(StructOpt, Debug)]
#[structopt(name = "jsonTester", rename_all = "kebab-case")]
struct OptionalArguments {
    /// Absolute path to the JSON file that will be tested
    #[structopt(name = "config-path", parse(try_from_str = parse_config))]
    configs: Configs,

    /// The number of times will run the tests
    #[structopt(parse(try_from_str = parse_none_zero_u8), default_value = "5")]
    test_counter: u8,

    /// Absolute path to save the excel report file to
    #[structopt(short = "s", long = "save-file", parse(from_os_str), default_value = &DEFAULT_PATH_TO_SAVE_FILE)]
    path_to_save_file: PathBuf,

    /// Number of threads to use to run the test
    #[structopt(short, long, parse(try_from_str = parse_none_zero_u8), default_value = "3")]
    thread_count: u8,

    /// If set, will run the program with single thread only (like NodeJS), the '--thread-count' flag will be ignored.
    #[structopt(long)]
    single_thread: bool,

    /// Prints additional debug information
    #[structopt(short = "D", long)]
    debug: bool,
}
/* #endregion */

// Example: Shaked-TODO make an example

fn main() -> Result<(), Box<dyn Error>> {
    let options = OptionalArguments::from_args();
    let runtime = if options.single_thread {
        Builder::new_current_thread().build()?
    } else {
        Builder::new_multi_thread().worker_threads(options.thread_count.into()).build()?
    };

    runtime.block_on(async move {
        if options.debug {
            println!("{:#?}", options);
        }
    });

    // let mut excel_generator = ExcelGenerator::new(
    //     options.path_to_save_file.to_str().ok_or("Invalid path to save file")?,
    //     options.json_path.to_str().ok_or("Invalid path to json file")?,
    //     &options.sample_interval,
    //     options.number_of_letters,
    //     options.depth,
    //     options.number_of_children
    // )?;
    // let raw_json = fs::read_to_string(options.json_path.as_path())
    //     .expect("Couldn't read the input JSON file");
    // let value_to_search: i64 = 2_000_000_000;
    // let value_to_search: Value = json!(value_to_search);

    // for count in 0..options.test_counter {
    //     /* #region Test preparations */
    //     let mut reporter = Report::new();
    //     let (main_sender, thread_reciver) = mpsc::channel();
    //     let (thread_sender, main_reciver) = mpsc::channel();
    //     let pc_usage_exporter_thread = thread::spawn(move ||
    //         pc_usage_exporter::main(
    //             thread_sender,
    //             thread_reciver,
    //             &options.sample_interval));
    //     /* #endregion */
        
    //     /* #region Testing */
    //     let title = String::from("Test Generating JSON");
    //     reporter.measure(&title, ||
    //         json_generator::Generator::generate_json(
    //             CHARACTER_POLL,
    //             options.number_of_letters,
    //             options.depth,
    //             options.number_of_children
    //         )
    //     )?;

    //     let title = String::from("Test Deserialize JSON");
    //     let json: Value = reporter.measure(&title, ||
    //         serde_json::from_str(&raw_json)
    //             .expect("Couldn't parse the input JSON")
    //     );

    //     let title = String::from("Test Iterate Iteratively");
    //     reporter.measure(&title, ||
    //         assert!(!breadth_first_search::run(&json, &value_to_search), "BFS the tree found value that shouldn't be in it: {}", value_to_search)
    //     );

    //     let title = String::from("Test Iterate Recursively");
    //     reporter.measure(&title, ||
    //         assert!(!depth_first_search::run(&json, &value_to_search), "DFS the tree found value that shouldn't be in it: {}", value_to_search)
    //     );

    //     let title = String::from("Test Serialize JSON");
    //     reporter.measure(&title, ||
    //         serde_json::to_string(&json)
    //     )?;
    //     /* #endregion */
        
    //     /* #region Getting PC Usage from other thread */
    //     main_sender.send(()).expect("Couldn't terminate PC usage thread");
    //     pc_usage_exporter_thread.join().expect("Couldn't join pc_usage_exporter_thread");
    //     let mut pc_usage = vec![];
    //     for received in main_reciver {
    //         pc_usage.push(received);
    //     }
    //     /* #endregion */

    //     excel_generator.append_worksheet(format!("Test {}", count + 1), reporter.get_measures(), &pc_usage)?;
    // }

    Ok(())
}
