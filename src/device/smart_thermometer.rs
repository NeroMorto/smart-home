use crate::{
    device::smart_thermometer::backends::static_thermometer::StaticThermometer,
    reportable_trait::Reportable,
};
pub mod backends;
pub trait ThermometerBackend: Send + Sync + std::fmt::Debug {
    fn get_temperature(&self) -> f32;
}
#[derive(Debug)]
pub struct SmartThermometer {
    pub inner: Box<dyn ThermometerBackend>,
}

impl SmartThermometer {
    pub fn new(backend: Box<dyn ThermometerBackend>) -> Self {
        Self { inner: backend }
    }
    pub fn get_temperature(&self) -> f32 {
        self.inner.get_temperature()
    }
}

impl Reportable for SmartThermometer {
    fn report(&self) -> String {
        format!("Thermometer | Themperature: {}", self.get_temperature())
    }
}
impl Default for SmartThermometer {
    fn default() -> Self {
        Self {
            inner: Box::new(StaticThermometer::new(42.)),
        }
    }
}
