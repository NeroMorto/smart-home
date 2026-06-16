use iced::Element;
use iced::widget::{button, pick_list, row, text, text_input};
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug)]
pub struct CreateDeviceComponent {
    input_value: String,
    device_type: DeviceType,
    is_loading: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize)]
pub enum DeviceType {
    Socket,
    Thermometer,
}

impl DeviceType {
    const ALL: [DeviceType; 2] = [DeviceType::Socket, DeviceType::Thermometer];
}

impl Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Socket => write!(f, "socket"),
            DeviceType::Thermometer => write!(f, "thermometer"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CreateDeviceMessage {
    InputChanged(String),
    DeviceTypeChanged(DeviceType),
    Submit,
}
#[derive(Clone, Debug, Serialize)]
pub struct CreateDevice {
    pub name: String,
    pub device_type: DeviceType,
}

impl CreateDeviceComponent {
    pub fn new() -> Self {
        Self {
            input_value: "".into(),
            device_type: DeviceType::Socket,
            is_loading: false,
        }
    }

    pub fn set_is_loading(&mut self, is_loading: bool) {
        self.is_loading = is_loading;
    }

    pub fn update(&mut self, message: CreateDeviceMessage) -> Option<CreateDevice> {
        match message {
            CreateDeviceMessage::InputChanged(value) => {
                self.input_value = value;
                None
            }
            CreateDeviceMessage::DeviceTypeChanged(device_type) => {
                self.device_type = device_type;
                None
            }
            CreateDeviceMessage::Submit => {
                let device_name = self.input_value.trim().to_owned();
                if !self.input_value.is_empty() {
                    self.input_value.clear();
                    Some(CreateDevice {
                        name: device_name,
                        device_type: self.device_type,
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn view<'a>(&self) -> Element<'a, CreateDeviceMessage> {
        row![
            text_input("New room name", &self.input_value)
                .on_input(CreateDeviceMessage::InputChanged)
                .on_submit(CreateDeviceMessage::Submit),
            pick_list(
                DeviceType::ALL,
                Some(self.device_type),
                CreateDeviceMessage::DeviceTypeChanged
            ),
            button(text("➕").size(20))
                .on_press_maybe(if self.is_loading {
                    None
                } else {
                    Some(CreateDeviceMessage::Submit)
                })
                .padding([5, 15]),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center)
        .into()
    }
}
