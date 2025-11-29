mod device;
mod room;

pub use device::{Device, ElectricalSocket, ElectricalSocketState, Thermometer};
pub use room::Room;

type Rooms = [Room; 1];
pub struct SmartHome {
    rooms: Rooms,
}

impl SmartHome {
    pub fn new(rooms: Rooms) -> Self {
        Self { rooms }
    }
    pub fn get_room(&self, index: usize) -> &Room {
        self.check_room_index_bounds(index);
        &self.rooms[index]
    }
    pub fn get_room_mut(&mut self, index: usize) -> &mut Room {
        self.check_room_index_bounds(index);
        &mut self.rooms[index]
    }

    pub fn report(&self) {
        println!("Smart Home report");
        self.rooms.iter().for_each(|r| r.report());
    }

    fn check_room_index_bounds(&self, room_index: usize) {
        let bounds = self.rooms.len();
        if room_index >= bounds {
            panic!("Room index `{room_index}` out of bounds `{bounds}`",);
        }
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    #[test]
    fn test_smart_home() {
        let smart_home = SmartHome::new([Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ])]);
        smart_home.report();
    }

    #[test]
    fn test_get_room() {
        let smart_home = SmartHome::new([Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ])]);
        let room = smart_home.get_room(0);
        if let Device::ElectricalSocket(e) = room.get_device(0) {
            assert!(matches!(e.get_state(), ElectricalSocketState::On));
            assert_eq!(e.get_power(), 120.);
            return;
        }
        unreachable!()
    }

    #[test]
    #[should_panic(expected = "Room index `2` out of bounds `1`")]
    fn test_get_missing_room() {
        SmartHome::new([Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ])])
        .get_room(2);
    }

    #[test]
    fn test_get_room_mut() {
        let mut smart_home = SmartHome::new([Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::Off)),
            Device::Thermometer(Thermometer::new(34.0)),
        ])]);
        let room = smart_home.get_room_mut(0);
        if let Device::ElectricalSocket(s) = room.get_device(0) {
            assert!(matches!(s.get_state(), ElectricalSocketState::Off));
            assert_eq!(s.get_power(), 0.);
        } else {
            unreachable!();
        }
        if let Device::ElectricalSocket(s) = room.get_device_mut(0) {
            assert!(matches!(s.get_state(), ElectricalSocketState::Off));
            assert_eq!(s.get_power(), 0.);
            s.toggle();
            assert!(matches!(s.get_state(), ElectricalSocketState::On));
            assert_eq!(s.get_power(), 120.);
        } else {
            unreachable!();
        }
    }

    #[test]
    #[should_panic(expected = "Room index `2` out of bounds `1`")]
    fn test_get_missing_room_mut() {
        SmartHome::new([Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ])])
        .get_room_mut(2);
    }

    #[test]
    fn test_report() {
        SmartHome::new([Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ])])
        .report();
    }
}
