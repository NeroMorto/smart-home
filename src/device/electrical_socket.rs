use std::fmt::Debug;

pub trait SmartSocket: Sync + Send + Debug {
    fn toggle(&mut self);
    fn get_power(&self) -> f32;
}

#[derive(Debug)]
pub struct ElectricalSocket {
    inner: Box<dyn SmartSocket>,
}

impl ElectricalSocket {
    pub fn new(backend: Box<dyn SmartSocket>) -> Self {
        Self { inner: backend }
    }

    pub fn get_power(&self) -> f32 {
        self.inner.get_power()
    }

    pub fn toggle(&mut self) {
        self.inner.toggle()
    }
}
