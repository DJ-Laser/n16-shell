use iced::widget::image;

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

  fn execute(&self, listing_index: usize);

  fn meta() -> ProviderMeta {
    ProviderMeta {
      id: Self::id(),
      name: Self::name(),
      priority: Self::priority(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Listing {
  name: String,
  executable: bool,
  icon: Option<image::Handle>,
  // Should be the id of the provider that created it
  provider: &'static str,
}

impl Listing {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn executable(&self) -> bool {
    self.executable
  }

  pub fn icon(&self) -> Option<&image::Handle> {
    self.icon.as_ref()
  }
}

#[derive(Debug)]
pub enum SectionKind {}

#[derive(Debug, Clone)]
pub struct SectionMeta {
  title: String,
  priority: i32,
}

#[derive(Debug)]
pub struct Section<'a> {
  listings: Vec<&'a Listing>,
}
