use crate::device::Device;
type Devices = [Device; 2];
pub struct Room {
    devices: Devices,
}

impl Room {
    pub fn new(devices: Devices) -> Self {
        Self { devices }
    }

    pub fn get_device(&self, index: usize) -> &Device {
        self.check_device_index_bounds(index);
        &self.devices[index]
    }

    pub fn get_device_mut(&mut self, index: usize) -> &mut Device {
        self.check_device_index_bounds(index);
        &mut self.devices[index]
    }

    pub fn report(&self) {
        println!("Room report");
        self.devices.iter().for_each(|d| d.report());
    }

    fn check_device_index_bounds(&self, device_index: usize) {
        let bounds = self.devices.len();
        if device_index >= bounds {
            panic!("Device index `{device_index}` out of bounds `{bounds}`",);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ElectricalSocket, ElectricalSocketState, Thermometer};

    #[test]
    fn test_room() {
        let room = Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ]);
        assert_eq!(room.devices.len(), 2);
    }

    #[test]
    fn test_get_device() {
        let room = Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ]);
        if let Device::ElectricalSocket(socket) = room.get_device(0) {
            assert_eq!(socket.get_power(), 120.);
            assert!(matches!(socket.get_state(), ElectricalSocketState::On));
            return;
        }
        unreachable!()
    }

    #[test]
    #[should_panic(expected = "Device index `2` out of bounds `2`")]
    fn test_get_missing_device() {
        Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ])
        .get_device(2);
    }

    #[test]
    fn test_get_device_mut() {
        let mut room = Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ]);
        if let Device::Thermometer(t) = room.get_device_mut(1) {
            assert_eq!(t.get_temperature(), 34.);
            return;
        }
        unreachable!()
    }

    #[test]
    #[should_panic(expected = "Device index `2` out of bounds `2`")]
    fn test_get_missing_device_mut() {
        Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ])
        .get_device_mut(2);
    }

    #[test]
    fn test_device_index_bounds() {
        let room = Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ]);
        room.check_device_index_bounds(1)
    }

    #[test]
    #[should_panic(expected = "Device index `2` out of bounds `2`")]
    fn test_device_index_bounds_panics() {
        let room = Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ]);
        room.check_device_index_bounds(2)
    }

    #[test]
    fn test_room_report() {
        Room::new([
            Device::ElectricalSocket(ElectricalSocket::new(120., ElectricalSocketState::On)),
            Device::Thermometer(Thermometer::new(34.0)),
        ])
        .report();
    }
}
