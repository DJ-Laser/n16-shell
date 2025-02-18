use std::fmt::Debug;

use iced::{
  widget::{image, svg},
  Task,
};

use crate::Message;

pub trait Provider {
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

pub trait Listing: Debug {
  fn name(&self) -> &str;

  fn icon(&self) -> Option<&ListingIcon>;

  fn executable(&self) -> bool;
  fn execute(&self) -> Task<Message>;
}

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
