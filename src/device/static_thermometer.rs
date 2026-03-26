use crate::device::thermometer::SmartThermometer;

#[derive(Debug)]
pub struct StaticThermometer {
    temperature: f32,
}

impl StaticThermometer {
    pub fn new(temperature: f32) -> Self {
        Self { temperature }
    }
}
impl SmartThermometer for StaticThermometer {
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::Thermometer;

    #[test]
    fn test_get_temperature() {
        let thermometer = Thermometer::new(Box::new(StaticThermometer::new(50.)));
        assert_eq!(thermometer.get_temperature(), 50.);
    }
}
