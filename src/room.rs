use crate::SmartHomeError;
use crate::device::Device;
use crate::reportable_trait::Reportable;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Default)]
pub struct Room {
    devices: HashMap<String, Device>,
    subscribers: Vec<Box<dyn Subscriber>>,
}

impl Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room")
            .field("devices", &self.devices)
            .field("subscribers_count", &self.subscribers.len())
            .finish()
    }
}

impl Reportable for Room {
    fn report(&self) -> String {
        let mut room_report = format!("Room: devices count = {}", self.devices.len());
        room_report.extend(
            self.devices
                .iter()
                .map(|(device_name, device)| format!("({device_name} | {})", device.report())),
        );
        room_report
    }
}

impl Room {
    pub fn new<I, S>(devices: I) -> Self
    where
        I: IntoIterator<Item = (S, Device)>,
        S: Into<String>,
    {
        Self {
            devices: HashMap::from_iter(
                devices
                    .into_iter()
                    .map(|(device_name, device)| (device_name.into(), device)),
            ),
            subscribers: vec![],
        }
    }

    pub fn add_device(&mut self, device_name: &str, device: Device) -> Result<(), SmartHomeError> {
        if self.devices.contains_key(device_name) {
            return Err(SmartHomeError::DeviceAlreadyExists(device_name.into()));
        }
        for sub in &mut self.subscribers {
            sub.on_event(&device);
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

    pub fn add_sub(&mut self, handler: impl Subscriber + 'static) {
        self.subscribers.push(Box::new(handler));
    }
}

pub trait Subscriber {
    fn on_event(&mut self, device: &Device);
}

impl<F> Subscriber for F
where
    F: FnMut(&Device),
{
    fn on_event(&mut self, device: &Device) {
        self(device);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::smart_socket::SmartSocket;
    use crate::device::smart_socket::backends::static_electrical_socket::StaticElectricalSocket;
    use crate::device::smart_thermometer::SmartThermometer;
    use crate::device::smart_thermometer::backends::static_thermometer::StaticThermometer;

    #[test]
    fn test_default_instantiation() {
        let room = Room::default();
        assert_eq!(room.devices.len(), 0);
    }

    #[test]
    fn test_add_device() {
        let mut room = Room::default();
        assert_eq!(room.devices.len(), 0);
        let _ = room.add_device(
            "DeviceName",
            SmartSocket::new(Box::new(StaticElectricalSocket::new(0., false.into()))).into(),
        );
        assert_eq!(room.devices.len(), 1);
    }
    #[test]
    fn test_add_device_which_already_exists() {
        let mut room = Room::new(vec![
            (
                "Unused socket",
                SmartSocket::new(Box::new(StaticElectricalSocket::new(0., false.into()))).into(),
            ),
            (
                "Unused thermometer",
                SmartThermometer::new(Box::new(StaticThermometer::new(0.))).into(),
            ),
        ]);
        assert_eq!(room.devices.len(), 2);
        let res = room.add_device(
            "Unused socket",
            SmartThermometer::new(Box::new(StaticThermometer::new(32.))).into(),
        );
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
            SmartSocket::new(Box::new(StaticElectricalSocket::new(0., false.into()))).into(),
        )]);
        assert_eq!(room.devices.len(), 1);
        let _ = room.remove_device("Unused socket");
        assert_eq!(room.devices.len(), 0);
    }

    #[test]
    fn test_remove_device_which_not_exists() {
        let mut room = Room::default();
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
            SmartSocket::new(Box::new(StaticElectricalSocket::new(220., true.into()))).into(),
        )]);
        if let Device::SmartSocket(socket) = room.get_device("Unused socket").unwrap() {
            assert_eq!(socket.get_power(), 220.0);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_get_missing_device() {
        let room = Room::default();
        assert!(room.get_device("Missing device").is_none());
    }

    #[test]
    fn test_get_device_mut() {
        let mut room = Room::new(vec![(
            "Teapot socket",
            SmartSocket::new(Box::new(StaticElectricalSocket::new(220., true.into()))).into(),
        )]);
        let socket = room.get_device_mut("Teapot socket").unwrap();
        if let Device::SmartSocket(e) = socket {
            assert_eq!(e.get_power(), 220.0);
            e.toggle();
            assert_eq!(e.get_power(), 0.);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn test_get_missing_device_mut() {
        let mut room = Room::default();
        assert!(room.get_device_mut("Missing device").is_none());
    }

    #[test]
    fn test_report() {
        let room = Room::new(vec![(
            "Unused socket",
            SmartSocket::new(Box::new(StaticElectricalSocket::new(0., false.into()))).into(),
        )]);
        room.report();
    }
}
