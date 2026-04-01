use smart_home_lib::device::smart_socket::SmartSocket;
use smart_home_lib::device::smart_socket::backends::static_electrical_socket::StaticElectricalSocket;
use smart_home_lib::device::smart_thermometer::SmartThermometer;
use smart_home_lib::device::smart_thermometer::backends::static_thermometer::StaticThermometer;
use smart_home_lib::reportable_trait::Reportable;
use smart_home_lib::room::Room;
use smart_home_lib::{SmartHome, SmartHomeError};

fn main() -> Result<(), SmartHomeError> {
    let mut home = SmartHome::new(vec![("Unused room", Room::default())]);

    home.remove_room("Unused room")?;
    home.add_room("Guest room", Room::default())?;

    if let Some(room) = home.get_room_mut("Guest room") {
        room.add_device(
            "First socket",
            SmartSocket::new(Box::new(StaticElectricalSocket::new(0., false.into()))).into(),
        )?;
        room.add_device(
            "Main thermometer",
            SmartThermometer::new(Box::new(StaticThermometer::new(32.))).into(),
        )?;
    }
    report(&home);
    if let Some(room) = home.get_room_mut("Guest room") {
        report(room);
    }

    if let Ok(device) = home.get_device("Guest room", "Main thermometer") {
        report(device);
    }

    if let Some(room) = home.get_room_mut("Guest room") {
        room.remove_device("First socket")?;
    }
    home.remove_room("Guest room")?;
    Ok(())
}

fn report(reportable: &impl Reportable) -> String {
    reportable.report()
}
