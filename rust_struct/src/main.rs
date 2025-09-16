use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use iced::border::Radius;
use iced::widget::container::Style;
use iced::widget::{column, row, scrollable, text};
use iced::{Border, Element, Length, Subscription, Task};

use rust_ib2c_shared_data::SharedData;

pub fn main() -> iced::Result {
    iced::application("Ruststruct", update, view)
        .subscription(subscription)
        .run()
}

fn subscription(_state: &State) -> Subscription<Message> {
    iced::time::every(std::time::Duration::from_millis(50)).map(|_| Message::FetchData)
}

async fn fetch_data(stream: Arc<Mutex<Option<TcpStream>>>) -> Option<Vec<SharedData>> {
    let mut tcp_steam = stream.try_lock().ok()?;
    if tcp_steam.is_none() {
        let new_stream = TcpStream::connect("127.0.0.1:13337").ok()?;
        new_stream.set_nonblocking(true).ok()?;
        new_stream.set_read_timeout(Some(Duration::from_millis(500))).ok()?;
        *tcp_steam = Some(new_stream);
    } 

    let stream = tcp_steam.as_mut()?;

    let mut result = Vec::new();
    while result.len() < 10000 {
        let mut length_buf = [0u8; 4];
        if let Err(e) = stream.read_exact(&mut length_buf) {
            if e.kind() == std::io::ErrorKind::ConnectionReset {
                *tcp_steam = None;
                return None;
            }
            return Some(result);
        }
        let length = u32::from_be_bytes(length_buf) as usize;
        let mut data_buf = vec![0u8; length];
        if let Err(e) = stream.read_exact(&mut data_buf) {
            if e.kind() == std::io::ErrorKind::ConnectionReset {
                *tcp_steam = None;
                return None;
            }
            return Some(result);
        }
        let data: SharedData = serde_json::from_slice(&data_buf).ok()?;
        result.push(data);
    }   
    
    Some(result)
}


#[derive(Debug, Clone)]
enum Message {
    FetchData,
    DataReceived(Option<Vec<SharedData>>),
}

#[derive(Default)]
struct State {
    tcp_stream: Arc<Mutex<Option<TcpStream>>>,
    module_data: HashMap<String, SharedData>,
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::FetchData => {
            return Task::perform(fetch_data(state.tcp_stream.clone()), Message::DataReceived)
        },
        Message::DataReceived(data) => {
            if let Some(data) = data {
                for d in data {
                    state.module_data.entry(d.source.clone())
                        .and_modify(|e| *e = d.clone())
                        .or_insert(d);
                }
            } else {
                state.module_data.clear();
            }
        }
        
    }
    Task::none()
}

fn view(state: &'_ State) -> Element<'_, Message> {
    let mut col = column![
        text("Module Data").size(40),
    ]
    .width(iced::Length::Fill)
    .padding(20)
    .spacing(10);

    if state.module_data.is_empty() {
        col = col.push(text("No data received yet...").size(30));
        return scrollable(col).into();
    }

    for (key, data) in &state.module_data {
        let mut outer_col = column![];
        let mut outer_row = row![];
        let mut inner_col = column![].width(Length::FillPortion(1));
        outer_col = outer_col.push(text(key).size(26));
        inner_col = inner_col.push(text("Meta Data:").size(20));
        inner_col = inner_col.push(row![
            text("Loop duration:").width(Length::Fixed(200.0)), text(format!("{:.3}ms", data.active_time.as_secs_f64()*1000.0)),
        ]);
        inner_col = inner_col.push(row![
            text("Activity:").width(Length::Fixed(200.0)), text(format!("{:.2}", data.activity)),
        ]);
        inner_col = inner_col.push(row![
            text("Target Rating:").width(Length::Fixed(200.0)), text(format!("{:.2}", data.target_rating)),
        ]);
        inner_col = inner_col.push(row![
            text("Stimulation:").width(Length::Fixed(200.0)), text(format!("{:.2}", data.stimulation)),
        ]);
        inner_col = inner_col.push(row![
            text("Inhibition:").width(Length::Fixed(200.0)), text(format!("{:.2}", data.inhibition)),
        ]);
        outer_row = outer_row.push(inner_col);
        let mut inner_col = column![].width(Length::FillPortion(1));
        inner_col = inner_col.push(text("Port Data:").size(20));
        for (port_name, port_data) in data.data.iter() {
            inner_col = inner_col.push(row![
                text(port_name).width(Length::Fixed(200.0)), text(format!("{}", port_data)),
            ]);
        }
        outer_row = outer_row.push(inner_col);
        outer_col = outer_col.push(outer_row);


        let container = iced::widget::Container::new(outer_col)
            .padding(10)
            .width(iced::Length::Fill)
            .style(|_| Style {
                background: Some(iced::Background::Color(iced::Color { r: 0.1, g: 0.1, b: 0.1, a: 0.5, })),
                border: Border {
                    width: 1.0,
                    radius: Radius::new(5.0),
                    color: iced::Color::WHITE,
                },
                ..Style::default()
            });
        col = col.push(container);
    }

    scrollable(col).into()
}