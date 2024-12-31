use iced::widget::{button, horizontal_space, image, row, text};
use iced::{alignment, Background, Length};

use crate::listings::Listing;
use crate::theme::{self, Base16Theme};

use super::Component;

pub fn view(listing: Listing, on_press: crate::Message) -> impl Into<Component> {
  let image_size = 30;
  let font_size = 20;

  let mut row = row![]
    .align_y(alignment::Vertical::Center)
    .width(Length::Fill);

  let i = listing.icon();
  if let Some(icon) = i {
    let image = image(icon).width(image_size).height(image_size);
    row = row.push(image);
  } else {
    let space = horizontal_space().width(image_size).height(image_size);
    row = row.push(space);
  };

  row = row.push(
    text(listing.name().to_string())
      .align_y(alignment::Vertical::Center)
      .height(image_size)
      .size(font_size),
  );

  button(row)
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
