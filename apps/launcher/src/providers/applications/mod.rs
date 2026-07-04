use std::{ffi::OsStr, path::PathBuf, process};

use async_trait::async_trait;
use freedesktop_desktop_entry::{self as desktop};
use iced::widget::{image, svg};
use itertools::Itertools;
use xdg::BaseDirectories;

use icon_theme::get_icon_themes;
use icons::get_icon;

use crate::providers::{
  ExecutionFinishAction, Match, MatchIcon, Provider, ProviderInfo, ProviderType,
};

mod icon_theme;
mod icons;

#[derive(Debug, Clone)]
pub struct ApplicationInfo {
  name: String,
  icon: Option<MatchIcon>,
  command: Option<String>,

  #[expect(unused)]
  desktop_file: PathBuf,
}

impl ApplicationInfo {
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
          MatchIcon::Vector(svg::Handle::from_path(icon_path))
        } else {
          MatchIcon::Bitmap(image::Handle::from_path(icon_path))
        }
      }),
      command,
      desktop_file,
    }
  }
}

fn get_data_dirs(env: &BaseDirectories) -> Vec<PathBuf> {
  let mut data_dirs: Vec<PathBuf> = vec![];

  if let Some(data_home) = env.get_data_home() {
    data_dirs.push(data_home);
  }

  data_dirs.append(&mut env.get_data_dirs());

  data_dirs
}

pub fn get_application_info() -> Vec<ApplicationInfo> {
  let data_dirs = get_data_dirs(&BaseDirectories::new());
  let icon_themes = get_icon_themes(&data_dirs);
  let locales = desktop::get_languages_from_env();

  let entries =
    desktop::Iter::new(data_dirs.iter().map(|p| p.join("applications"))).entries(Some(&locales));

  entries
    .unique()
    .filter_map(|entry| {
      if !matches!(entry.type_(), Some("Application")) || entry.no_display() {
        return None;
      }

      let icon = get_icon(&entry, "hicolor", &icon_themes, &data_dirs);

      let name = entry.name(&locales)?;
      let exec = entry.exec();

      Some(ApplicationInfo::new(
        name.to_string(),
        icon,
        exec.map(str::to_string),
        entry.path.to_owned(),
      ))
    })
    .collect()
}

pub struct ApplicationProvider {
  application_info: Vec<ApplicationInfo>,
}

impl ApplicationProvider {
  #[expect(clippy::new_without_default)]
  pub fn new() -> Self {
    Self {
      application_info: Vec::new(),
    }
  }
}

#[async_trait]
impl Provider for ApplicationProvider {
  fn init() -> (ProviderInfo, Self)
  where
    Self: Sized,
  {
    (
      ProviderInfo {
        id: "n16/applications".into(),
        name: "Applications".into(),
        provider_type: ProviderType::Static,
      },
      Self {
        application_info: get_application_info(),
      },
    )
  }

  async fn matches(&self) -> Vec<Match> {
    return self
      .application_info
      .iter()
      .enumerate()
      .map(|(id, info)| Match {
        title: info.name.clone(),
        description: None,
        icon: info.icon.clone(),
        keywords: Vec::new(),
        executable: info.command.is_some(),
        id: id as u64,
      })
      .collect();
  }

  async fn matches_dynamic(&self, _search_text: String) -> Vec<Match> {
    unimplemented!()
  }

  async fn execute_match(&self, selected_match: Match) -> ExecutionFinishAction {
    let Some(application_info) = self.application_info.get(selected_match.id as usize) else {
      return ExecutionFinishAction::Close;
    };

    let Some(command) = &application_info.command else {
      return ExecutionFinishAction::Close;
    };

    let args: Vec<&str> = command
      .split_ascii_whitespace()
      .filter(|s| !s.starts_with('%'))
      .collect();

    if let Err(error) = process::Command::new(args[0])
      .args(&args[1..])
      .stdin(process::Stdio::null())
      .stdout(process::Stdio::null())
      .stderr(process::Stdio::null())
      .spawn()
    {
      eprintln!("{error}");
    };

    ExecutionFinishAction::Close
  }
}
