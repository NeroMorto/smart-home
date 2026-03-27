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
