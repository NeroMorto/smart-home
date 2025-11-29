use smart_home_lib::SmartHome;
use smart_home_lib::{Device, ElectricalSocket, Room, Thermometer};

fn main() {
    let socket = Device::ElectricalSocket(ElectricalSocket::new(10., true.into()));
    let thermometer = Device::Thermometer(Thermometer::new(10.));
    let room = Room::new([socket, thermometer]);

    let mut home = SmartHome::new([room]);
    home.report();

    if let Device::ElectricalSocket(s) = home.get_room_mut(0).get_device_mut(0) {
        s.toggle()
    }

    home.report();
}
