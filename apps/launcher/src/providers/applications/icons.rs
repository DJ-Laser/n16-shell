use std::{
  collections::{HashMap, HashSet},
  fs, io,
  path::{Path, PathBuf},
};

use freedesktop_desktop_entry::DesktopEntry;

use super::icon_theme::{FALLBACK_THEME, IconTheme, IconType};

fn find_icon_in_dir(icon_name: &str, dir: &Path) -> io::Result<Option<PathBuf>> {
  for entry in fs::read_dir(dir)? {
    let path = entry?.path();

    let file_stem = match path.file_stem() {
      Some(file_stem) => file_stem,
      None => return Ok(None),
    };

    if file_stem == icon_name {
      return Ok(Some(path));
    }
  }

  Ok(None)
}

fn get_icon_for_theme<'s>(
  icon_name: &str,
  desired_theme: &'s str,
  icon_themes: &'s HashMap<String, IconTheme>,
  searched_themes: &mut HashSet<&'s str>,
) -> Option<PathBuf> {
  if let Some(icon_theme) = icon_themes.get(desired_theme) {
    let desired_size = 48;
    let mut best_icon = None;
    let mut best_size_delta = i32::MAX;

    for size_dir in icon_theme.directories() {
      let icon = match find_icon_in_dir(icon_name, size_dir.full_path()) {
        Ok(Some(icon)) => icon,
        Ok(None) | Err(_) => continue,
      };

      let size_delta = match size_dir.icon_type() {
        IconType::Fixed => {
          let icon_size = size_dir.size() * size_dir.scale();
          (icon_size - desired_size).abs()
        }
        IconType::Threshold(threshold) => {
          let icon_size = size_dir.size() * size_dir.scale();
          let mut size_delta = (icon_size - desired_size).abs();
          if size_delta <= *threshold {
            size_delta = 0;
          }

          size_delta
        }
        IconType::Scalable { max_size, min_size } => {
          if desired_size < *min_size {
            (min_size - desired_size).abs()
          } else if desired_size > *max_size {
            (max_size - desired_size).abs()
          } else {
            0
          }
        }
      };

      if size_delta < best_size_delta {
        best_icon = Some(icon);
        best_size_delta = size_delta;
      }
    }

    if let Some(icon) = best_icon {
      return Some(icon);
    }

    // If the icon was not found, search it's inherited themes
    for inherited in icon_theme.inherits() {
      if searched_themes.contains(&inherited.as_str()) {
        continue;
      }

      if let Some(icon) = get_icon_for_theme(icon_name, inherited, icon_themes, searched_themes) {
        return Some(icon);
      }

      searched_themes.insert(inherited);
    }

    searched_themes.insert(desired_theme);
  }

  if !searched_themes.contains(FALLBACK_THEME) {
    return get_icon_for_theme(icon_name, FALLBACK_THEME, icon_themes, searched_themes);
  }

  None
}

pub fn get_icon(
  entry: &DesktopEntry,
  desired_theme: &str,
  icon_themes: &HashMap<String, IconTheme>,
  data_dirs: &[PathBuf],
) -> Option<PathBuf> {
  let icon_name = entry.icon()?;

  if icon_name.starts_with("/") {
    // Icon entry is an absolute path to the icon file
    return Some(PathBuf::from(icon_name));
  }

  // Search in XDG icon dirs
  let icon = get_icon_for_theme(icon_name, desired_theme, icon_themes, &mut HashSet::new());
  if let Some(icon) = icon {
    return Some(icon);
  }

  // Use icons in the pixmaps dir as a fallback
  for data_dir in data_dirs.iter() {
    let icon = match find_icon_in_dir(icon_name, &data_dir.join("pixmaps")) {
      Ok(Some(icon)) => icon,
      Ok(None) | Err(_) => continue,
    };

    return Some(icon);
  }

  None
}
