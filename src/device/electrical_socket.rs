use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug)]
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
#[derive(Debug)]
pub struct ElectricalSocket {
    pub state: ElectricalSocketState,
    pub power: f32,
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
    fn test_get_state() {
        assert!(matches!(
            ElectricalSocket::new(1.0, ElectricalSocketState::Off).get_state(),
            ElectricalSocketState::Off
        ));
        assert!(matches!(
            ElectricalSocket::new(1.0, ElectricalSocketState::On).get_state(),
            ElectricalSocketState::On
        ));
    }

    #[test]
    fn test_get_power() {
        assert_eq!(
            ElectricalSocket::new(220.0, ElectricalSocketState::Off).get_power(),
            0.
        );
        assert_eq!(
            ElectricalSocket::new(220.0, ElectricalSocketState::On).get_power(),
            220.
        );
    }
    #[test]
    fn test_toggle() {
        let mut socket = ElectricalSocket::new(220.0, ElectricalSocketState::Off);
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
