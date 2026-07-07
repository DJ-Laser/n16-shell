use clap::CommandFactory;
use clap_complete::{generate_to, shells::Bash};
use std::env;
use std::io::Error;

mod src {
  #![allow(unused, reason = "Build script does not use all of the src module")]
  mod cli;

  pub use cli::Cli;
}

fn main() -> Result<(), Error> {
  // Priotitize `N16_COMPLETION_OUT_DIR` for completion when building the nix package
  let Some(outdir) = env::var_os("N16_COMPLETION_OUT_DIR").or(env::var_os("OUT_DIR")) else {
    return Ok(());
  };

  let mut cmd = src::Cli::command();
  let path = generate_to(Bash, &mut cmd, "n16", outdir)?;

  println!(
    "cargo:warning=completion file is generated: {}",
    path.display()
  );

  Ok(())
}
