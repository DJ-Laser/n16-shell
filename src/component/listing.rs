use iced::widget::{button, horizontal_space, image, row, text};
use iced::{Background, Length};

use crate::listings::Listing;
use crate::theme::{self, Base16Theme};

use super::Component;

pub fn view(listing: Listing, on_press: crate::Message) -> impl Into<Component> {
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
    .style(|theme: &Base16Theme, status| {
      let base = button::Style {
        background: Some(Background::Color(theme.base00)),
        ..theme::button::base(theme)
      };

      match status {
        button::Status::Active | button::Status::Pressed => base,
        button::Status::Hovered => button::Style {
          background: Some(Background::Color(theme.base01)),
          ..base
        },
        button::Status::Disabled => theme::button::disabled(base),
      }
    })
    .on_press(on_press)
}
