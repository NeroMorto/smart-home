use crate::AppState;
use actix_web::{HttpResponse, Responder, Scope, web};
use serde::{Deserialize, Serialize};
use smart_home_lib::SmartHomeError;
use smart_home_lib::room::Room;

#[derive(Deserialize)]
struct CreateRoom {
    room_name: String,
}

#[derive(Serialize)]
struct RoomDetail {
    name: String,
}

async fn add_room(room: web::Json<CreateRoom>, data: web::Data<AppState>) -> impl Responder {
    let mut sh = data.smart_home.lock().unwrap();
    let may_be_room = sh.add_room(&room.room_name, Room::new(vec![]));
    match may_be_room {
        Ok(_) => HttpResponse::Created().body(room.room_name.clone()),

        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn rooms(data: web::Data<AppState>) -> impl Responder {
    let smart_home = &data.smart_home.lock().unwrap(); // <- get app_name
    let room_names = smart_home
        .get_rooms()
        .keys()
        .map(|room_name| room_name.to_owned())
        .collect::<Vec<String>>();
    web::Json(room_names)
}

async fn room_detail(room_name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let sh = data.smart_home.lock().unwrap();
    let room = sh.get_room(room_name.as_str());
    if room.is_none() {
        return HttpResponse::NotFound().body(format!("Room with name {room_name} not found"));
    }

    HttpResponse::Ok().json(RoomDetail {
        name: room_name.to_string(),
    })
}

async fn delete_room(room_name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut sh = data.smart_home.lock().unwrap();
    match sh.remove_room(room_name.as_str()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => match err {
            SmartHomeError::RoomNotFound(room_name) => {
                HttpResponse::NotFound().body(format!("Romm with name {room_name} not found"))
            }
            _ => HttpResponse::InternalServerError()
                .body("Unexpected error during room delete action".to_string()),
        },
    }
}

pub fn routes() -> Scope {
    web::scope("/rooms")
        .route("", web::get().to(rooms))
        .route("", web::post().to(add_room))
        .route("{room_name}", web::get().to(room_detail))
        .route("{room_name}", web::delete().to(delete_room))
}
