use iced::widget::image;

use super::ApplicationProvider;
use crate::listings::{Listing, Provider};

#[derive(Debug)]
pub struct ListingData {
  pub name: String,
  pub icon: Option<image::Handle>,
  pub command: Option<String>,
}

impl Into<Listing> for &ListingData {
  fn into(self) -> Listing {
    Listing {
      name: self.name.clone(),
      icon: self.icon.clone(),
      executable: self.command.is_some(),
      provider: ApplicationProvider::id(),
    }
  }
}
