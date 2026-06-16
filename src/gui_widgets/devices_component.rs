use crate::gui_widgets::create_device::{CreateDevice, CreateDeviceComponent, CreateDeviceMessage};
use iced::widget::{Container, button, column, stack, text};
use iced::{Element, Length, Task, Theme};

pub struct DevicesComponent {
    current_room_name: Option<String>,
    device_info: Vec<String>,
    devices: Vec<String>,
    pub selected_device: Option<String>,
    device_creator: CreateDeviceComponent,
    show_device_info: bool,
}

#[derive(Debug, Clone)]
pub enum DevicesMessage {
    FetchDevices(String),
    RetrievedDevices(Vec<String>),
    SelectDevice(Option<String>),
    CreateDeviceAction(CreateDeviceMessage),
    CloseDeviceDetails,
    RetrievedDevicesDetails(Vec<String>),

    DeviceAdded(String),
    AddDeviceFailed,
}

impl DevicesComponent {
    pub fn new() -> Self {
        Self {
            current_room_name: None,
            devices: vec![],
            device_info: vec![],
            selected_device: None,
            device_creator: CreateDeviceComponent::new(),
            show_device_info: false,
        }
    }

    pub fn update(&mut self, message: DevicesMessage) -> Task<DevicesMessage> {
        match message {
            DevicesMessage::FetchDevices(room_name) => {
                self.current_room_name = Some(room_name.clone());
                Task::perform(fetch_devices(room_name), DevicesMessage::RetrievedDevices)
            }
            DevicesMessage::RetrievedDevices(devices) => {
                self.devices = devices;
                Task::none()
            }
            DevicesMessage::SelectDevice(device_name) => {
                self.selected_device = device_name.clone();
                if let Some(device_name) = device_name {
                    return Task::perform(
                        fetch_device_info(self.current_room_name.clone().unwrap(), device_name),
                        DevicesMessage::RetrievedDevicesDetails,
                    );
                } else {
                    self.device_info = vec![];
                    self.show_device_info = false;
                }

                Task::none()
            }
            DevicesMessage::CreateDeviceAction(action) => {
                if let Some(device) = self.device_creator.update(action) {
                    if self.current_room_name.is_none() {
                        return Task::none();
                    }
                    self.device_creator.set_is_loading(true);
                    Task::perform(
                        add_device(self.current_room_name.clone().unwrap(), device.clone()),
                        move |result| {
                            if result {
                                DevicesMessage::DeviceAdded(device.name)
                            } else {
                                DevicesMessage::AddDeviceFailed
                            }
                        },
                    )
                } else {
                    Task::none()
                }
            }
            DevicesMessage::DeviceAdded(device_name) => {
                self.device_creator.set_is_loading(false);
                self.devices.push(device_name);
                Task::none()
            }
            DevicesMessage::AddDeviceFailed => {
                self.device_creator.set_is_loading(false);
                Task::none()
            }
            DevicesMessage::CloseDeviceDetails => {
                self.show_device_info = false;
                Task::none()
            }
            DevicesMessage::RetrievedDevicesDetails(device_info) => {
                self.device_info = device_info;
                self.show_device_info = true;
                Task::none()
            }
        }
    }

    pub fn view<'a>(&self) -> Element<'a, DevicesMessage> {
        let device_creator = self
            .device_creator
            .view()
            .map(DevicesMessage::CreateDeviceAction);
        let devices: Vec<Element<'_, DevicesMessage>> = self
            .devices
            .iter()
            .map(|device| {
                button(text(device.clone()))
                    .on_press(DevicesMessage::SelectDevice(Some(device.clone())))
                    .into()
            })
            .collect();
        let base_view = Container::new(column![text("Devices"), device_creator].extend(devices))
            .height(Length::Fill);
        if self.show_device_info {
            let device_info: Vec<Element<'_, DevicesMessage>> = self
                .device_info
                .iter()
                .map(|row| text(row.clone()).into())
                .collect();
            let col = column![text("Device state").size(24),].padding(20);
            let col = col.extend(device_info);
            let col = col.push(button("Close").on_press(DevicesMessage::CloseDeviceDetails));

            let overlay: Container<'_, DevicesMessage> = Container::new(col)
                .width(iced::Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center)
                .style(|_theme: &Theme| iced::widget::container::Style {
                    background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.9).into()),
                    ..Default::default()
                });
            return stack![base_view, overlay,].into();
        }

        base_view.into()
    }
}

async fn fetch_device_info(room_name: String, device_name: String) -> Vec<String> {
    let response = reqwest::get(format!(
        "http://127.0.0.1:8080/rooms/{room_name}/devices/{device_name}"
    ))
    .await;
    match response {
        Ok(response) => response.json::<Vec<String>>().await.unwrap(),
        Err(e) => {
            eprintln!("{}", e);
            vec![]
        }
    }
}

async fn add_device(room_name: String, device: CreateDevice) -> bool {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://127.0.0.1:8080/rooms/{room_name}/devices"))
        .json(&device)
        .send()
        .await;
    match response {
        Ok(_) => true,
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    }
}

async fn fetch_devices(room_name: String) -> Vec<String> {
    let res = reqwest::get(format!("http://127.0.0.1:8080/rooms/{room_name}/devices")).await;
    match res {
        Ok(response) => response.json().await.unwrap(),
        Err(e) => {
            eprintln!("{}", e);
            vec![]
        }
    }
}
