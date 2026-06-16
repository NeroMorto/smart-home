use crate::AppState;
use actix_web::{Responder, Scope, web};
use smart_home_lib::device::Device;

async fn report(data: web::Data<AppState>) -> impl Responder {
    let sh = data.smart_home.lock().unwrap();
    let mut report: Vec<String> = vec![];
    report.push("*******************************".into());
    report.push("====== Smart home report ======".into());
    report.push("*******************************".into());
    for (room_name, room) in sh.get_rooms() {
        report.push(format!("====== Room: {room_name} ======"));
        for (device_name, device) in room.devices() {
            report.push(format!("Device: {device_name}"));
            match device {
                Device::Thermometer(device) => report.push(format!(
                    "      Temperature: {temperature:.1}",
                    temperature = device.get_temperature()
                )),
                Device::ElectricalSocket(device) => report.push(format!(
                    "      Power: {power:.1}",
                    power = device.get_power()
                )),
            }
        }
    }
    report.push("================================".into());

    web::Json(report)
}

pub fn routes() -> Scope {
    web::scope("/report").route("", web::get().to(report))
}
