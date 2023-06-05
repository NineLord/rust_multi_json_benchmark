/* #region Imports */
// Standard
use std::{
    error::Error,
    time::Duration,
    collections::HashMap, sync::Arc,
};

// 3rd Party
use xlsxwriter::{Workbook, XlsxError, Worksheet, Format, format::{FormatBorder, FormatAlignment, FormatVerticalAlignment}};

// Project
use crate::utils::math_data_collector::MathDataCollector;

use super::config::Configs;
/* #endregion */

struct MathDataCollectors {
    average_generating_jsons: MathDataCollector,
    average_iterating_jsons_iteratively: MathDataCollector,
    average_iterating_jsons_recursively: MathDataCollector,
    average_deserializing_jsons: MathDataCollector,
    average_serializing_jsons: MathDataCollector,
    total_average_cpu: MathDataCollector,
    total_average_ram: MathDataCollector,
}

pub struct ExcelGenerator<'a> {
    about_information: &'a Configs,
    workbook: Workbook,
    format_border: Format,
    format_border_center: Format,
    json_names: Vec<Arc<String>>,
    worksheet_names: Vec<String>,
    // math_data_collectors: MathDataCollectors,
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

        Ok(ExcelGenerator {
            about_information: configs,
            workbook: Workbook::new(path_to_save_file)?,
            format_border,
            format_border_center,
            json_names,
            worksheet_names: vec!(),
        })
    }
    // pub fn new(path_to_save_file: &'a str, json_path: &'a str, sample_interval: &'a Duration, number_of_letters: u8, depth: u8, number_of_children: u8) ->
    //     Ok(ExcelGenerator {
    //         about_information: AboutInformation {
    //             json_path,
    //             sample_interval,
    //             number_of_letters,
    //             depth,
    //             number_of_children,
    //         },
    //         workbook: Workbook::new(path_to_save_file)?,
    //         format_border,
    //         format_border_center,
    //         worksheet_names: vec![],
    //         math_data_collectors: MathDataCollectors {
    //             average_generating_jsons: MathDataCollector::new(),
    //             average_iterating_jsons_iteratively: MathDataCollector::new(),
    //             average_iterating_jsons_recursively: MathDataCollector::new(),
    //             average_deserializing_jsons: MathDataCollector::new(),
    //             average_serializing_jsons: MathDataCollector::new(),
    //             total_average_cpu: MathDataCollector::new(),
    //             total_average_ram: MathDataCollector::new(),
    //         },
    //     })
    // }

    /* #region Adding Data */
    pub fn append_worksheet(&mut self, worksheet_name: String, measures: &HashMap<&str, Duration>) -> Result<(), Box<dyn Error>> {
        self.worksheet_names.push(worksheet_name);
        let worksheet_name = self.worksheet_names.last().ok_or("Couldn't get the worksheet_name")?;
        let mut worksheet = self.workbook.add_worksheet(Some(worksheet_name))?;

    //     ExcelGenerator::generate_titles(&mut worksheet, &self.format_border, &self.format_border_center)?;
    //     #[allow(unused)]
    //     let (column_cpu_usage, column_ram_usage) =
    //         ExcelGenerator::add_data(&mut self.math_data_collectors, &mut worksheet, measures, pc_usage, &self.format_border_center)?;

        Ok(())
    }

    // fn generate_titles(worksheet: &mut Worksheet, format_borader: &Format, format_borader_center: &Format) -> Result<(), XlsxError> {
    //     let format_borader = Some(format_borader);
    //     let format_borader_center = Some(format_borader_center);

    //     /* #region Column 1 */
    //     /* #region Table 1 */

    //     worksheet.write_string(0, 0, "Title", format_borader_center)?;
    //     worksheet.write_string(1, 0, "Generating JSON", format_borader)?;
    //     worksheet.write_string(2, 0, "Iterating JSON Iteratively - BFS", format_borader)?;
    //     worksheet.write_string(3, 0, "Iterating JSON Recursively - DFS", format_borader)?;
    //     worksheet.write_string(4, 0, "Deserializing JSON", format_borader)?;
    //     worksheet.write_string(5, 0, "Serializing JSON", format_borader)?;
    //     worksheet.write_string(6, 0, "Total", format_borader)?;
        
    //     /* #endregion */

    //     /* #region Table 2 */
    //     worksheet.write_string(8, 0, "Average CPU (%)", format_borader)?;
    //     worksheet.write_string(9, 0, "Average RAN (MB)", format_borader)?;
    //     /* #endregion */
    //     /* #endregion */

    //     /* #region Column 2 */
    //     worksheet.write_string(0, 1, "Time (ms)", format_borader_center)?;
    //     /* #endregion */

    //     /* #region Column 4 */
    //     worksheet.write_string(0, 3, "CPU (%)", format_borader_center)?;
    //     /* #endregion */

    //     /* #region Column 5 */
    //     worksheet.write_string(0, 4, "RAM (MB)", format_borader_center)?;
    //     /* #endregion */

    //     Ok(())
    // }

    // fn add_data(
    //     math_data_collectors: &mut MathDataCollectors,
    //     worksheet: &mut Worksheet,
    //     measures: &HashMap<&str, Duration>,
    //     pc_usage: &[PcUsage],
    //     format_borader_center: &Format) ->
    // Result<(MathDataCollector, MathDataCollector), Box<dyn Error>> {
    //     let format_borader_center = Some(format_borader_center);

    //     let mut row_total = MathDataCollector::new();
    //     let mut column_cpu_usage = MathDataCollector::new();
    //     let mut column_ram_usage = MathDataCollector::new();

    //     /* #region JSON Manipulations */
    //     for (test_name, test_result) in measures {
    //         let milliseconds = test_result.as_millis() as f64;
    //         match *test_name {
    //             "Test Generating JSON" => {
    //                 worksheet.write_number(1, 1, milliseconds, format_borader_center)?;
    //                 math_data_collectors.average_generating_jsons.add(milliseconds);
    //             },
    //             "Test Iterate Iteratively" => {
    //                 worksheet.write_number(2, 1, milliseconds, format_borader_center)?;
    //                 math_data_collectors.average_iterating_jsons_iteratively.add(milliseconds);
    //             },
    //             "Test Iterate Recursively" => {
    //                 worksheet.write_number(3, 1, milliseconds, format_borader_center)?;
    //                 math_data_collectors.average_iterating_jsons_recursively.add(milliseconds);
    //             },
    //             "Test Deserialize JSON" => {
    //                 worksheet.write_number(4, 1, milliseconds, format_borader_center)?;
    //                 math_data_collectors.average_deserializing_jsons.add(milliseconds);
    //             },
    //             "Test Serialize JSON" => {
    //                 worksheet.write_number(5, 1, milliseconds, format_borader_center)?;
    //                 math_data_collectors.average_serializing_jsons.add(milliseconds);
    //             },
    //             _ => return Err(Box::from(format!("Invalid test type: {}", *test_name)))
    //         }
    //         row_total.add(milliseconds);
    //     }

    //     worksheet.write_number(6, 1, row_total.get_sum(), format_borader_center)?;
    //     /* #endregion */

    //     /* #region PC Usage */
    //     let mut current_row_number = 1;
    //     for PcUsage {cpu, ram} in pc_usage {
    //         let cpu = *cpu as f64;
    //         let ram = *ram as f64;
    //         worksheet.write_number(current_row_number, 3, cpu, format_borader_center)?;
    //         worksheet.write_number(current_row_number, 4, ram, format_borader_center)?;

    //         column_cpu_usage.add(cpu);
    //         column_ram_usage.add(ram);

    //         math_data_collectors.total_average_cpu.add(cpu);
    //         math_data_collectors.total_average_ram.add(ram);

    //         current_row_number += 1;
    //     }

    //     if let Some(average) = column_cpu_usage.get_average() {
    //         worksheet.write_number(8, 1, average, format_borader_center)?
    //     }

    //     if let Some(average) = column_ram_usage.get_average() {
    //         worksheet.write_number(9, 1, average, format_borader_center)?
    //     }
    //     /* #endregion */

    //     Ok((column_cpu_usage, column_ram_usage))
    // }
    // /* #endregion */

    // /* #region Add Summary Worksheet */
    // fn add_average_worksheet(&mut self) -> Result<(), XlsxError> {
    //     let mut worksheet = self.workbook.add_worksheet(Some("Average"))?;
    //     ExcelGenerator::generate_average_titles(&mut worksheet, &self.format_border, &self.format_border_center)?;
    //     ExcelGenerator::add_average_data(&mut worksheet, &self.format_border_center, &self.math_data_collectors)?;

    //     Ok(())
    // }

    // fn generate_average_titles(worksheet: &mut Worksheet, format_border: &Format, format_border_center: &Format) -> Result<(), XlsxError> {
    //     let format_border = Some(format_border);
    //     let format_border_center = Some(format_border_center);

    //     /* #region Column 1 */
    //     /* #region Table 1 */
    //     worksheet.write_string(0, 0, "Title", format_border_center)?;
    //     worksheet.write_string(1, 0, "Average Generating JSONs", format_border)?;
    //     worksheet.write_string(2, 0, "Average Iterating JSONs Iteratively - BFS", format_border)?;
    //     worksheet.write_string(3, 0, "Average Iterating JSONs Recursively - DFS", format_border)?;
    //     worksheet.write_string(4, 0, "Average Deserializing JSONs", format_border)?;
    //     worksheet.write_string(5, 0, "Average Serializing JSONs", format_border)?;
    //     worksheet.write_string(6, 0, "Average Totals", format_border)?;
    //     /* #endregion */

    //     /* #region Table 2 */
    //     worksheet.write_string(8, 0, "Average Total CPU (%)", format_border)?;
    //     worksheet.write_string(9, 0, "Average Total RAN (MB)", format_border)?;
    //     /* #endregion */
    //     /* #endregion */

    //     /* #region Column 2 */
    //     worksheet.write_string(0, 1, "Time (ms)", format_border_center)?;
    //     /* #endregion */

    //     Ok(())
    // }
    
    // fn add_average_data(worksheet: &mut Worksheet, format_border_center: &Format, math_data_collectors: &MathDataCollectors) -> Result<(), XlsxError> {
    //     let format_border_center = Some(format_border_center);

    //     let mut total_averages = MathDataCollector::new();
    //     let mut cells = HashMap::new();
    //     cells.insert(1, &math_data_collectors.average_generating_jsons);
    //     cells.insert(2, &math_data_collectors.average_iterating_jsons_iteratively);
    //     cells.insert(3, &math_data_collectors.average_iterating_jsons_recursively);
    //     cells.insert(4, &math_data_collectors.average_deserializing_jsons);
    //     cells.insert(5, &math_data_collectors.average_serializing_jsons);
    //     for (cell_row, math_data_collector) in cells {
    //         if let Some(average) = math_data_collector.get_average() {
    //             worksheet.write_number(cell_row, 1, average, format_border_center)?;
    //             total_averages.add(average);
    //         }
    //     }
    //     worksheet.write_number(6, 1, total_averages.get_sum(), format_border_center)?;

    //     if let Some(total_average_cpu) = math_data_collectors.total_average_cpu.get_average() {
    //         worksheet.write_number(8, 1, total_average_cpu, format_border_center)?;
    //     }

    //     if let Some(total_average_ram) = math_data_collectors.total_average_ram.get_average() {
    //         worksheet.write_number(9, 1, total_average_ram, format_border_center)?;
    //     }

    //     Ok(())
    // }
    // /* #endregion */

    // /* #region Add About Worksheet */
    // fn add_about_worksheet(&mut self) -> Result<(), XlsxError> {
    //     let mut worksheet = self.workbook.add_worksheet(Some("About"))?;
    //     let format_border = Some(&self.format_border);
    //     let format_border_center = Some(&self.format_border_center);

    //     worksheet.write_string(0, 0, "Path to JSON to be tested on (Iterating/Deserializing/Serializing)", format_border)?;
    //     worksheet.write_string(0, 1, self.about_information.json_path, format_border_center)?;

    //     worksheet.write_string(1, 0, "CPU/RAM Sampling Interval (milliseconds)", format_border)?;
    //     worksheet.write_number(1, 1, self.about_information.sample_interval.as_millis() as f64, format_border_center)?;

    //     worksheet.write_string(2, 0, "Number of letters to generate for each node in the generated JSON tree", format_border)?;
    //     worksheet.write_number(2, 1, self.about_information.number_of_letters as f64, format_border_center)?;

    //     worksheet.write_string(3, 0, "Depth of the generated JSON tree", format_border)?;
    //     worksheet.write_number(3, 1, self.about_information.depth as f64, format_border_center)?;

    //     worksheet.write_string(4, 0, "Number of children each node in the generated JSON tree going to have", format_border)?;
    //     worksheet.write_number(4, 1, self.about_information.number_of_children as f64, format_border_center)?;

    //     Ok(())
    // }
    // /* #endregion */

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


