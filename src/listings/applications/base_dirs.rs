use std::path::PathBuf;

use xdg::BaseDirectories;

pub fn get_data_dirs(env: &BaseDirectories) -> Vec<PathBuf> {
  let mut data_dirs: Vec<PathBuf> = vec![];
  data_dirs.push(env.get_data_home());
  data_dirs.append(&mut env.get_data_dirs());

  data_dirs
}
