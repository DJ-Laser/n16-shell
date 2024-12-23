use std::collections::HashMap;

use iced::widget::image;

pub mod applications;

#[derive(Clone)]
pub struct ProviderMeta {
  id: &'static str,
  name: &'static str,
  priority: i32,
}

pub trait Provider {
  fn new() -> Self;

  fn id() -> &'static str;
  fn name() -> &'static str;

  fn priority() -> i32;

  fn update_listings(&mut self);
  fn listings(&self) -> Vec<Listing>;

  fn meta() -> ProviderMeta {
    ProviderMeta {
      id: Self::id(),
      name: Self::name(),
      priority: Self::priority(),
    }
  }
}

#[derive(Clone)]
pub struct Listing {
  name: String,
  runnable: bool,
  icon: Option<image::Handle>,
}

#[derive(Clone)]
pub struct SectionMeta {
  title: String,
  priority: i32,
}

pub struct Section<'a> {
  listings: Vec<&'a Listing>,
}
