use iced::widget::{image, svg};

pub mod applications;

#[derive(Debug, Clone)]
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

  fn execute(&self, listing_id: usize);

  fn meta() -> ProviderMeta {
    ProviderMeta {
      id: Self::id(),
      name: Self::name(),
      priority: Self::priority(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum ListingIcon {
  Bitmap(image::Handle),
  Vector(svg::Handle),
}

#[derive(Debug, Clone)]
pub struct Listing {
  name: String,
  executable: bool,
  icon: Option<ListingIcon>,
  // Should be the id of the provider that created it
  provider: &'static str,
  // Unique id for use with Provider::execute
  id: usize,
}

impl Listing {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn executable(&self) -> bool {
    self.executable
  }

  pub fn icon(&self) -> &Option<ListingIcon> {
    &self.icon
  }

  pub fn id(&self) -> usize {
    self.id
  }
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
  listings: Vec<&'a Listing>,
}

impl<'a> Section<'a> {
  pub fn title(&self) -> &str {
    self.meta.title()
  }

  pub fn priority(&self) -> i32 {
    self.meta.priority()
  }

  pub fn listings(&self) -> &Vec<&Listing> {
    &self.listings
  }
}
