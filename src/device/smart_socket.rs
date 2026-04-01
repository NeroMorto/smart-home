use std::fmt::Debug;
pub mod backends;
pub mod socket_state;
use crate::{
    device::smart_socket::backends::static_electrical_socket::StaticElectricalSocket,
    reportable_trait::Reportable,
};
pub trait SocketBackend: Sync + Send + Debug {
    fn toggle(&mut self);
    fn get_power(&self) -> f32;
}
#[derive(Debug)]
pub struct SmartSocket {
    inner: Box<dyn SocketBackend>,
}

impl SmartSocket {
    pub fn new(backend: Box<dyn SocketBackend>) -> Self {
        Self { inner: backend }
    }

    pub fn get_power(&self) -> f32 {
        self.inner.get_power()
    }

    pub fn toggle(&mut self) {
        self.inner.toggle()
    }
}

impl Reportable for SmartSocket {
    fn report(&self) -> String {
        format!("SmartSocket | Power: {}", self.get_power())
    }
}

impl Default for SmartSocket {
    fn default() -> Self {
        Self::new(Box::new(StaticElectricalSocket::new(220., true.into())))
    }
}
