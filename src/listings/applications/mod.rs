use std::process;

use super::{Listing, Provider};
use base_dirs::get_data_dirs;
use freedesktop_desktop_entry::{self as desktop, DesktopEntry};
use iced::widget::image;
use icons::get_icon;
use listing::ListingData;
use phf::phf_map;
use xdg::BaseDirectories;

mod base_dirs;
mod icons;
mod listing;

static CATEGORY_TO_SECTION: phf::Map<&'static str, &'static str> = phf_map! {
  "Game" => "Gaming",
  "ActionGame" => "Gaming",
  "AdventureGame" => "Gaming",

  "Development" => "Development",
  "TextEditor" => "Development",
  "IDE" => "Development",

  "InstantMessaging" => "Internet",
  "WebBrowser" => "Internet",
  "Chat" => "Internet",

  "System" => "System",
  "Settings" => "System",
  "HardwareSettings" => "System",
  "Printing" => "System",

  "$NO_MATCHES" => "Other",
};

fn get_section(entry: &DesktopEntry) -> &'static str {
  if let Some(categories) = entry.categories() {
    for category in categories {
      if let Some(section) = CATEGORY_TO_SECTION.get(category) {
        return section;
      }
    }
  }

  CATEGORY_TO_SECTION
    .get("$NO_MATCHES")
    .expect("Should have a fallback section name")
}

#[derive(Default)]
pub struct ApplicationProvider {
  listings: Vec<ListingData>,
}

impl Provider for ApplicationProvider {
  fn new() -> Self {
    Self::default()
  }

  fn id() -> &'static str {
    "applications"
  }

  fn name() -> &'static str {
    "Application Provider"
  }

  fn priority() -> i32 {
    100
  }

  fn update_listings(&mut self) {
    let data_dirs = get_data_dirs(&BaseDirectories::new().unwrap());
    let locales = &desktop::get_languages_from_env()[..];

    let entries = desktop::Iter::new(desktop::default_paths()).entries(Some(locales));

    let applications = entries.filter_map(move |entry| {
      if !matches!(entry.type_(), Some("Application")) || entry.no_display() {
        return None;
      }

      let icon = get_icon(&entry, &data_dirs).map(image::Handle::from_path);
      let name = entry.name(locales)?;
      let exec = entry.exec();

      return Some(ListingData {
        name: name.to_string(),
        icon,
        command: exec.map(str::to_string),
      });
    });

    self.listings = applications.collect();
  }

  fn listings(&self) -> Vec<Listing> {
    self.listings.iter().map(|l| l.into()).collect()
  }

  fn execute(&self, listing_index: usize) {
    let listing = &self.listings[listing_index];
    let command = listing
      .command
      .as_ref()
      .expect("Should not run listings with executable: false");

    let args: Vec<&str> = command
      .split_ascii_whitespace()
      .filter_map(|s| if s.starts_with('%') { None } else { Some(s) })
      .collect();

    match process::Command::new(args[0]).args(&args[1..]).spawn() {
      Err(error) => panic!("{}", error),
      Ok(_) => (),
    };
  }
}
