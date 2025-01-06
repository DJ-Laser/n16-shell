use super::ApplicationProvider;
use crate::listings::{Listing, ListingIcon, Provider};

#[derive(Debug)]
pub struct ListingData {
  pub name: String,
  pub icon: Option<ListingIcon>,
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
