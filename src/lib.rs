use device::Device;
use reportable_trait::Reportable;
use room::Room;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub mod device;
pub mod reportable_trait;
pub mod room;
mod room_macro;

#[derive(Debug)]
pub struct SmartHome {
    rooms: HashMap<String, Room>,
}

impl Reportable for SmartHome {
    fn report(&self) {
        println!(
            "{} Smart Home: room count = {} {}",
            "=".repeat(5),
            self.rooms.len(),
            "=".repeat(5)
        );
        self.rooms.values().for_each(|room| room.report());
        println!("{} Report end {}", "=".repeat(10), "=".repeat(10));
    }
}

#[derive(Debug)]
pub enum SmartHomeError {
    RoomNotFound(String),
    RoomAlreadyExists(String),

    DeviceNotFound(String),
    DeviceAlreadyExists(String),
}

impl Display for SmartHomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartHomeError::RoomNotFound(error) => write!(f, "{}", error),
            SmartHomeError::RoomAlreadyExists(error) => write!(f, "{}", error),
            SmartHomeError::DeviceNotFound(error) => write!(f, "{}", error),
            SmartHomeError::DeviceAlreadyExists(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for SmartHomeError {}

impl SmartHome {
    pub fn new(rooms: Vec<(&str, Room)>) -> Self {
        Self {
            rooms: HashMap::from_iter(
                rooms
                    .into_iter()
                    .map(|(room_name, room)| (room_name.into(), room))
                    .collect::<Vec<(String, Room)>>(),
            ),
        }
    }
    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }
    pub fn get_room_mut(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    pub fn add_room(&mut self, name: &str, room: Room) -> Result<(), SmartHomeError> {
        if self.rooms.contains_key(name) {
            return Err(SmartHomeError::RoomAlreadyExists(name.into()));
        }
        self.rooms.insert(name.into(), room);
        Ok(())
    }

    pub fn get_device(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&Device, SmartHomeError> {
        if let Some(room) = self.rooms.get(room_name) {
            if let Some(device) = room.get_device(device_name) {
                return Ok(device);
            }
            return Err(SmartHomeError::DeviceNotFound(device_name.into()));
        }
        Err(SmartHomeError::RoomNotFound(room_name.into()))
    }

    pub fn remove_room(&mut self, name: &str) -> Result<(), SmartHomeError> {
        if self.rooms.remove(name).is_none() {
            return Err(SmartHomeError::RoomNotFound(name.into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::electrical_socket::ElectricalSocket;
    use crate::device::static_electrical_socket::StaticElectricalSocket;
    use crate::device::static_thermometer::StaticThermometer;
    use crate::device::thermometer::Thermometer;

    #[test]
    fn test_get_device() {
        let home = SmartHome::new(vec![(
            "Room",
            Room::new(vec![(
                "Unused socket",
                Thermometer::new(Box::new(StaticThermometer::new(32.))).into(),
            )]),
        )]);
        home.get_device("Room", "Unused socket").unwrap().report();
    }

    #[test]
    fn test_get_device_from_missing_room() {
        let home = SmartHome::new(vec![]);
        match home.get_device("Bathroom", "Some device").err().unwrap() {
            SmartHomeError::RoomNotFound(room_name) => {
                assert_eq!(room_name, "Bathroom")
            }
            _ => unreachable!("Unexpected error"),
        }
    }

    #[test]
    fn test_get_missing_device() {
        let home = SmartHome::new(vec![("Bathroom", Room::new(vec![]))]);
        match home.get_device("Bathroom", "Some device").err().unwrap() {
            SmartHomeError::DeviceNotFound(room_name) => {
                assert_eq!(room_name, "Some device")
            }
            _ => unreachable!("Unexpected error"),
        }
    }

    #[test]
    fn test_get_room() {
        let home = SmartHome::new(vec![("Room", Room::new(vec![]))]);
        let _ = home.get_room("Room").unwrap();
    }

    #[test]
    fn test_get_missing_room() {
        let home = SmartHome::new(vec![]);
        assert!(home.get_room("Missing room").is_none());
    }

    #[test]
    fn test_get_room_mut() {
        let mut home = SmartHome::new(vec![("Kitchen", Room::new(vec![]))]);
        let _ = home.get_room_mut("Kitchen").unwrap();
    }
    #[test]
    fn test_get_missing_room_mut() {
        let mut home = SmartHome::new(vec![]);
        assert!(home.get_room_mut("Missing room").is_none());
    }

    #[test]
    fn test_add_room() {
        let mut home = SmartHome::new(vec![]);
        assert_eq!(home.rooms.len(), 0);
        let _ = home.add_room("Room", Room::new(vec![]));
        assert_eq!(home.rooms.len(), 1);
        let err = home.add_room("Room", Room::new(vec![])).err().unwrap();
        match err {
            SmartHomeError::RoomAlreadyExists(_) => {}
            _ => unreachable!("Unexpected error"),
        }
    }

    #[test]
    fn test_add_room_which_already_exists() {
        let mut home = SmartHome::new(vec![("Room", Room::new(vec![]))]);
        let res = home.add_room("Room", Room::new(vec![]));
        match res.err().unwrap() {
            SmartHomeError::RoomAlreadyExists(room_name) => {
                assert_eq!(room_name, "Room")
            }
            _ => unreachable!("Unexpected error"),
        }
    }

    #[test]
    fn test_remove_room() {
        let mut home = SmartHome::new(vec![(
            "Room",
            Room::new(vec![(
                "Unused socket",
                ElectricalSocket::new(Box::new(StaticElectricalSocket::new(0., false.into())))
                    .into(),
            )]),
        )]);
        assert_eq!(home.rooms.len(), 1);
        home.remove_room("Room").unwrap();
        assert_eq!(home.rooms.len(), 0);
    }

    #[test]
    fn test_remove_room_which_does_not_exist() {
        let res = SmartHome::new(vec![]).remove_room("Missing bathroom");

        match res.err().unwrap() {
            SmartHomeError::RoomNotFound(room_name) => {
                assert_eq!(room_name, "Missing bathroom")
            }
            _ => unreachable!("Unexpected error"),
        }
    }

    #[test]
    fn test_report() {
        let home = SmartHome::new(vec![(
            "Room",
            Room::new(vec![(
                "Unused socket",
                ElectricalSocket::new(Box::new(StaticElectricalSocket::new(0., false.into())))
                    .into(),
            )]),
        )]);

        home.report();
    }

    #[test]
    fn test_display_smart_home_error() {
        assert_eq!(
            SmartHomeError::RoomNotFound("Room not found".into()).to_string(),
            "Room not found"
        );
        assert_eq!(
            SmartHomeError::DeviceNotFound("Device not found".into()).to_string(),
            "Device not found"
        );
        assert_eq!(
            SmartHomeError::RoomAlreadyExists("Room already exists".into()).to_string(),
            "Room already exists"
        );
        assert_eq!(
            SmartHomeError::DeviceAlreadyExists("Device already exists".into()).to_string(),
            "Device already exists"
        );
    }
}
