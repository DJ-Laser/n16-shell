use std::fmt::Display;

use chrono::TimeZone;
use iced::{
  widget::{container, text},
  Length,
};

use super::Component;

pub fn view<Tz: TimeZone>(time: chrono::DateTime<Tz>) -> impl Into<Component>
where
  Tz::Offset: Display,
{
  let time = format!("{}", time.format("%H:%M"));
  container(text(time)).center_x(70).center_y(Length::Fill)
}
