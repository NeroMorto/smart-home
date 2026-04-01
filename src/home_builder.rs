use std::collections::HashMap;

use crate::{SmartHome, device::Device, room::Room};
#[derive(Default)]
pub struct HomeBuilder {
    rooms: HashMap<String, Room>,
}
pub struct RoomBuilder {
    name: String,
    hb: HomeBuilder,
    devices: HashMap<String, Device>,
}
impl RoomBuilder {
    pub fn new(hb: HomeBuilder, room_name: &str) -> Self {
        Self {
            hb,
            name: room_name.into(),
            devices: HashMap::new(),
        }
    }
    pub fn add_device(mut self, name: impl ToString, device: impl Into<Device>) -> Self {
        self.devices.insert(name.to_string(), device.into());
        self
    }
    pub fn add_room(mut self, room_name: &str) -> RoomBuilder {
        self.hb.rooms.insert(self.name, Room::new(self.devices));
        Self {
            name: room_name.into(),
            hb: self.hb,
            devices: HashMap::new(),
        }
    }
    pub fn build(mut self) -> SmartHome {
        self.hb.rooms.insert(self.name, Room::new(self.devices));
        SmartHome::new(self.hb.rooms)
    }
}
impl HomeBuilder {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(self, name: &str) -> RoomBuilder {
        RoomBuilder::new(self, name)
    }
}
