use iced::widget::{column, text};

use super::{Component, match_entry};
use crate::launcher::providers::{Match, ProviderInfo};

pub fn view(
  provider_info: &ProviderInfo,
  matches: Vec<(usize, &Match)>,
  selected: Option<usize>,
  on_press: impl Fn(usize) -> crate::launcher::gui::Message,
) -> impl Into<Component> {
  let mut matches_veiw = column![text(provider_info.name.clone())];

  for (idx, match_entry) in matches {
    let is_selected: bool = selected.is_some_and(|s| s == idx);
    matches_veiw = matches_veiw.push(match_entry::view(match_entry, is_selected, on_press(idx)));
  }

  matches_veiw
}
