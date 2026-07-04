use iced::widget::{Space, button, image, row, svg, text};
use iced::{Length, alignment};
use n16_core::theme::{self, Base16Theme};

use crate::providers::{Match, MatchIcon};

use super::Component;

pub fn view(
  match_entry: &Match,
  selected: bool,
  on_press: crate::launcher::Message,
) -> impl Into<Component> {
  let image_size = 30;
  let font_size = 20;

  let mut row = row![]
    .align_y(alignment::Vertical::Center)
    .width(Length::Fill)
    .spacing(10);

  match match_entry.icon.clone() {
    Some(MatchIcon::Bitmap(handle)) => {
      let image = image(handle).width(image_size).height(image_size);
      row = row.push(image);
    }
    Some(MatchIcon::Vector(handle)) => {
      let svg = svg(handle.clone()).width(image_size).height(image_size);
      row = row.push(svg);
    }
    None => {
      let space = Space::new().width(image_size).height(image_size);
      row = row.push(space);
    }
  }

  row = row.push(
    text(match_entry.title.to_string())
      .align_y(alignment::Vertical::Center)
      .height(image_size)
      .size(font_size),
  );

  button(row)
    .padding([5, 0])
    .style(move |theme: &Base16Theme, status| {
      let base = button::Style {
        background: Some(theme.base00.into()),
        ..theme::button::base(theme)
      };

      match status {
        _ if selected => button::Style {
          background: Some(theme.base01.into()),
          ..base
        },

        button::Status::Hovered => button::Style {
          background: Some(theme.base01.into()),
          ..base
        },

        button::Status::Active | button::Status::Pressed => base,
        button::Status::Disabled => theme::button::disabled(base),
      }
    })
    .on_press(on_press)
}
