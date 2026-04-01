use smart_home_lib::{
    device::{Device, smart_socket::SmartSocket, smart_thermometer::SmartThermometer},
    report_composer::ReportGenerator,
    room::Room,
};

fn main() {
    let room = Room::default();
    let device = Device::default();
    let socket1 = SmartSocket::default();
    let socket2 = SmartSocket::default();
    let thermo1 = SmartThermometer::default();
    let thermo2 = SmartThermometer::default();

    let report = ReportGenerator::new()
        .with(&room)
        .with(&device)
        .with(&socket1)
        .with(&socket2)
        .with(&thermo1)
        .with(&thermo2)
        .report();
    println!("Report: {}", report);
}
