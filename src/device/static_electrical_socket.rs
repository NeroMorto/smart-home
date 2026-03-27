use crate::device::electrical_socket::SmartSocket;
use crate::device::socket_state::ElectricalSocketState;

#[derive(Debug)]
pub struct StaticElectricalSocket {
    pub state: ElectricalSocketState,
    pub power: f32,
}

impl StaticElectricalSocket {
    pub fn new(power: f32, state: ElectricalSocketState) -> Self {
        Self { state, power }
    }
    pub fn get_state(&self) -> ElectricalSocketState {
        self.state
    }
}

impl SmartSocket for StaticElectricalSocket {
    fn toggle(&mut self) {
        match self.state {
            ElectricalSocketState::Off => self.state = ElectricalSocketState::On,
            ElectricalSocketState::On => self.state = ElectricalSocketState::Off,
        }
    }

    fn get_power(&self) -> f32 {
        match self.state {
            ElectricalSocketState::On => self.power,
            ElectricalSocketState::Off => 0.,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::device::electrical_socket::*;
    use crate::device::socket_state::ElectricalSocketState;
    use crate::device::static_electrical_socket::StaticElectricalSocket;

    #[test]
    fn test_get_state() {
        assert!(matches!(
            StaticElectricalSocket::new(1.0, ElectricalSocketState::Off).get_state(),
            ElectricalSocketState::Off
        ));
        assert!(matches!(
            StaticElectricalSocket::new(1.0, ElectricalSocketState::On).get_state(),
            ElectricalSocketState::On
        ));
    }

    #[test]
    fn test_get_power() {
        assert_eq!(
            StaticElectricalSocket::new(220.0, ElectricalSocketState::Off).get_power(),
            0.
        );
        assert_eq!(
            StaticElectricalSocket::new(220.0, ElectricalSocketState::On).get_power(),
            220.
        );
    }
    #[test]
    fn test_toggle() {
        let mut socket = StaticElectricalSocket::new(220.0, ElectricalSocketState::Off);
        assert!(matches!(socket.get_state(), ElectricalSocketState::Off));
        assert_eq!(socket.get_power(), 0.);

        socket.toggle();

        assert!(matches!(socket.get_state(), ElectricalSocketState::On));
        assert_eq!(socket.get_power(), 220.);

        socket.toggle();

        assert!(matches!(socket.get_state(), ElectricalSocketState::Off));
        assert_eq!(socket.get_power(), 0.);
    }

    #[test]
    fn test_display_state() {
        assert_eq!(ElectricalSocketState::Off.to_string(), "Off");
        assert_eq!(ElectricalSocketState::On.to_string(), "On");
    }
}
