#![allow(unused, dead_code)] // Shaked-TODO: delete this
/* #region Imports */
// Standard
use std::{
    env,
    fs,
    thread,
    time::{Duration, SystemTime},
    error::Error,
    path::{Path, PathBuf},
    sync::{mpsc, Arc},
    num::ParseIntError,
};

// 3rd Party
use home::home_dir;
use once_cell::sync::Lazy;
use structopt::StructOpt;
use serde_json::{ Value, json };
use tokio::{runtime::Builder, sync::RwLock, task::{self, JoinSet}, join};

// Project
use rust_multi_json_benchmark::{json_generator, test_json::{measurement_types::MeasurementType, reporter::{REPORT_INSTANCE, ReportData}, run_test_loop::RunTestLoop, excel_generator, measurement::Measurement}};
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
/* #endregion */

/* #region CLI Arguments */
fn parse_config(source: &str) -> Result<Configs, Box<dyn Error>> {
    let raw_config_file = fs::read_to_string(Path::new(source))?;
    let configs: Configs = serde_json::from_str(&raw_config_file)?;
    Ok(configs)
}

fn parse_none_zero_u8(source: &str) -> Result<u8, Box<dyn Error>> {
    let num: u8 = source.parse()?;
    if num == 0 {
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

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let options = OptionalArguments::from_args();
    let runtime = if options.single_thread {
        Builder::new_current_thread()
            .enable_all()
            .build()
    } else {
        Builder::new_multi_thread()
            .enable_all()
            .worker_threads(options.thread_count.into())
            .build()
    }.expect("Failed building the Runtime");

    runtime.block_on(async { async_main(options).await })
}

async fn async_main(mut options: OptionalArguments) -> Result<(), Box<dyn Error + Send + Sync>> {    
    if options.debug {
        println!("{:#?}", options);
    }

    /* #region Test preparations */
    let mut test_names = Vec::with_capacity(options.configs.len());

    for config in options.configs.iter_mut() {
        config.raw = Some(Arc::new(fs::read_to_string(&config.path)?));
        test_names.push(Arc::clone(&config.name));
    }

    let value_to_search: i64 = 2_000_000_000;
    let value_to_search = json!(value_to_search);
    let test_runner = Arc::new(RunTestLoop::new(options.test_counter, value_to_search));
    let mut task_handlers = Vec::with_capacity(options.configs.len());
    /* #endregion */

    /* #region Testing */
    let mut total_test_length = Measurement::new();
    for config in options.configs.iter() {
        let test_runner = Arc::clone(&test_runner);
        let json_name = Arc::clone(&config.name);
        let number_of_letters = config.number_of_letters;
        let depth = config.depth;
        let number_of_children = config.number_of_children;
        let raw_json = Arc::clone(config.raw.as_ref().expect("Config doesn't contain raw of the JSON file"));
        task_handlers.push(task::spawn(async move {
            test_runner.run_test(json_name, number_of_letters, depth, number_of_children, raw_json).await
        }));
    }
    for join_handler in task_handlers {
        join_handler.await??
    }
    total_test_length.set_finish_time();
    let total_test_length = total_test_length
        .get_duration()
        .expect("Didn't start measurement of the whole test");
    /* #endregion */

    if options.debug {
        let reporter = REPORT_INSTANCE.read().await;
        println!("{:#?}\nWhole test: {}", reporter.get_measures(), total_test_length.as_millis());
    }

    let mut excel_generator = ExcelGenerator::new(
        options.path_to_save_file.to_str().ok_or("Invalid path to save file")?,
        test_names,
        total_test_length,
        &options.configs
    )?;

    {
        let reporter = REPORT_INSTANCE.read().await;
        let report: &ReportData = reporter.get_measures();
        for (test_name, test_case) in report {
            excel_generator.append_worksheet(test_name, test_case)?;
        }
    }

    Ok(())
}
