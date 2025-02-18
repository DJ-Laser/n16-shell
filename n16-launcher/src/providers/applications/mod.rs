use std::{collections::HashMap, path::PathBuf};

use freedesktop_desktop_entry::{self as desktop, DesktopEntry};
use icon_theme::{get_icon_themes, IconTheme};
use icons::get_icon;
use itertools::Itertools;
use listing::ListingData;
use phf::phf_map;
use xdg::BaseDirectories;

use crate::listings::{Listing, Provider};

mod icon_theme;
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

pub fn get_data_dirs(env: &BaseDirectories) -> Vec<PathBuf> {
  let mut data_dirs: Vec<PathBuf> = vec![];
  data_dirs.push(env.get_data_home());
  data_dirs.append(&mut env.get_data_dirs());

  data_dirs
}

#[derive(Default)]
pub struct ApplicationProvider {
  listings: Vec<ListingData>,
  icon_themes: HashMap<String, IconTheme>,
  data_dirs: Vec<PathBuf>,
  locales: Vec<String>,
}

impl ApplicationProvider {
  pub fn new() -> Self {
    Self::default()
  }

  fn update_data(&mut self) {
    self.data_dirs = get_data_dirs(&BaseDirectories::new().unwrap());
    self.icon_themes = get_icon_themes(&self.data_dirs);
    self.locales = desktop::get_languages_from_env();
  }

  fn make_listing(&self, entry: &DesktopEntry) -> Option<ListingData> {
    if !matches!(entry.type_(), Some("Application")) || entry.no_display() {
      return None;
    }

    let icon = get_icon(&entry, "hicolor", &self.icon_themes, &self.data_dirs);

    let name = entry.name(&self.locales)?;
    let exec = entry.exec();

    Some(ListingData::new(
      name.to_string(),
      icon,
      exec.map(str::to_string),
      PathBuf::from(entry.path.to_owned()),
    ))
  }

  fn update(&mut self) {
    self.update_data();

    let entries = desktop::Iter::new(self.data_dirs.iter().map(|p| p.join("applications")))
      .entries(Some(&self.locales));

    let listings = entries
      .unique()
      .filter_map(|entry| self.make_listing(&entry));

    self.listings = listings.collect();
  }
}

impl Provider for ApplicationProvider {
  fn name(&self) -> &'static str {
    "Application Provider"
  }

  fn priority(&self) -> i32 {
    100
  }

  fn update_listings(&mut self) -> Option<Vec<Box<dyn Listing>>> {
    self.update();

    Some(
      self
        .listings
        .iter()
        .map(|listing| Box::new(listing.clone()) as Box<dyn Listing>)
        .collect(),
    )
  }
}
