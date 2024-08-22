use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use chrono::{Timelike, Utc};
use iced::{
    futures::SinkExt,
    widget::{
        column,
        text::Style,
    },
    Color, Command,
};

#[derive(Clone, Debug)]
pub enum ClockMessage {
    UpdateClock(String),
    UpdateDate(String),
}

#[derive(Clone, Debug)]
pub struct Clock {
    _date: String,
    time: String,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            _date: "".to_string(),
            time: "".to_string(),
        }
    }

    pub fn handle_message(&mut self, clock_message: ClockMessage) -> iced::Command<ClockMessage> {
        match clock_message {
            ClockMessage::UpdateClock(new_time) => {
                println!("Updating time to {}", &new_time);
                self.time = new_time;
                Command::none()
            }
            _ => todo!(),
        }
    }

    pub fn view(&self) -> iced::Element<ClockMessage> {
        iced::widget::container(column![iced::widget::text!("{}", self.time).style(
            |_theme| {
                Style {
                    color: Some(Color::WHITE),
                }
            }
        )])
        .into()
    }

    pub fn subscription(&self) -> iced::Subscription<ClockMessage> {
        struct ClockWorker;
        iced::subscription::channel(
            std::any::TypeId::of::<ClockWorker>(),
            100,
            |mut output| async move {
                loop {
                    let now = Utc::now();
                    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
                    println!("Sending {} to main thread", formatted_time);
                    output
                        .send(ClockMessage::UpdateClock(formatted_time))
                        .await
                        .unwrap();
                    println!("Through sending thing");
                    // Calculate the duration until the next minute
                    let next_minute = now.with_second(0).unwrap().with_nanosecond(0).unwrap()
                        + chrono::Duration::minutes(1);

                    let duration_until_next_minute =
                        next_minute.timestamp_millis() - Utc::now().timestamp_millis();

                    // Sleep until the next minute
                    {
                        let deadline = Instant::now()
                            + Duration::from_millis(duration_until_next_minute as u64);
                        let now = Instant::now();

                        if let Some(delay) = deadline.checked_duration_since(now) {
                            sleep(delay);
                        }
                    };
                }
            },
        )
    }
}
