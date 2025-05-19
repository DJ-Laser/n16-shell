use std::{env, fs, path::PathBuf};

pub use config::*;

mod config;

pub fn find_config_file() -> Option<PathBuf> {
  let mut config_dir = {
    if let Some(path) = (env::var_os("XDG_CONFIG_DIR")).filter(|path| !path.is_empty()) {
      PathBuf::from(path)
    } else if let Some(path) = env::var_os("HOME") {
      let mut path = PathBuf::from(path);
      path.push(".config");

      path
    } else {
      return None;
    }
  };

  config_dir.push("n16-shell/config.kdl");
  println!("{:?}", config_dir);
  Some(config_dir)
}

pub fn load_config_file() -> Option<Config> {
  let Some(config_path) = find_config_file() else {
    eprintln!("$HOME unset, using default config");
    return None;
  };

  let text = fs::read_to_string(config_path).ok()?;

  let config: Result<Config, _> = knuffel::parse("config.kdl", &text);

  match config {
    Ok(config) => Some(config),
    Err(error) => {
      eprintln!("{:?}", miette::Report::new(error));
      None
    }
  }
}
