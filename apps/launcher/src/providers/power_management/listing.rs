use std::process;

use crate::{Message, listings::Listing};

#[derive(Debug, Clone)]
pub enum PowerManagementListing {
  Shutdown,
  Suspend,
  Hibernate,
  Reboot,
}

impl PowerManagementListing {
  fn subcommand(&self) -> &'static [&'static str] {
    match self {
      Self::Shutdown => &["poweroff"],
      Self::Suspend => &["suspend"],
      Self::Hibernate => &["hibernate"],
      Self::Reboot => &["reboot"],
    }
  }

  async fn run_command(self) -> crate::Message {
    if let Err(error) = process::Command::new("systemctl")
      .args(self.subcommand())
      .spawn()
    {
      panic!("{}", error)
    };

    Message::ListingExecuted
  }
}

impl Listing for PowerManagementListing {
  fn name(&self) -> &str {
    match self {
      Self::Shutdown => "Shut Down",
      Self::Suspend => "Suspend",
      Self::Hibernate => "Hibernate",
      Self::Reboot => "Reboot",
    }
  }

  fn icon(&self) -> Option<&crate::listings::ListingIcon> {
    None
  }

  fn executable(&self) -> bool {
    true
  }

  fn execute(&self) -> iced::Task<crate::Message> {
    iced::Task::future(self.clone().run_command())
  }
}
