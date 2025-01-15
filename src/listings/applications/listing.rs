use std::{hash::Hash, path::PathBuf};

use iced::{advanced::svg, widget::image};

use super::ApplicationProvider;
use crate::listings::{Listing, ListingIcon, Provider};

#[derive(Debug, Clone, Hash)]
pub enum Icon {
  Bitmap(PathBuf),
  Vector(PathBuf),
}

#[derive(Debug, Hash)]
pub struct ListingData {
  pub name: String,
  pub icon: Option<Icon>,
  pub command: Option<String>,
  pub id: usize,
}

impl Into<Listing> for &ListingData {
  fn into(self) -> Listing {
    Listing {
      name: self.name.clone(),
      icon: self.icon.clone().map(|icon| match icon {
        Icon::Bitmap(path) => ListingIcon::Bitmap(image::Handle::from_path(path)),
        Icon::Vector(path) => ListingIcon::Vector(svg::Handle::from_path(path)),
      }),
      executable: self.command.is_some(),
      provider: ApplicationProvider::id(),
      id: self.id,
    }
  }
}
