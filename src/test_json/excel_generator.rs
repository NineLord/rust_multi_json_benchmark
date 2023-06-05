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
        // ExcelGenerator::add_data(&mut worksheet, &self.json_names, measures,
        //     &mut self.averages_per_jsons, &mut self.averages_all_jsons,
        //     &self.format_border, &self.format_border_center)?;
        let mut test_data_collectors = get_data_collectors_for_each_test();
        let mut current_row = 0;

        for json_name in &self.json_names {
            let test_data = measures
                .get(json_name)
                .ok_or_else(|| format!("Given database doesn't have a the JSON name: {}", json_name))?;

            worksheet.merge_range(
                current_row,
                0,
                current_row,
                1,
                json_name,
                Some(&self.format_border_center))?;
            current_row += 1;

            let mut json_data_collector = MathDataCollector::new();

            current_row = ExcelGenerator::add_test_data(MeasurementType::GenerateJson, "Generating JSON", &mut worksheet, current_row, test_data, json_name, &self.format_border, &self.format_border_center, &mut self.averages_per_jsons, &mut self.averages_all_jsons, &mut json_data_collector, &mut test_data_collectors)?;


            // let measurement_type = MeasurementType::GenerateJson;
            // let value = test_data
            //     .get(&measurement_type)
            //     .ok_or_else(|| format!("Given database doesn't have a measurement type: {:?}", measurement_type))?
            //     .get_duration()
            //     .ok_or_else(|| String::from("Given database measurement's didn't finish running"))?
            //     .as_millis() as f64;
            // worksheet.write_string(current_row, 1, "TODO", Some(&self.format_border))?;
            // worksheet.write_number(current_row, 2, value, Some(&self.format_border_center))?;
            // json_data_collector.add(value);
            // self.averages_per_jsons
            //     .get_mut(json_name)
            //     .ok_or_else(|| format!("averages_per_jsons doesn't have the given JSON name: {}", json_name))?
            //     .get_mut(&measurement_type)
            //     .ok_or_else(|| format!("averages_per_jsons doesn't have the given measurement type: {:?}", measurement_type))?
            //     .add(value);
            // self.averages_all_jsons
            //     .get_mut(&measurement_type)
            //     .ok_or_else(|| format!("averages_all_jsons doesn't have the given measurement type: {:?}", measurement_type))?
            //     .add(value);
            // test_data_collectors
            //     .get_mut(&measurement_type)
            //     .ok_or_else(|| format!("test_data_collectors doesn't have the given measurement type: {:?}", measurement_type))?
            //     .add(value);
            // current_row += 1;
            
        }
        Ok(())
    }

    // fn add_test_data( // Shaked-TODO: delete this
    //     &mut self,
    //     worksheet: &mut Worksheet,
    //     current_row: u32,
    //     test_data: &HashMap<MeasurementType, Measurement>,
    //     json_data_collector: &mut MathDataCollector,
    //     test_data_collectors: &mut HashMap<MeasurementType, MathDataCollector>,
    //     json_name: &String,
    //     measurement_type: MeasurementType
    // ) -> Result<u32, Box<dyn Error + Send + Sync>> {
    //     let value = test_data
    //         .get(&measurement_type)
    //         .ok_or_else(|| format!("Given database doesn't have a measurement type: {:?}", measurement_type))?
    //         .get_duration()
    //         .ok_or_else(|| String::from("Given database measurement's didn't finish running"))?
    //         .as_millis() as f64;
    //     worksheet.write_string(current_row, 1, "TODO", Some(&self.format_border))?;
    //     worksheet.write_number(current_row, 2, value, Some(&self.format_border_center))?;
    //     json_data_collector.add(value);
    //     self.averages_per_jsons
    //         .get_mut(json_name)
    //         .ok_or_else(|| format!("averages_per_jsons doesn't have the given JSON name: {}", json_name))?
    //         .get_mut(&measurement_type)
    //         .ok_or_else(|| format!("averages_per_jsons doesn't have the given measurement type: {:?}", measurement_type))?
    //         .add(value);
    //     self.averages_all_jsons
    //         .get_mut(&measurement_type)
    //         .ok_or_else(|| format!("averages_all_jsons doesn't have the given measurement type: {:?}", measurement_type))?
    //         .add(value);
    //     test_data_collectors
    //         .get_mut(&measurement_type)
    //         .ok_or_else(|| format!("test_data_collectors doesn't have the given measurement type: {:?}", measurement_type))?
    //         .add(value);
    //     Ok(current_row + 1)
    // }

    #[allow(clippy::too_many_arguments)]
    fn add_test_data(
        measurement_type: MeasurementType,
        title: &'static str,
        worksheet: &mut Worksheet,
        current_row: u32,
        test_data: &HashMap<MeasurementType, Measurement>,
        json_name: &String,
        format_border: &Format,
        format_border_center: &Format,
        averages_per_jsons: &mut HashMap<Arc<String>, HashMap<MeasurementType, MathDataCollector>>,
        averages_all_jsons: &mut HashMap<MeasurementType, MathDataCollector>,
        json_data_collector: &mut MathDataCollector,
        test_data_collectors: &mut HashMap<MeasurementType, MathDataCollector>,
    ) -> Result<u32, Box<dyn Error + Send + Sync>> {
        let value = test_data
            .get(&measurement_type)
            .ok_or_else(|| format!("Given database doesn't have a measurement type: {:?}", measurement_type))?
            .get_duration()
            .ok_or_else(|| String::from("Given database measurement's didn't finish running"))?
            .as_millis() as f64;
        worksheet.write_string(current_row, 0, title, Some(format_border))?;
        worksheet.write_number(current_row, 1, value, Some(format_border_center))?;
        json_data_collector.add(value);
        averages_per_jsons
            .get_mut(json_name)
            .ok_or_else(|| format!("averages_per_jsons doesn't have the given JSON name: {}", json_name))?
            .get_mut(&measurement_type)
            .ok_or_else(|| format!("averages_per_jsons doesn't have the given measurement type: {:?}", measurement_type))?
            .add(value);
        averages_all_jsons
            .get_mut(&measurement_type)
            .ok_or_else(|| format!("averages_all_jsons doesn't have the given measurement type: {:?}", measurement_type))?
            .add(value);
        test_data_collectors
            .get_mut(&measurement_type)
            .ok_or_else(|| format!("test_data_collectors doesn't have the given measurement type: {:?}", measurement_type))?
            .add(value);

        Ok(current_row + 1)
    }

    fn add_data( // Shaked-TODO: delete this
        worksheet: &mut Worksheet,
        json_names: &Vec<Arc<String>>,
        measures: &HashMap<Arc<String>, HashMap<MeasurementType, Measurement>>,
        averages_per_jsons: &mut HashMap<Arc<String>, HashMap<MeasurementType, MathDataCollector>>,
        averages_all_jsons: &mut HashMap<MeasurementType, MathDataCollector>,
        format_border: &Format,
        format_border_center: &Format,
    )
    -> Result<(), Box<dyn Error + Send + Sync>> {
        let test_data_collectors = get_data_collectors_for_each_test();
        let mut current_row = 0;

        for json_name in json_names {
            let test_data = measures
                .get(json_name)
                .ok_or_else(|| format!("Given database doesn't have a the JSON name: {}", json_name))?;

            worksheet.merge_range(
                current_row,
                0,
                current_row,
                1,
                json_name,
                Some(format_border_center))?;
            current_row += 1;

            let mut json_data_collector = MathDataCollector::new();
            
            let measurement_type = MeasurementType::GenerateJson;
            let value = test_data
                .get(&measurement_type)
                .ok_or_else(|| format!("Given database doesn't have a measurement type: {:?}", measurement_type))?
                .get_duration()
                .ok_or_else(|| String::from("Given database measurement's didn't finish running"))?
                .as_millis() as f64;
            worksheet.write_string(current_row, 1, "TODO", Some(format_border))?;
            worksheet.write_number(current_row, 2, value, Some(format_border_center))?;
            json_data_collector.add(value);
            averages_per_jsons
                .get_mut(json_name)
                .ok_or_else(|| String::from("TODO"))?
                .get_mut(&measurement_type)
                .ok_or_else(|| String::from("TODO"))?
                .add(value);
        }

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


