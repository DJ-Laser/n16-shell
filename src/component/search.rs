use iced::widget::text_input;

use super::Component;

pub fn view(query: &str) -> impl Into<Component> {
  text_input("Search", query)
}
