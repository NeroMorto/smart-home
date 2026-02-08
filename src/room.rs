use crate::SmartHomeError;
use crate::device::Device;
use crate::reportable_trait::Reportable;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Room {
    devices: HashMap<String, Device>,
}

impl Reportable for Room {
    fn report(&self) {
        println!("Room: devices count = {}", self.devices.len());
        self.devices.iter().for_each(|(_, device)| device.report());
    }
}

impl Room {
    pub fn new(devices: Vec<(&str, Device)>) -> Self {
        Self {
            devices: HashMap::from_iter(
                devices
                    .into_iter()
                    .map(|(device_name, device)| (device_name.to_string(), device)),
            ),
        }
    }

    pub fn add_device(&mut self, device_name: &str, device: Device) -> Result<(), SmartHomeError> {
        if self.devices.contains_key(device_name) {
            return Err(SmartHomeError::DeviceAlreadyExists(device_name.into()));
        }
        self.devices.insert(device_name.into(), device);
        Ok(())
    }

    pub fn get_device(&self, device_name: &str) -> Option<&Device> {
        self.devices.get(device_name)
    }

    pub fn get_device_mut(&mut self, device_name: &str) -> Option<&mut Device> {
        self.devices.get_mut(device_name)
    }

    pub fn remove_device(&mut self, device_name: &str) -> Result<(), SmartHomeError> {
        if self.devices.remove(device_name).is_none() {
            return Err(SmartHomeError::DeviceNotFound(device_name.into()));
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{ElectricalSocket, Thermometer};

    #[test]
    fn test_add_device() {
        let mut room = Room::new(vec![]);
        assert_eq!(room.devices.len(), 0);
        let _ = room.add_device("DeviceName", ElectricalSocket::new(0., false.into()).into());
        assert_eq!(room.devices.len(), 1);
    }
    #[test]
    fn test_add_device_which_already_exists() {
        let mut room = Room::new(vec![
            (
                "Unused socket",
                ElectricalSocket::new(0., false.into()).into(),
            ),
            ("Unused thermometer", Thermometer::new(0.).into()),
        ]);
        assert_eq!(room.devices.len(), 2);
        let res = room.add_device("Unused socket", Thermometer::new(0.).into());
        assert_eq!(room.devices.len(), 2);
        match res.err().unwrap() {
            SmartHomeError::DeviceAlreadyExists(device_name) => {
                assert_eq!(device_name, "Unused socket");
            }
            _ => unreachable!("Unexpected error variant"),
        }
    }

    #[test]
    fn test_remove_device() {
        let mut room = Room::new(vec![(
            "Unused socket",
            ElectricalSocket::new(0., false.into()).into(),
        )]);
        assert_eq!(room.devices.len(), 1);
        let _ = room.remove_device("Unused socket");
        assert_eq!(room.devices.len(), 0);
    }

    #[test]
    fn test_remove_device_which_not_exists() {
        let mut room = Room::new(vec![]);
        assert_eq!(room.devices.len(), 0);
        let _ = room.remove_device("Unused socket");
        let res = room.remove_device("Unused socket");
        match res.err().unwrap() {
            SmartHomeError::DeviceNotFound(device_name) => {
                assert_eq!(device_name, "Unused socket");
            }
            _ => unreachable!("Unexpected error variant"),
        }
    }

    #[test]
    fn test_get_device() {
        let room = Room::new(vec![(
            "Unused socket",
            ElectricalSocket::new(220., true.into()).into(),
        )]);
        if let Device::ElectricalSocket(socket) = room.get_device("Unused socket").unwrap() {
            assert_eq!(socket.get_power(), 220.0);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_get_missing_device() {
        let room = Room::new(vec![]);
        assert!(room.get_device("Missing device").is_none());
    }

    #[test]
    fn test_get_device_mut() {
        let mut room = Room::new(vec![(
            "Teapot socket",
            ElectricalSocket::new(220., true.into()).into(),
        )]);
        let socket = room.get_device_mut("Teapot socket").unwrap();
        if let Device::ElectricalSocket(e) = socket {
            assert_eq!(e.get_power(), 220.0);
            e.toggle();
            assert_eq!(e.get_power(), 0.);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn test_get_missing_device_mut() {
        let mut room = Room::new(vec![]);
        assert!(room.get_device_mut("Missing device").is_none());
    }

    #[test]
    fn test_report() {
        let room = Room::new(vec![(
            "Unused socket",
            ElectricalSocket::new(0., false.into()).into(),
        )]);
        room.report()
    }
}
