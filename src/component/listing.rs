use iced::widget::{horizontal_space, image, row, text, Row};
use iced::Length;

use crate::listings::Listing;

pub fn view(listing: Listing) -> Row<'static, crate::Message> {
  let mut row = row![];
  if let Some(icon) = listing.icon() {
    row = row.push(image(icon));
  } else {
    row = row.push(horizontal_space());
  };

  row = row.push(text(listing.name().to_string()));

  row.width(Length::Fill)
}
