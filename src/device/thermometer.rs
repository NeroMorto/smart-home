pub trait SmartThermometer: Send + Sync + std::fmt::Debug {
    fn get_temperature(&self) -> f32;
}
#[derive(Debug)]
pub struct Thermometer {
    pub inner: Box<dyn SmartThermometer>,
}

impl Thermometer {
    pub fn new(backend: Box<dyn SmartThermometer>) -> Self {
        Self { inner: backend }
    }
    pub fn get_temperature(&self) -> f32 {
        self.inner.get_temperature()
    }
}
