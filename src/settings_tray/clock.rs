use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use chrono::{Local, Timelike, Utc};
use iced::{futures::SinkExt, widget::column, Length, Task};

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

    pub fn handle_message(&mut self, clock_message: ClockMessage) -> Task<ClockMessage> {
        match clock_message {
            ClockMessage::UpdateClock(new_time, new_date) => {
                self.time = new_time;
                self.date = new_date;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<ClockMessage> {
        iced::widget::container(column![
            iced::widget::text!("{}", self.time)
                // .horizontal_alignment(iced::alignment::Horizontal::Center)
                .size(14.0),
            iced::widget::text!("{}", self.date)
                // .horizontal_alignment(iced::alignment::Horizontal::Center)
                .size(10.0)
        ])
        .center_y(Length::Fill)
        .into()
    }

    pub fn subscription(&self) -> iced::Subscription<ClockMessage> {
        iced::Subscription::run(|| {
            iced::stream::channel(0, |mut output| async move {
                loop {
                    let now = Local::now();
                    let formatted_time = now.format("%I:%M %p").to_string();
                    let formatted_date = now.format("%Y-%m-%d").to_string();
                    let _ = output
                        .send(ClockMessage::UpdateClock(formatted_time, formatted_date))
                        .await
                        .unwrap();
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
            })
        })
    }
}
