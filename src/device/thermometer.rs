#[derive(Debug)]
pub struct Thermometer {
    pub temperature: f32,
}

impl Thermometer {
    pub fn new(temperature: f32) -> Self {
        Self { temperature }
    }
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

#[cfg(test)]
mod tests {
    use crate::device::Thermometer;

    #[test]
    fn test_get_temperature() {
        let thermometer = Thermometer::new(50.);
        assert_eq!(thermometer.get_temperature(), 50.);
    }
}
