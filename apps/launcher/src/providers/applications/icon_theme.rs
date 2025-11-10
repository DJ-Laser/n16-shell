use std::{
  collections::HashMap,
  fs, io,
  path::{Path, PathBuf},
};

use tini::Ini;

#[derive(Debug, Clone)]
pub enum IconType {
  Fixed,
  Scalable { max_size: i32, min_size: i32 },
  Threshold(i32),
}

#[derive(Debug, Clone)]
pub struct IconDir {
  size: i32,
  scale: i32,
  icon_type: IconType,
  full_path: PathBuf,
}

impl IconDir {
  pub fn size(&self) -> i32 {
    self.size
  }

  pub fn scale(&self) -> i32 {
    self.scale
  }

  pub fn icon_type(&self) -> &IconType {
    &self.icon_type
  }

  pub fn full_path(&self) -> &Path {
    &self.full_path
  }
}

#[derive(Debug, Clone)]
pub struct IconTheme {
  directory_name: String,
  inherits: Vec<String>,
  directories: Vec<IconDir>,
}

impl IconTheme {
  pub fn directory_name(&self) -> &str {
    &self.directory_name
  }

  pub fn inherits(&self) -> &Vec<String> {
    &self.inherits
  }

  pub fn directories(&self) -> &Vec<IconDir> {
    &self.directories
  }
}

pub const FALLBACK_THEME: &str = "hicolor";

fn find_theme_directories(theme_directory: &Path) -> Option<IconTheme> {
  let mut directories = Vec::new();
  let directory_name = theme_directory.file_name()?.to_str()?.to_string();

  for entry in fs::read_dir(theme_directory).ok()? {
    let entry = match entry {
      Ok(entry) => entry,
      Err(_) => continue,
    };

    // Only process directories with the "apps" subfolder
    let apps_dir = fs::metadata(entry.path().join("apps"));
    if let Ok(apps_dir) = apps_dir {
      if !apps_dir.is_dir() {
        continue;
      }
    } else {
      continue;
    }

    let entry_name = entry.file_name().to_str()?.to_string();

    if entry_name == "scalable" {
      let mut full_path = theme_directory.join(entry_name);
      full_path.push("apps");

      directories.push(IconDir {
        size: 128,
        scale: 1,
        icon_type: IconType::Scalable {
          max_size: 256,
          min_size: 1,
        },
        full_path,
      });

      continue;
    }

    let mut chars = entry_name.chars();

    let width: String = chars.by_ref().take_while(|c| c.is_ascii_digit()).collect();
    let width: i32 = width.parse().ok()?;

    let height: String = chars.by_ref().take_while(|c| c.is_ascii_digit()).collect();
    let height: i32 = height.parse().ok()?;

    let scale: String = chars.by_ref().take_while(|c| c.is_ascii_digit()).collect();
    let scale = if !scale.is_empty() {
      scale.parse().ok()?
    } else {
      1
    };

    if width != height {
      continue;
    }

    let mut full_path = theme_directory.join(entry_name);
    full_path.push("apps");

    directories.push(IconDir {
      size: width,
      scale,
      icon_type: IconType::Threshold(2),
      full_path,
    });
  }

  if directories.is_empty() {
    return None;
  };

  let mut inherits = Vec::new();
  if directory_name != FALLBACK_THEME {
    inherits.push(FALLBACK_THEME.to_string());
  }

  Some(IconTheme {
    directory_name,
    inherits,
    directories,
  })
}

fn parse_theme_index(theme_directory: &Path) -> Option<IconTheme> {
  let theme_index = match Ini::from_file(&theme_directory.join("index.theme")) {
    Ok(theme_index) => theme_index,
    // If there was no index.theme, find directories by their name
    Err(_) => return find_theme_directories(theme_directory),
  };

  let directory_name = theme_directory.file_name()?.to_str()?.to_string();
  let mut inherits: Vec<String> = theme_index
    .get_vec("Icon Theme", "Inherits")
    .unwrap_or_else(Vec::new);

  // If the theme doesn't explicitly inherit from theb fallback, manually add it.
  if directory_name != FALLBACK_THEME && inherits.iter().any(|s| s == FALLBACK_THEME) {
    inherits.push(FALLBACK_THEME.into());
  }

  let directories: Vec<PathBuf> = theme_index.get_vec("Icon Theme", "Directories")?;

  let directories = directories
    .into_iter()
    .filter(|path| path.ends_with("apps"))
    .filter_map(|path| {
      let section_name = path.to_str()?;

      let size = theme_index.get(section_name, "Size")?;
      let scale = theme_index.get(section_name, "Scale").unwrap_or(1);

      let icon_type: Option<String> = theme_index.get(section_name, "Type");
      let icon_type = match icon_type.as_deref() {
        Some("Fixed") => IconType::Fixed,
        Some("Scalable") => {
          let max_size: i32 = theme_index.get(section_name, "MaxSize").unwrap_or(size);
          let min_size: i32 = theme_index.get(section_name, "MinSize").unwrap_or(size);

          IconType::Scalable { max_size, min_size }
        }
        Some("Threshold") | None => {
          let threshold: i32 = theme_index.get(section_name, "Threshold").unwrap_or(2);
          IconType::Threshold(threshold)
        }

        Some(_) => return None,
      };

      Some(IconDir {
        size,
        scale,
        icon_type,
        full_path: theme_directory.join(path),
      })
    });

  Some(IconTheme {
    directory_name,
    inherits,
    directories: directories.collect(),
  })
}

fn get_icon_theme_dirs(icons_dir: &Path) -> io::Result<Vec<PathBuf>> {
  let mut theme_dirs = Vec::new();
  let files = fs::read_dir(icons_dir)?;

  for entry in files.into_iter().flatten() {
    let is_dir = fs::metadata(entry.path())
      .map(|f| f.is_dir())
      .unwrap_or(false);
    if is_dir {
      theme_dirs.push(entry.path());
    }
  }

  Ok(theme_dirs)
}

pub fn get_icon_themes(data_dirs: &Vec<PathBuf>) -> HashMap<String, IconTheme> {
  let mut themes: HashMap<String, IconTheme> = HashMap::new();

  for data_dir in data_dirs {
    let theme_dirs = match get_icon_theme_dirs(&data_dir.join("icons")) {
      Ok(theme_dirs) => theme_dirs,
      Err(_) => continue,
    };

    for theme_dir in theme_dirs {
      let icon_theme = parse_theme_index(&theme_dir);

      if let Some(mut icon_theme) = icon_theme {
        if let Some(prev_theme) = themes.get_mut(icon_theme.directory_name()) {
          prev_theme.directories.append(&mut icon_theme.directories);

          for inherited in icon_theme.inherits {
            if !prev_theme.inherits.contains(&inherited) {
              prev_theme.inherits.push(inherited);
            }
          }
        } else {
          themes.insert(icon_theme.directory_name.clone(), icon_theme);
        }
      }
    }
  }

  themes
}
