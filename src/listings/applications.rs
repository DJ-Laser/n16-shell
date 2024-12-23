use super::{Listing, Provider};
use freedesktop_desktop_entry::{self as desktop, DesktopEntry};
use iced::widget::image;
use phf::phf_map;

#[derive(Default)]
pub struct ApplicationProvider {
  listings: Vec<Listing>,
}

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
    let locales = desktop::get_languages_from_env();
    let locales = &locales[..];
    let entries = desktop::Iter::new(desktop::default_paths()).entries(Some(locales));
    let applications = entries.filter_map(move |entry| {
      if !matches!(entry.type_(), Some("Application")) || entry.no_display() {
        return None;
      }

      let name = entry.name(locales)?;
      let icon = entry.icon().map(|path| image::Handle::from_path(path));
      let exec = entry.exec();

      println!("{:?}  {:?} exec:{:?}", name, get_section(&entry), exec);

      return Some(Listing {
        name: name.to_string(),
        icon,
        runnable: exec.is_some(),
      });
    });

    self.listings = applications.collect();
  }

  fn listings(&self) -> Vec<Listing> {
    self.listings.clone()
  }
}
