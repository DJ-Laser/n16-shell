use listing::PowerManagementListing;

use crate::listings::Provider;

mod listing;

pub struct PowerManagementProvider {}

impl PowerManagementProvider {
  pub fn new() -> Self {
    Self {}
  }
}

impl Provider for PowerManagementProvider {
  fn name(&self) -> &'static str {
    "Power Management"
  }

  fn priority(&self) -> i32 {
    0
  }

  fn update_listings(&mut self) -> Option<Vec<Box<dyn crate::listings::Listing>>> {
    Some(vec![
      Box::new(PowerManagementListing::Shutdown),
      Box::new(PowerManagementListing::Suspend),
      Box::new(PowerManagementListing::Hibernate),
      Box::new(PowerManagementListing::Reboot),
    ])
  }
}
