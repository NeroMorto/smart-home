pub mod electrical_socket;
pub mod thermometer;
pub use electrical_socket::{ElectricalSocket, ElectricalSocketState};
pub use thermometer::Thermometer;

pub enum Device {
    Thermometer(Thermometer),
    ElectricalSocket(ElectricalSocket),
}

impl Device {
    pub fn report(&self) {
        match self {
            Device::Thermometer(t) => {
                println!("Temperature: {}", t.get_temperature());
            }
            Device::ElectricalSocket(s) => {
                println!("State: {}. Power: {}", s.get_state(), s.get_power());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report() {
        let socket =
            Device::ElectricalSocket(ElectricalSocket::new(220., ElectricalSocketState::On));
        socket.report();
        let thermometer = Device::Thermometer(Thermometer::new(22.));
        thermometer.report();
    }
}
