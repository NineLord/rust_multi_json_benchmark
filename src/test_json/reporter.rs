use std::sync::Arc;
/* #region Imports */
// Standard
use std::time::{SystemTime, Duration};
use std::collections::HashMap;
use std::future::Future;
use once_cell::sync::Lazy;
use tokio::sync::{RwLock, RwLockReadGuard};

// Project
use super::measurement_types::MeasurementType;
/* #endregion */

pub static REPORT_INSTANCE: Lazy<Arc<RwLock<Report>>> = Lazy::new(|| {
    Arc::new(RwLock::new(Report::new()))
});

pub type ReportData <'a> = HashMap<String, HashMap<&'a str, HashMap<MeasurementType, Duration>>>;

pub struct Report<'a> {
    measurement_duration: ReportData<'a>,
}

impl<'a> Report<'a> {
    fn new() -> Report<'a> {
        Report {
            measurement_duration: HashMap::new()
        }
    }

    pub fn measure<F, R>(&mut self, test_count: String, json_name: &'a str, measurement_type: MeasurementType, function: F) -> R
    where F: FnOnce() -> R {
        let start_time = SystemTime::now();
        let function_result = function();
        let finish_time = SystemTime::now();

        let duration = finish_time.duration_since(start_time).expect("Start time should be earlier to finish time");
        {
            self.measurement_duration
                .entry(test_count).or_default()
                .entry(json_name).or_default()
                .insert(measurement_type, duration);
        }
        
        function_result
    }

    pub async fn async_measure<F, FutureR, R>(&mut self, test_count: String, json_name: &'a str, measurement_type: MeasurementType, function: F) -> R
    where F: FnOnce() -> FutureR,
    FutureR: Future<Output = R> {
        let start_time = SystemTime::now();
        let function_result = function().await;
        let finish_time = SystemTime::now();

        let duration = finish_time.duration_since(start_time).expect("Start time should be earlier to finish time");
        {
            self.measurement_duration
                .entry(test_count).or_default()
                .entry(json_name).or_default()
                .insert(measurement_type, duration);
        }

        function_result
    }

    pub fn get_measures(&self) -> &ReportData<'a> {
        &self.measurement_duration
    }
}

#[cfg(test)]
mod tests {
    /* #region Imports */
    // Project
    use super::*;
    /* #endregion */

    #[test]
    fn measure_sleep() {
        let mut reporter = Report::new();
        let test_case = "Test 1";
        let json_name = "Json 1";
        let measurement_type = MeasurementType::GenerateJson;
        
        reporter.measure(String::from(test_case), json_name, measurement_type.clone(), || {
            std::thread::sleep(Duration::from_millis(1000));
        });

        let measurements = reporter.get_measures();
        let test_map = measurements.get(test_case).expect("No test map");
        let json_map = test_map.get(json_name).expect("No json map");
        let duration = json_map.get(&measurement_type).expect("No duration for measurement type");
        let duration = duration.as_millis();
        assert!((1000..2000).contains(&duration), "duration isn't in range: {}", duration);
    }
}