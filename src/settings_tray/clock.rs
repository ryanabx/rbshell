use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use chrono::{Timelike, Utc};
use iced::{
    futures::SinkExt,
    widget::{column, text::Style},
    Color, Command,
};

#[derive(Clone, Debug)]
pub enum ClockMessage {
    UpdateClock(String, String),
}

#[derive(Clone, Debug)]
pub struct Clock {
    date: String,
    time: String,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            date: "".to_string(),
            time: "".to_string(),
        }
    }

    pub fn handle_message(&mut self, clock_message: ClockMessage) -> iced::Command<ClockMessage> {
        match clock_message {
            ClockMessage::UpdateClock(new_time, new_date) => {
                self.time = new_time;
                self.date = new_date;
                Command::none()
            }
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
        iced::subscription::channel(
            std::any::TypeId::of::<ClockMessage>(),
            0,
            |mut output| async move {
                loop {
                    let now = Utc::now();
                    let formatted_time = now.format("%H:%M:%S").to_string();
                    let formatted_date = now.format("%Y-%m-%d").to_string();
                    println!("Sending {} to main thread", formatted_time);
                    let _ = output
                        .send(ClockMessage::UpdateClock(formatted_time, formatted_date))
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
