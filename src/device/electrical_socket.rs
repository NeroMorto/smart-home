use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElectricalSocketState {
    On,
    Off,
}

impl Display for ElectricalSocketState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElectricalSocketState::On => write!(f, "On"),
            ElectricalSocketState::Off => write!(f, "Off"),
        }
    }
}
impl From<bool> for ElectricalSocketState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::On,
            false => Self::Off,
        }
    }
}

pub struct ElectricalSocket {
    state: ElectricalSocketState,
    power: f32,
}

impl ElectricalSocket {
    pub fn new(power: f32, state: ElectricalSocketState) -> Self {
        Self { state, power }
    }
    pub fn get_state(&self) -> ElectricalSocketState {
        self.state
    }

    pub fn toggle(&mut self) {
        match self.state {
            ElectricalSocketState::Off => self.state = ElectricalSocketState::On,
            ElectricalSocketState::On => self.state = ElectricalSocketState::Off,
        }
    }

    pub fn get_power(&self) -> f32 {
        match self.state {
            ElectricalSocketState::On => self.power,
            ElectricalSocketState::Off => 0.,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electrical_socket() {
        let mut socket = ElectricalSocket::new(1.0, ElectricalSocketState::Off);
        assert!(matches!(socket.get_state(), ElectricalSocketState::Off));
        assert_eq!(socket.get_power(), 0.0);
        socket.toggle();
        assert!(matches!(socket.get_state(), ElectricalSocketState::On));
        assert_eq!(socket.get_power(), 1.);
        socket.toggle();
        assert!(matches!(socket.get_state(), ElectricalSocketState::Off));
        assert_eq!(socket.get_power(), 0.0);
    }

    #[test]
    fn test_state_from_bool() {
        assert_eq!(ElectricalSocketState::from(true), ElectricalSocketState::On);
        assert_eq!(
            ElectricalSocketState::from(false),
            ElectricalSocketState::Off
        );
    }

    #[test]
    fn test_state_display() {
        assert_eq!(format!("{}", ElectricalSocketState::On), "On");
        assert_eq!(format!("{}", ElectricalSocketState::Off), "Off");
    }
}
