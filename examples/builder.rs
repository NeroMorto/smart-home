use smart_home_lib::{
    SmartHome,
    device::{smart_socket::SmartSocket, smart_thermometer::SmartThermometer},
    home_builder::HomeBuilder,
};

fn main() {
    let _home: SmartHome = HomeBuilder::new()
        .add_room("First room")
        .add_device("Socket_1", SmartSocket::default())
        .add_device("Socket_2", SmartSocket::default())
        .add_device("Thermo_1", SmartThermometer::default())
        .add_room("Second room")
        .add_device("Socket_3", SmartSocket::default())
        .add_device("Thermo_2", SmartThermometer::default())
        .build();
}
