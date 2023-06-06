/* #region Imports */
// Standard
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

// 3rd-Party
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

// Project
use super::measurement::Measurement;
use super::measurement_types::MeasurementType;
/* #endregion */

pub static REPORT_INSTANCE: Lazy<RwLock<Report>> = Lazy::new(|| RwLock::new(Report::new()));

pub type ReportData = HashMap<String, HashMap<Arc<String>, HashMap<MeasurementType, Measurement>>>;

pub struct Report {
    measurement_duration: ReportData,
}

impl Report {
    fn new() -> Report {
        Report {
            measurement_duration: HashMap::new()
        }
    }

    pub fn start_measure(&mut self, test_count: String, json_name: Arc<String>, measurement_type: MeasurementType) {
        self.measurement_duration
                .entry(test_count).or_default()
                .entry(json_name).or_default()
                .insert(measurement_type, Measurement::new());
    }

    pub fn finish_measure(&mut self, test_count: &str, json_name: Arc<String>, measurement_type: &MeasurementType) -> Result<(), String> {
        self.measurement_duration
            .get_mut(test_count).ok_or_else(|| format!("Can't find test count: {}", test_count))?
            .get_mut(&json_name).ok_or_else(|| format!("Can't find json name: {}", json_name))?
            .get_mut(measurement_type).ok_or_else(|| format!("Can't find measurement type: {:?}", measurement_type))?
            .set_finish_time();
        
        Ok(())
    }

    pub fn get_measures(&self) -> &ReportData {
        &self.measurement_duration
    }

    pub fn measure<F, R>(test_count: String, json_name: Arc<String>, measurement_type: MeasurementType, function: F) -> R
    where F: FnOnce() -> R {
        { REPORT_INSTANCE.blocking_write().start_measure(test_count.clone(), Arc::clone(&json_name), measurement_type.clone()); }
        let function_result = function();
        { REPORT_INSTANCE.blocking_write().finish_measure(&test_count, json_name, &measurement_type); }
        
        function_result
    }

    pub async fn async_measure<F: Future>(test_count: String, json_name: Arc<String>, measurement_type: MeasurementType, future: F) -> F::Output {
        {
            let mut reporter = REPORT_INSTANCE.write().await;
            reporter.start_measure(test_count.clone(), Arc::clone(&json_name), measurement_type.clone());
        }
        let function_result = future.await;
        {
            let mut reporter = REPORT_INSTANCE.write().await;
            reporter.finish_measure(&test_count, json_name, &measurement_type);
        }

        function_result
    }
}

#[cfg(test)]
mod tests {
    /* #region Imports */
    // Standard
    use std::time::Duration;

    // Project
    use super::*;
    /* #endregion */

    #[test]
    fn measure_sleep() {
        let test_case = String::from("Test 1");
        let json_name = Arc::new(String::from("Json 1"));
        let measurement_type = MeasurementType::GenerateJson;
        
        Report::measure(test_case.clone(), Arc::clone(&json_name), measurement_type.clone(), || {
            std::thread::sleep(Duration::from_millis(1000));
        });

        let duration;
        {
            let reporter = REPORT_INSTANCE.blocking_read();
            let measurements = reporter.get_measures();
            let test_map = measurements.get(&test_case).expect("No test map");
            let json_map = test_map.get(&json_name).expect("No json map");
            let measurement = json_map.get(&measurement_type).expect("No duration for measurement type");
            duration = measurement.get_duration().expect("Measurement haven't finished");
        }
        let duration = duration.as_millis();
        assert!((1000..2000).contains(&duration), "duration isn't in range: {}", duration);
    }

}