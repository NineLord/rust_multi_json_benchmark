/* #region Imports */
// Standard
use std::time::{SystemTime, Duration};
use std::collections::HashMap;
/* #endregion */

pub struct Report<'a> {
    measurement_duration: HashMap<&'a str, Duration>,
}

impl<'a> Report<'a> {
    pub fn new() -> Report<'a> {
        Report {
            measurement_duration: HashMap::new()
        }
    }

    pub fn measure<F, R>(&mut self, function_name: &'a str, function: F) -> R
    where F: Fn() -> R {
        let start_time = SystemTime::now();
        let function_result = function();
        let finish_time = SystemTime::now();
        let duration = finish_time.duration_since(start_time).expect("Start time should be earlier to finish time");
        self.measurement_duration.insert(function_name, duration);
        function_result
    }

    pub fn get_measures(&self) -> &HashMap<&str, Duration> {
        &self.measurement_duration
    }
}