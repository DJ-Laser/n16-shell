use clap::CommandFactory;
use clap_complete::{generate_to, shells::Bash};
use std::env;
use std::io::Error;

mod src {
  #![allow(unused)]
  mod cli;

  pub use cli::Cli;
}

fn main() -> Result<(), Error> {
  // Priotitize `N16_COMPLETION_OUT_DIR` for completion when building the nix package
  let outdir = match env::var_os("N16_COMPLETION_OUT_DIR").or(env::var_os("OUT_DIR")) {
    Some(outdir) => outdir,
    None => return Ok(()),
  };

  let mut cmd = src::Cli::command();
  let path = generate_to(Bash, &mut cmd, "n16", outdir)?;

  println!("cargo:warning=completion file is generated: {path:?}");

  Ok(())
}
