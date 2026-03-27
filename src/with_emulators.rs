use smart_home_lib::device::tcp_electrical_socket::TcpElectricalSocket;
use smart_home_lib::device::udp_thermometer::UdpThermometer;
use smart_home_lib::device::{Device, ElectricalSocket, thermometer::Thermometer};
use smart_home_lib::reportable_trait::Reportable;
use smart_home_lib::room::Room;
use smart_home_lib::{SmartHome, SmartHomeError};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), SmartHomeError> {
    let mut home = SmartHome::new(vec![("Unused room", Room::new(vec![]))]);

    home.remove_room("Unused room")?;
    home.add_room("Guest room", Room::new(vec![]))?;

    if let Some(room) = home.get_room_mut("Guest room") {
        room.add_device(
            "First socket",
            ElectricalSocket::new(Box::new(TcpElectricalSocket::new("127.0.0.1:9002"))).into(),
        )?;
        room.add_device(
            "Main thermometer",
            Thermometer::new(Box::new(UdpThermometer::new("127.0.0.1:9005"))).into(),
        )?;
    }

    report(&home);
    sleep(Duration::from_millis(1000));
    if let Some(room) = home.get_room_mut("Guest room") {
        report(room);
    }

    if let Ok(thermometer) = home.get_device("Guest room", "Main thermometer") {
        report(thermometer);
    }
    if let Some(socket) = home
        .get_room_mut("Guest room")
        .unwrap()
        .get_device_mut("First socket")
    {
        println!("===== Separate socket report =====");
        report(socket);
        match socket {
            Device::Thermometer(_) => {}
            Device::ElectricalSocket(socket) => {
                println!("===== Toggle socket =====");
                socket.toggle();
            }
        }

        println!("===== Separate socket report =====");
        report(socket);
    }

    if let Some(room) = home.get_room_mut("Guest room") {
        room.remove_device("First socket")?;
    }
    home.remove_room("Guest room")?;
    Ok(())
}

fn report(reportable: &impl Reportable) {
    reportable.report()
}
