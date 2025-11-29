pub struct Thermometer {
    temperature: f32,
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
    use super::*;

    #[test]
    fn test_thermometer() {
        let thermometer = Thermometer::new(22.);
        assert_eq!(thermometer.get_temperature(), 22.);
    }
}
