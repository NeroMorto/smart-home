use iced::{
    Element,
    widget::{button, row, text, text_input},
};

pub struct CreateRoomComponent {
    input_value: String,
    is_loading: bool,
}

#[derive(Clone, Debug)]
pub enum CreateRoomMessage {
    InputChanged(String),
    Submit,
}

impl CreateRoomComponent {
    pub fn new() -> Self {
        Self {
            input_value: String::new(),
            is_loading: false,
        }
    }
    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    pub fn update(&mut self, message: CreateRoomMessage) -> Option<String> {
        match message {
            CreateRoomMessage::InputChanged(value) => {
                self.input_value = value;
                None
            }
            CreateRoomMessage::Submit => {
                let name = self.input_value.trim().to_string();
                if !name.is_empty() {
                    self.input_value.clear();
                    Some(name)
                } else {
                    None
                }
            }
        }
    }

    pub fn view<'a>(&self) -> Element<'a, CreateRoomMessage> {
        row![
            text_input("New room name", &self.input_value)
                .on_input(CreateRoomMessage::InputChanged)
                .on_submit(CreateRoomMessage::Submit),
            button(text("➕").size(20))
                .on_press_maybe(if self.is_loading {
                    None
                } else {
                    Some(CreateRoomMessage::Submit)
                })
                .padding([5, 15]),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center)
        .into()
    }
}
