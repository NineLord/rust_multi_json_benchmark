/* #region Imports */
// Standard
use std::{
    error::Error,
    time::Duration,
    collections::HashMap, sync::Arc, hash::Hash,
};
// 3rd Party
use xlsxwriter::{Workbook, XlsxError, Worksheet, Format, format::{FormatBorder, FormatAlignment, FormatVerticalAlignment}};
use strum::IntoEnumIterator;

// Project
use crate::utils::math_data_collector::MathDataCollector;
use super::{config::Configs, measurement_types::MeasurementType, measurement::Measurement};
/* #endregion */

pub struct ExcelGenerator<'a> {
    about_information: &'a Configs,
    workbook: Workbook,
    format_border: Format,
    format_border_center: Format,
    json_names: Vec<Arc<String>>,
    total_test_length: Duration,
    averages_per_jsons: HashMap<Arc<String>, HashMap<MeasurementType, MathDataCollector>>,
    averages_all_jsons: HashMap<MeasurementType, MathDataCollector>,
}

fn get_data_collectors_for_each_test() -> HashMap<MeasurementType, MathDataCollector> {
    let mut data_collectors = HashMap::new();
    for measurement_type in MeasurementType::iter() {
        data_collectors.insert(measurement_type, MathDataCollector::new());
    }
    data_collectors
}

impl <'a> ExcelGenerator<'a> {
    pub fn new(path_to_save_file: &'a str, json_names: Vec<Arc<String>>, total_test_length: Duration, configs: &'a Configs)
    -> Result<ExcelGenerator<'a>, Box<dyn Error + Send + Sync>> {
        let mut format_border = Format::new();
        format_border.set_border(FormatBorder::Thin);

        let mut format_border_center = Format::new();
        format_border_center.set_border(FormatBorder::Thin);
        format_border_center.set_align(FormatAlignment::Center);
        format_border_center.set_vertical_align(FormatVerticalAlignment::VerticalTop);

        let mut averages_per_jsons = HashMap::new();
        for json_name in json_names.iter() {
            let json_name = Arc::clone(json_name);
            averages_per_jsons.insert(json_name, get_data_collectors_for_each_test());
        }

        Ok(ExcelGenerator {
            about_information: configs,
            workbook: Workbook::new(path_to_save_file)?,
            format_border,
            format_border_center,
            json_names,
            total_test_length,
            averages_per_jsons,
            averages_all_jsons: get_data_collectors_for_each_test(),
        })
    }

    /* #region Adding Data */
    pub fn append_worksheet(&mut self, worksheet_name: &String, measures: &HashMap<Arc<String>, HashMap<MeasurementType, Measurement>>)
    -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut worksheet = self.workbook.add_worksheet(Some(worksheet_name))?;

        Ok(())
    }
    /* #endregion */

    fn close(&mut self) -> Result<(), XlsxError> {
        // self.add_average_worksheet()?;
        // self.add_about_worksheet()?;

        Ok(())
    }
}

impl <'a> Drop for ExcelGenerator<'a> {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}


