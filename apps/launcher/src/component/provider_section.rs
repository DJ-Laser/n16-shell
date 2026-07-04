use iced::widget::column;

use super::{Component, match_entry};
use crate::providers::{Match, ProviderInfo};

pub fn view(
  _provider_info: &ProviderInfo,
  matches: Vec<&Match>,
  selected: Option<usize>,
  on_press: impl Fn(usize) -> crate::launcher::Message,
) -> impl Into<Component> {
  let mut matches_veiw = column![];

  for (idx, match_entry) in matches.into_iter().enumerate() {
    let is_selected: bool = selected.is_some_and(|s| s == idx);
    matches_veiw = matches_veiw.push(match_entry::view(match_entry, is_selected, on_press(idx)));
  }

  matches_veiw
}
