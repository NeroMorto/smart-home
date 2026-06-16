use crate::AppState;
use actix_web::{HttpResponse, Responder, Scope, web};
use serde::Deserialize;
use smart_home_lib::SmartHomeError;
use smart_home_lib::device::Device;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
enum DeviceType {
    Socket,
    Thermometer,
}
#[derive(Deserialize, Debug)]
struct CreateDevice {
    name: String,
    device_type: DeviceType,
}

async fn devices(room_name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let sh = data.smart_home.lock().unwrap();
    if let Some(room) = sh.get_room(room_name.as_str()) {
        let devices = room.devices();
        let device_names = devices
            .keys()
            .map(|device_name| device_name.to_owned())
            .collect::<Vec<String>>();
        return HttpResponse::Ok().json(device_names);
    };
    HttpResponse::NotFound().body(format!("Room with name {room_name} not found"))
}

async fn add_device(
    room_name: web::Path<String>,
    device_data: web::Json<CreateDevice>,
    data: web::Data<AppState>,
) -> impl Responder {
    println!("device name: {:?}", device_data);
    let mut sh = data.smart_home.lock().unwrap();
    let room = sh.get_room_mut(room_name.as_str()).unwrap();
    match device_data.device_type {
        DeviceType::Socket => {
            let device = data.br.create_device("socket").unwrap();
            _ = room.add_device(device_data.name.as_str(), device);
        }
        DeviceType::Thermometer => {
            let device = data.br.create_device("thermometer").unwrap();
            _ = room.add_device(device_data.name.as_str(), device);
        }
    }
    HttpResponse::Created().json(HashMap::from([("device_name", device_data.name.clone())]))
}

async fn device_detail(
    path: web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let (room_name, device_name) = path.into_inner();
    let sh = data.smart_home.lock().unwrap();
    let device = sh.get_device(room_name.as_str(), device_name.as_str());
    let mut device_data: Vec<String> = vec![];
    match device {
        Ok(d) => match d {
            Device::Thermometer(thermometer) => {
                device_data.push(format!("name: {device_name}"));
                device_data.push(format!(
                    "temperature: {temperature:.1}",
                    temperature = thermometer.get_temperature()
                ));
                HttpResponse::Ok().json(device_data)
            }
            Device::ElectricalSocket(electrical_socket) => {
                device_data.push(format!("name: {device_name}"));
                device_data.push(format!(
                    "power: {power:.1}",
                    power = electrical_socket.get_power()
                ));
                HttpResponse::Ok().json(device_data)
            }
        },
        Err(err) => match err {
            SmartHomeError::RoomNotFound(_) => {
                HttpResponse::NotFound().body(format!("Room with name {room_name} not found"))
            }
            SmartHomeError::DeviceNotFound(_) => {
                HttpResponse::NotFound().body(format!("Device with name {device_name} not found"))
            }
            SmartHomeError::DeviceAlreadyExists(_) => HttpResponse::Conflict()
                .body(format!("Device with name {device_name} already exists")),
            _ => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        },
    }
}
async fn delete_device(
    path: web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let (room_name, device_name) = path.into_inner();
    let mut sh = data.smart_home.lock().unwrap();
    let room = sh.get_room_mut(room_name.as_str()).unwrap();
    match room.remove_device(device_name.as_str()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => {
            HttpResponse::NotFound().body(format!("Device with name {device_name} not found"))
        }
    }
}

pub fn routes() -> Scope {
    web::scope("/rooms/{room_name}/devices")
        .route("", web::get().to(devices))
        .route("", web::post().to(add_device))
        .route("{device_name}", web::get().to(device_detail))
        .route("{device_name}", web::delete().to(delete_device))
}
