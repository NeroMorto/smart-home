mod electrical_socket;
mod thermometer;

use crate::reportable_trait::Reportable;
pub use electrical_socket::ElectricalSocket;
pub use thermometer::Thermometer;

#[derive(Debug)]
pub enum Device {
    Thermometer(Thermometer),
    ElectricalSocket(ElectricalSocket),
}

impl Reportable for Device {
    fn report(&self) {
        match self {
            Device::Thermometer(t) => {
                println!("Thermometer device: temperature = {}", t.temperature);
            }
            Device::ElectricalSocket(s) => {
                println!(
                    "Electrical socket device: state = {}, power = {}",
                    s.state, s.power
                );
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
    #[test]
    fn test_into_device() {
        let _ = Device::from(Thermometer::new(32.));
        let _ = Device::from(ElectricalSocket::new(0., false.into()));
    }

    #[test]
    fn test_report() {
        let thermometer = Thermometer::new(32.);
        let electrical_socket = ElectricalSocket::new(0., false.into());

        let thermometer_device: Device = thermometer.into();
        thermometer_device.report();

        let electrical_socket_device: Device = electrical_socket.into();
        electrical_socket_device.report();
    }
}
