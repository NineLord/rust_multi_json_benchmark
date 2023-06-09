// #![allow(unused, dead_code)]
/* #region Imports */
// Standard
use std::{
    env,
    fs,
    error::Error,
    path::{Path, PathBuf},
    sync::Arc,
};

// 3rd Party
use home::home_dir;
use once_cell::sync::Lazy;
use structopt::StructOpt;
use serde_json::json;
use tokio::{runtime::Builder, task};

// Project
use rust_multi_json_benchmark::{test_json::{reporter::{REPORT_INSTANCE, ReportData}, run_test_loop::RunTestLoop, measurement::Measurement}};
use rust_multi_json_benchmark::test_json::{
    config::Configs,
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

fn parse_none_zero_u32(source: &str) -> Result<u32, Box<dyn Error>> {
    let num: u32 = source.parse()?;
    if num == 0 {
        Err("The number has to be none zero".into())
    } else {
        Ok(num)
    }
}

fn parse_none_zero_usize(source: &str) -> Result<usize, Box<dyn Error>> {
    let num: usize = source.parse()?;
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
    #[structopt(parse(try_from_str = parse_none_zero_u32), default_value = "5")]
    test_counter: u32,

    /// Absolute path to save the excel report file to
    #[structopt(short = "s", long = "save-file", parse(from_os_str), default_value = &DEFAULT_PATH_TO_SAVE_FILE)]
    path_to_save_file: PathBuf,

    /// Number of threads to use to run the test
    #[structopt(short, long, parse(try_from_str = parse_none_zero_usize))]
    thread_count: Option<usize>,

    /// If set, will run the program with single thread only (like NodeJS), the '--thread-count' flag will be ignored.
    #[structopt(long)]
    single_thread: bool,

    /// Prints additional debug information
    #[structopt(short = "D", long)]
    debug: bool,
}
/* #endregion */

// Example: clear ; cargo run --bin json_tester -- ../rust_json_benchmark/junk/config_4.json 10 -s ./junk/report_rust.xlsx

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let options = OptionalArguments::from_args();
    let runtime = if options.single_thread {
        Builder::new_current_thread()
            .enable_all()
            .build()
    } else {
        let mut runtime_builder = Builder::new_multi_thread();
        runtime_builder.enable_all();
        if let Some(thread_count) = options.thread_count {
            runtime_builder.worker_threads(thread_count);
        }
        runtime_builder.build()
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
        for counter in 1..=options.test_counter {
            let test_name = format!("Test {}", counter);
            let test_case = report
                .get(&test_name)
                .ok_or_else(|| format!("Report doesn't contain the test name: {}", test_name))?;
            excel_generator.append_worksheet(&test_name, test_case)?;
        }
    }

    Ok(())
}
