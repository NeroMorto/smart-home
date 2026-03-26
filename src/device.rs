pub mod electrical_socket;
pub mod socket_state;
pub mod static_electrical_socket;
pub mod static_thermometer;
pub mod tcp_electrical_socket;
pub mod thermometer;
pub mod udp_thermometer;

use crate::device::thermometer::Thermometer;
use crate::reportable_trait::Reportable;
pub use electrical_socket::ElectricalSocket;

#[derive(Debug)]
pub enum Device {
    Thermometer(Thermometer),
    ElectricalSocket(ElectricalSocket),
}

impl Reportable for Device {
    fn report(&self) {
        match self {
            Device::Thermometer(t) => {
                println!("Thermometer device: temperature = {}", t.get_temperature());
            }
            Device::ElectricalSocket(s) => {
                println!("Electrical socket device: power = {}", s.get_power());
            }
        }
    }
}

impl From<Thermometer> for Device {
    fn from(thermometer: Thermometer) -> Self {
        Self::Thermometer(thermometer)
    }
}

impl From<ElectricalSocket> for Device {
    fn from(electric_socket: ElectricalSocket) -> Self {
        Self::ElectricalSocket(electric_socket)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::static_electrical_socket::StaticElectricalSocket;
    use crate::device::static_thermometer::StaticThermometer;
    #[test]
    fn test_into_device() {
        let _ = Device::from(Thermometer::new(Box::new(StaticThermometer::new(32.))));
        let _ = Device::from(ElectricalSocket::new(Box::new(
            StaticElectricalSocket::new(0., false.into()),
        )));
    }

    #[test]
    fn test_report() {
        let thermometer = Thermometer::new(Box::new(StaticThermometer::new(32.)));
        let electrical_socket =
            ElectricalSocket::new(Box::new(StaticElectricalSocket::new(0., false.into())));

        let thermometer_device: Device = thermometer.into();
        thermometer_device.report();

        let electrical_socket_device: Device = electrical_socket.into();
        electrical_socket_device.report();
    }
}
