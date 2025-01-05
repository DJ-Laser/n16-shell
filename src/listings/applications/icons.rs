use std::{
  fs::{self, FileType},
  io,
  path::PathBuf,
};

use freedesktop_desktop_entry::DesktopEntry;

fn get_size_dirs(theme_dir: PathBuf) -> io::Result<Vec<PathBuf>> {
  let mut scale_dirs = Vec::new();
  let files = fs::read_dir(theme_dir)?;

  for entry in files {
    if let Ok(entry) = entry {
      let is_dir = entry
        .file_type()
        .as_ref()
        .map(FileType::is_dir)
        .unwrap_or(false);

      if is_dir {
        scale_dirs.push(entry.path());
      }
    }
  }

  Ok(scale_dirs)
}

fn find_icon_in_dir(icon_name: &str, dir: PathBuf) -> io::Result<Option<PathBuf>> {
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

pub fn get_icon<'a>(entry: &DesktopEntry, data_dirs: &Vec<PathBuf>) -> Option<PathBuf> {
  let icon_name = entry.icon()?;

  if icon_name.starts_with("/") {
    // Icon entry is an absolute path to the icon file
    return Some(PathBuf::from(icon_name));
  }

  // Search in XDG icon dirs
  for data_dir in data_dirs.iter() {
    let size_dirs = match get_size_dirs(data_dir.join("icons/hicolor")) {
      Ok(size_dirs) => size_dirs,
      Err(_) => continue,
    };

    for mut size_dir in size_dirs {
      size_dir.push("apps");

      let icon = match find_icon_in_dir(icon_name, size_dir) {
        Ok(Some(icon)) => icon,
        Ok(None) | Err(_) => continue,
      };

      return Some(icon);
    }
  }

  // Use icons in the pixmaps dir as a fallback
  for data_dir in data_dirs.iter() {
    let icon = match find_icon_in_dir(icon_name, data_dir.join("pixmaps")) {
      Ok(Some(icon)) => icon,
      Ok(None) | Err(_) => continue,
    };

    return Some(icon);
  }

  None
}
