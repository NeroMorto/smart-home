use crate::gui_widgets::create_room::CreateRoomComponent;
use crate::gui_widgets::devices_component::{DevicesComponent, DevicesMessage};
use crate::gui_widgets::rooms_component::{RoomsComponent, RoomsMessage};
use iced::widget::{Container, button};
use iced::{
    Element, Length, Task, Theme,
    widget::{column, row, rule, text},
};

mod gui_widgets;
struct SmartHomeState {
    selected_device: Option<String>,
    rooms: RoomsComponent,
    devices: DevicesComponent,
    report: Vec<String>,
    report_loading: bool,
    show_report: bool,
}
#[derive(Debug, Clone)]
enum SHMessage {
    RoomsAction(RoomsMessage),
    DevicesAction(DevicesMessage),
    Report,
    ReportRetrieved(Vec<String>),
    CloseReport,
}

fn update(state: &mut SmartHomeState, message: SHMessage) -> Task<SHMessage> {
    match message {
        SHMessage::RoomsAction(action) => {
            let widget_task = state.rooms.update(action.clone());

            if let RoomsMessage::SelectRoom(room_name) = action {
                state.selected_device = None;
                Task::batch(vec![
                    state
                        .devices
                        .update(DevicesMessage::SelectDevice(None))
                        .map(SHMessage::DevicesAction),
                    widget_task.map(SHMessage::RoomsAction),
                    state
                        .devices
                        .update(DevicesMessage::FetchDevices(room_name))
                        .map(SHMessage::DevicesAction),
                ])
            } else {
                widget_task.map(SHMessage::RoomsAction)
            }
        }
        SHMessage::DevicesAction(action) => {
            state.devices.update(action).map(SHMessage::DevicesAction)
        }
        SHMessage::Report => {
            state.report_loading = true;
            Task::perform(
                async move {
                    reqwest::get("http://localhost:8080/report")
                        .await
                        .unwrap()
                        .json::<Vec<String>>()
                        .await
                        .unwrap()
                },
                SHMessage::ReportRetrieved,
            )
        }
        SHMessage::ReportRetrieved(report_data) => {
            state.report_loading = false;
            state.report = report_data;
            state.show_report = true;
            Task::none()
        }
        SHMessage::CloseReport => {
            state.show_report = false;
            state.report = Vec::new();
            Task::none()
        }
    }
}

fn view(state: &SmartHomeState) -> Element<'_, SHMessage> {
    let rooms_sidebar = state.rooms.view().map(SHMessage::RoomsAction);

    let col = column![rooms_sidebar].width(200).spacing(5);
    let base_view = Container::new(column![
        row![col, rule::vertical(1), detail_pane(state)],
        Container::new(
            button(text("Report")).on_press_maybe(if state.report_loading {
                None
            } else {
                Some(SHMessage::Report)
            })
        )
        .padding(15)
    ])
    .height(Length::Fill);
    if state.show_report {
        let report: Vec<Element<'_, SHMessage>> = state
            .report
            .iter()
            .map(|report_row| text(report_row).into())
            .collect();
        let col = column(report);
        let overlay = Container::new(column![
            col,
            button(text("Close report")).on_press(SHMessage::CloseReport)
        ])
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .style(|_theme: &Theme| iced::widget::container::Style {
            background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.9).into()),
            ..Default::default()
        });

        return overlay.into();
    }

    base_view.into()
}

fn detail_pane(state: &SmartHomeState) -> Element<'_, SHMessage> {
    match (&state.rooms.selected_room, &state.devices.selected_device) {
        (None, None) => text("Select room").into(),
        (None, Some(_)) => unreachable!("Device cannot exist whtiout selected room!"),
        (Some(room_name), _) => column![
            text(room_name),
            state.devices.view().map(SHMessage::DevicesAction)
        ]
        .into(),
    }
}

impl Default for SmartHomeState {
    fn default() -> Self {
        Self {
            selected_device: Default::default(),
            rooms: RoomsComponent::new(CreateRoomComponent::new()),
            devices: DevicesComponent::new(),
            report: Default::default(),
            report_loading: Default::default(),
            show_report: Default::default(),
        }
    }
}

fn new() -> (SmartHomeState, Task<SHMessage>) {
    (
        SmartHomeState::default(),
        Task::done(SHMessage::RoomsAction(RoomsMessage::FetchRooms)),
    )
}

fn main() -> iced::Result {
    iced::application(new, update, view).run()
}
