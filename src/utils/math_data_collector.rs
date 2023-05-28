pub struct MathDataCollector {
    minimum: Option<f64>,
    maximum: Option<f64>,
    sum: f64,
    count: u64,
}

impl MathDataCollector {
    pub fn new() -> MathDataCollector {
        MathDataCollector {
            minimum: None,
            maximum: None,
            sum: 0.0,
            count: 0
        }
    }

    pub fn add(&mut self, data: f64) {
        self.sum += data;
        self.count += 1;
        self.minimum = match self.minimum {
            Some(minimum) => Some(f64::min(minimum, data)),
            None => Some(data),
        };
        self.maximum = match self.maximum {
            Some(maximum) => Some(f64::max(maximum, data)),
            None => Some(data),
        };
    }

    pub fn get_minimum(&self) -> Option<f64> {
        self.minimum
    }

    pub fn get_maximum(&self) -> Option<f64> {
        self.maximum
    }

    pub fn get_sum(&self) -> f64 {
        self.sum
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }

    pub fn get_average(&self) -> Option<f64> {
        match self.count {
            0 => None,
            count => Some(self.sum / (count as f64)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_values() {
        let mut data_collector = MathDataCollector::new();

        data_collector.add(1.0);
        data_collector.add(2.0);
        data_collector.add(3.0);
        data_collector.add(2.0);
        data_collector.add(1.0);
        data_collector.add(2.0);
        data_collector.add(3.0);
        data_collector.add(2.0);
        data_collector.add(1.0);

        assert_eq!(data_collector.get_maximum(), Some(3.0));
        assert_eq!(data_collector.get_minimum(), Some(1.0));
        assert_eq!(data_collector.get_sum(), 1.0 + 2.0 + 3.0 + 2.0 + 1.0 + 2.0 + 3.0 + 2.0 + 1.0);
        assert_eq!(data_collector.get_count(), 9);
        assert_eq!(data_collector.get_average(), Some((1.0 + 2.0 + 3.0 + 2.0 + 1.0 + 2.0 + 3.0 + 2.0 + 1.0) / 9.0));
    }
}