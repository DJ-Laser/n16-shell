use std::{
  collections::HashMap,
  ffi::OsStr,
  hash::{DefaultHasher, Hash, Hasher},
  path::PathBuf,
  process,
};

use super::{Listing, Provider};
use freedesktop_desktop_entry::{self as desktop, DesktopEntry};
use icon_theme::{get_icon_themes, IconTheme};
use icons::get_icon;
use itertools::Itertools;
use listing::{Icon, ListingData};
use phf::phf_map;
use xdg::BaseDirectories;

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
  fn update_data(&mut self) {
    self.data_dirs = get_data_dirs(&BaseDirectories::new().unwrap());
    self.icon_themes = get_icon_themes(&self.data_dirs);
    self.locales = desktop::get_languages_from_env();
  }

  fn make_listing(&self, id: usize, entry: &DesktopEntry) -> Option<ListingData> {
    if !matches!(entry.type_(), Some("Application")) || entry.no_display() {
      return None;
    }

    let icon = get_icon(&entry, "hicolor", &self.icon_themes, &self.data_dirs).map(|path| {
      if matches!(path.extension().and_then(OsStr::to_str), Some("svg")) {
        Icon::Vector(path)
      } else {
        Icon::Bitmap(path)
      }
    });

    let name = entry.name(&self.locales)?;
    let exec = entry.exec();

    Some(ListingData {
      name: name.to_string(),
      command: exec.map(str::to_string),
      icon,
      id,
    })
  }

  fn hash_listings(&self) -> u64 {
    let mut state = DefaultHasher::new();
    self.listings.hash(&mut state);
    state.finish()
  }
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

  fn update_listings(&mut self) -> bool {
    let old_hash = self.hash_listings();

    self.update_data();

    let entries = desktop::Iter::new(self.data_dirs.iter().map(|p| p.join("applications")))
      .entries(Some(&self.locales));

    let listings = entries
      .unique()
      .enumerate()
      .filter_map(|(id, entry)| self.make_listing(id, &entry));

    self.listings = listings.collect();

    return old_hash == self.hash_listings();
  }

  fn listings(&self) -> Vec<Listing> {
    self.listings.iter().map(|l| l.into()).collect()
  }

  fn execute(&self, listing_id: usize) {
    let listing = &self.listings.iter().find(|data| data.id == listing_id);
    let listing = match listing {
      Some(listing) => listing,
      None => panic!(
        "Attempted to execute listing with nonexistent id: {}",
        listing_id
      ),
    };

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
