use iced::widget::{button, horizontal_space, image, row, text, Button};
use iced::Length;

use crate::listings::Listing;

pub fn view(listing: Listing, on_press: crate::Message) -> Button<'static, crate::Message> {
  let mut row = row![];

  let i = listing.icon();
  if let Some(icon) = i {
    let image = image(icon).width(20).height(20);
    row = row.push(image);
  } else {
    let space = horizontal_space().width(20).height(20);
    row = row.push(space);
  };

  let text = text(listing.name().to_string()).size(20);
  row = row.push(text).padding(2);

  button(row.width(Length::Fill))
    .style(button::text)
    .on_press(on_press)
}
