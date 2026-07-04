use iced::widget::text_input;

use super::Component;
use crate::launcher::Message;

pub const SEARCH_INPUT_ID: &str = "SEARCH_QUERY_INPUT";

pub fn view(query: &str) -> impl Into<Component> {
  text_input("Search", query)
    .id(SEARCH_INPUT_ID)
    .on_input(Message::SearchQueryChanged)
}
