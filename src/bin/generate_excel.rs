#![allow(unused, unused_imports)]
/* #region Imports */
// Standard
use std::{
    error::Error,
    time::Duration,
    collections::HashMap
};

// 3rd Party
use xlsxwriter::{Workbook, XlsxError};

// Project
use rust_multi_json_benchmark::test_json::{excel_generator, pc_usage_exporter::PcUsage};
/* #endregion */

fn create_example_excel() -> Result<(), Box<dyn Error>> {
    let sample_interval = Duration::from_millis(50);
    let mut generator = excel_generator::ExcelGenerator::new(
        "/mnt/c/Users/Shaked/Documents/Mine/IdeaProjects/rust_multi_json_benchmark/junk/trying.xlsx",
        "json_path",
        &sample_interval,
        8, 10, 6)?;

    let mut measures = HashMap::new();
    measures.insert("Test Generating JSON", Duration::from_millis(1));
    measures.insert("Test Iterate Iteratively", Duration::from_millis(2));
    measures.insert("Test Iterate Recursively", Duration::from_millis(3));
    measures.insert("Test Deserialize JSON", Duration::from_millis(4));
    measures.insert("Test Serialize JSON", Duration::from_millis(5));
    let pc_usage = vec![
        PcUsage {cpu: 25.0, ram: 1500},
        PcUsage {cpu: 50.0, ram: 1700},
        PcUsage {cpu: 20.0, ram: 1800},
    ];
    generator.append_worksheet(String::from("Test 1"), &measures, &pc_usage)?;

    let mut measures = HashMap::new();
    measures.insert("Test Generating JSON", Duration::from_millis(2));
    measures.insert("Test Iterate Iteratively", Duration::from_millis(3));
    measures.insert("Test Iterate Recursively", Duration::from_millis(5));
    measures.insert("Test Deserialize JSON", Duration::from_millis(6));
    measures.insert("Test Serialize JSON", Duration::from_millis(8));
    let pc_usage = vec![
        PcUsage {cpu: 75.0, ram: 600},
        PcUsage {cpu: 50.0, ram: 200},
        PcUsage {cpu: 50.0, ram: 800},
    ];
    generator.append_worksheet(String::from("Test 2"), &measures, &pc_usage)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    create_example_excel()?;
    Ok(())
}
