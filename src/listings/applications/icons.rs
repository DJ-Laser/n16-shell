use std::{
  fs, io,
  path::{Path, PathBuf},
};

use freedesktop_desktop_entry::DesktopEntry;

use super::icon_theme::IconTheme;

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

pub fn get_icon<'a>(
  entry: &DesktopEntry,
  icon_themes: &Vec<IconTheme>,
  data_dirs: &Vec<PathBuf>,
) -> Option<PathBuf> {
  let icon_name = entry.icon()?;

  if icon_name.starts_with("/") {
    // Icon entry is an absolute path to the icon file
    return Some(PathBuf::from(icon_name));
  }

  // Search in XDG icon dirs
  for icon_theme in icon_themes {
    for size_dir in icon_theme.directories() {
      let icon = match find_icon_in_dir(icon_name, size_dir.full_path()) {
        Ok(Some(icon)) => icon,
        Ok(None) | Err(_) => continue,
      };

      return Some(icon);
    }
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
