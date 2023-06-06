use std::time::{SystemTime, Duration};

#[derive(Debug)]
pub struct Measurement {
    start_time: SystemTime,
    duration: Option<Duration>
}

impl Measurement {
    pub fn new() -> Measurement {
        Measurement {
            start_time: SystemTime::now(),
            duration: None
        }
    }

    pub fn set_finish_time(&mut self) {
        self.duration = Some(
            SystemTime::now()
                .duration_since(self.start_time)
                .expect("Start time should be earlier to finish time")
        );
    }


    pub fn get_duration(&self) -> &Option<Duration> {
        &self.duration
    }
}

impl Default for Measurement {
    
    fn default() -> Self {
        Measurement::new()
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
        let mut measurement = Measurement::default();
        std::thread::sleep(Duration::from_millis(1000));
        measurement.set_finish_time();
        let result = measurement.get_duration();

        match result {
            Some(duration) => {
                let duration_millis = duration.as_millis();
                assert!((1000..2000).contains(&duration_millis), "duration isn't in range: {}", duration_millis);
            },
            None => panic!("No measurement was found")
        }

    }

    #[test]
    #[should_panic]
    fn must_set_finish_duration() {
        let measurement = Measurement::default();
        let result = measurement.get_duration();

        match result {
            Some(duration) => {
                let duration_millis = duration.as_millis();
                assert!(duration_millis == 0, "duration isn't in range: {}", duration_millis);
            },
            None => panic!("No measurement was found")
        }

    }
}