pub mod smart_socket;
pub mod smart_thermometer;

use crate::device::smart_socket::SmartSocket;
use crate::device::smart_thermometer::SmartThermometer;
use crate::reportable_trait::Reportable;

#[derive(Debug)]
pub enum Device {
    Thermometer(SmartThermometer),
    SmartSocket(SmartSocket),
}

impl Default for Device {
    fn default() -> Self {
        Device::Thermometer(SmartThermometer::default())
    }
}

impl Reportable for Device {
    fn report(&self) -> String {
        match self {
            Device::Thermometer(t) => {
                format!("Thermometer device: temperature = {}", t.get_temperature())
            }
            Device::SmartSocket(s) => {
                format!("Electrical socket device: power = {}", s.get_power())
            }
        }
    }
}

impl From<SmartThermometer> for Device {
    fn from(thermometer: SmartThermometer) -> Self {
        Self::Thermometer(thermometer)
    }
}

impl From<SmartSocket> for Device {
    fn from(electric_socket: SmartSocket) -> Self {
        Self::SmartSocket(electric_socket)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{
        smart_socket::backends::static_electrical_socket::StaticElectricalSocket,
        smart_thermometer::backends::static_thermometer::StaticThermometer,
    };
    #[test]
    fn test_into_device() {
        let _ = Device::from(SmartThermometer::new(Box::new(StaticThermometer::new(32.))));
        let _ = Device::from(SmartSocket::new(Box::new(StaticElectricalSocket::new(
            0.,
            false.into(),
        ))));
    }

    #[test]
    fn test_report() {
        let thermometer = SmartThermometer::new(Box::new(StaticThermometer::new(32.)));
        let electrical_socket =
            SmartSocket::new(Box::new(StaticElectricalSocket::new(0., false.into())));

        let thermometer_device: Device = thermometer.into();
        thermometer_device.report();

        let electrical_socket_device: Device = electrical_socket.into();
        electrical_socket_device.report();
    }
}
