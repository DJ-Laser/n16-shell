use std::{ffi::OsStr, path::PathBuf, process};

use iced::{Task, advanced::svg, widget::image};

use crate::listings::{Listing, ListingIcon};

#[derive(Debug, Clone)]
pub struct ListingData {
  name: String,
  icon: Option<ListingIcon>,
  command: Option<String>,

  #[allow(unused)]
  desktop_file: PathBuf,
}

impl ListingData {
  pub fn new(
    name: String,
    icon: Option<PathBuf>,
    command: Option<String>,
    desktop_file: PathBuf,
  ) -> Self {
    Self {
      name,
      icon: icon.map(|icon_path| {
        if matches!(icon_path.extension().and_then(OsStr::to_str), Some("svg")) {
          ListingIcon::Vector(svg::Handle::from_path(icon_path))
        } else {
          ListingIcon::Bitmap(image::Handle::from_path(icon_path))
        }
      }),
      command,
      desktop_file,
    }
  }
}

impl Listing for ListingData {
  fn name(&self) -> &str {
    &self.name
  }

  fn icon(&self) -> Option<&ListingIcon> {
    self.icon.as_ref()
  }

  fn executable(&self) -> bool {
    self.command.is_some()
  }

  fn execute(&self) -> Task<crate::Message> {
    self
      .command
      .as_ref()
      .map_or_else::<Task<crate::Message>, _, _>(Task::none, |command| {
        iced::Task::future(run_entry_command(command.clone()))
      })
  }
}

async fn run_entry_command(command: String) -> crate::Message {
  let args: Vec<&str> = command
    .split_ascii_whitespace()
    .filter_map(|s| if s.starts_with('%') { None } else { Some(s) })
    .collect();

  match process::Command::new(args[0]).args(&args[1..]).spawn() {
    Err(error) => panic!("{}", error),
    Ok(_) => (),
  };

  crate::Message::ListingExecuted
}
