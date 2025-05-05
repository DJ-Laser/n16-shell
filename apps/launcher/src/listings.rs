use std::fmt::Debug;

use dyn_clone::DynClone;
use iced::{
  Task,
  widget::{image, svg},
};

use crate::Message;

pub trait Provider: Send {
  fn name(&self) -> &'static str;

  fn priority(&self) -> i32;

  /// Optionally returns a new list of listings when their data is updated
  fn update_listings(&mut self) -> Option<Vec<Box<dyn Listing>>>;
}

#[derive(Debug, Clone)]
pub enum ListingIcon {
  Bitmap(image::Handle),
  Vector(svg::Handle),
}

pub trait Listing: DynClone + Debug + Send {
  fn name(&self) -> &str;

  fn icon(&self) -> Option<&ListingIcon>;

  fn executable(&self) -> bool;
  fn execute(&self) -> Task<Message>;
}

dyn_clone::clone_trait_object!(Listing);

#[derive(Debug)]
pub enum SectionKind {}

#[derive(Debug, Clone)]
pub struct SectionMeta {
  title: String,
  priority: i32,
}

impl SectionMeta {
  pub fn title(&self) -> &str {
    &self.title
  }

  pub fn priority(&self) -> i32 {
    self.priority
  }
}

#[derive(Debug)]
pub struct Section<'a> {
  meta: &'a SectionMeta,
  listings: Vec<&'a dyn Listing>,
}

impl<'a> Section<'a> {
  pub fn title(&self) -> &str {
    self.meta.title()
  }

  pub fn priority(&self) -> i32 {
    self.meta.priority()
  }

  pub fn listings(&self) -> &Vec<&'a dyn Listing> {
    &self.listings
  }
}
