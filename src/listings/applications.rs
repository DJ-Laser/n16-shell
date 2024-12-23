use std::any::Any;

use super::{Listing, Provider};
use freedesktop_desktop_entry as desktop;

#[derive(Default)]
pub struct ApplicationProvider {
  listings: Vec<Listing>,
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
    let applications = entries
      .filter(move |entry| matches!(entry.type_(), Some("Application")) && !entry.no_display());

    for entry in applications {
      println!("{:?}  {:?}", entry.name(locales), entry.no_display())
    }
  }

  fn listings(&self) -> Vec<Listing> {
    self.listings.clone()
  }
}
