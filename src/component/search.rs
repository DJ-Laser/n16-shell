use iced::widget::text_input;

use super::Component;
use crate::Message;

pub fn preprocess_query(query: &str) -> String {
  let trimmed: String = query.split_whitespace().collect();
  trimmed.to_lowercase()
}

pub const SEARCH_INPUT_ID: &'static str = "SEARCH_QUERY_INPUT";

pub fn view(query: &str) -> impl Into<Component> {
  text_input("Search", query)
    .id(SEARCH_INPUT_ID)
    .on_input(Message::SearchQueryChanged)
}
