use crate::gui_widgets::create_room::{CreateRoomComponent, CreateRoomMessage};
use iced::widget::{button, rule};
use iced::widget::{column, text};
use iced::{Element, Task};
use std::collections::HashMap;

pub struct RoomsComponent {
    pub selected_room: Option<String>,
    rooms: Vec<String>,
    room_creator: CreateRoomComponent,
}

#[derive(Clone, Debug)]
pub enum RoomsMessage {
    FetchRooms,
    ReceivedRooms(Vec<String>),

    RoomCreator(CreateRoomMessage),
    RoomAdded(String),
    AddRoomFailed,

    SelectRoom(String),
}

impl RoomsComponent {
    pub fn new(room_creator: CreateRoomComponent) -> RoomsComponent {
        Self {
            rooms: vec![],
            selected_room: None,
            room_creator,
        }
    }

    pub fn update(&mut self, msg: RoomsMessage) -> Task<RoomsMessage> {
        match msg {
            RoomsMessage::FetchRooms => Task::perform(fetch_rooms(), RoomsMessage::ReceivedRooms),
            RoomsMessage::ReceivedRooms(rooms) => {
                self.rooms = rooms;
                Task::none()
            }
            RoomsMessage::SelectRoom(room_name) => {
                self.selected_room = Some(room_name);
                Task::none()
            }
            RoomsMessage::RoomCreator(action) => {
                if let Some(new_room_name) = self.room_creator.update(action) {
                    self.room_creator.set_loading(true);
                    Task::perform(add_room_to_api(new_room_name.clone()), move |result| {
                        if result {
                            RoomsMessage::RoomAdded(new_room_name)
                        } else {
                            RoomsMessage::AddRoomFailed
                        }
                    })
                } else {
                    Task::none()
                }
            }
            RoomsMessage::RoomAdded(new_room_name) => {
                self.room_creator.set_loading(false);
                self.rooms.push(new_room_name);
                Task::none()
            }
            RoomsMessage::AddRoomFailed => {
                self.room_creator.set_loading(true);
                Task::none()
            }
        }
    }

    pub fn view<'a>(&self) -> Element<'a, RoomsMessage> {
        let room_creator = self.room_creator.view().map(RoomsMessage::RoomCreator);
        let rooms: Vec<Element<'_, RoomsMessage>> = self
            .rooms
            .iter()
            .map(|room_name| {
                let is_selected = Some(room_name.clone()) == self.selected_room;
                let button_style = if is_selected {
                    button::primary
                } else {
                    button::secondary
                };
                button(text(room_name.clone()))
                    .width(iced::Length::Fill)
                    .on_press(RoomsMessage::SelectRoom(room_name.clone()))
                    .style(button_style)
                    .into()
            })
            .collect();
        let view = column![text("Rooms"), room_creator, rule::horizontal(1)];
        let view = view.extend(rooms);

        view.into()
    }
}

async fn fetch_rooms() -> Vec<String> {
    let data = reqwest::get("http://127.0.0.1:8080/rooms").await;

    match data {
        Ok(data) => data.json().await.unwrap(),
        Err(e) => {
            eprintln!("{:?}", e);
            vec![]
        }
    }
}

async fn add_room_to_api(name: String) -> bool {
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:8080/rooms")
        .json(&HashMap::from([("room_name", name)]))
        .send()
        .await;
    match res {
        Ok(_) => true,
        Err(e) => {
            eprintln!("{:?}", e);
            false
        }
    }
}
