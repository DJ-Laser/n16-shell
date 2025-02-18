use iced::widget::text_input;

use super::Component;
use crate::{listings::Listing, Message};

pub fn filter_listing(listing: &dyn Listing, query: &str) -> bool {
  let trimmed: String = query.split_whitespace().collect::<String>().to_lowercase();

  return listing.name().to_lowercase().contains(&trimmed);
}

pub const SEARCH_INPUT_ID: &'static str = "SEARCH_QUERY_INPUT";

pub fn view(query: &str) -> impl Into<Component> {
  text_input("Search", query)
    .id(SEARCH_INPUT_ID)
    .on_input(Message::SearchQueryChanged)
}
