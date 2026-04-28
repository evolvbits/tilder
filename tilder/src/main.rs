// (c) 2026 EvolvBits. All rights reserved.
//! Entry point for the `tilder` command-line application.

mod constants;
use clap::{ArgAction, Parser};
use tilder_core::{
  meta::{CreditsInfo, credits_string},
  traits::Capitalize,
};

#[derive(Parser)]
#[command(
  name = constants::APP_NAME,
  version = constants::APP_VERSION,
  about = constants::APP_DESCRIPTION,
  after_help = "\
EXAMPLES:
    $ tilder
\n"
)]
struct Cli {
  #[arg(help = "", num_args = 1..)]
  targets: Vec<String>,

  #[arg(short = 'c', long = "credits", action = ArgAction::SetTrue, help = "show credits")]
  credits: bool,
}

// tilder --credits   → nome, version, authors, maintainer
// tilder --info      → repositories, commit, date
// tilder --legal     → copyright, site, license

fn main() {
  let cli = Cli::parse();
  if cli.credits {
    let credits_info = CreditsInfo {
      name: &"tilder".capitalize(),
      version: env!("CARGO_PKG_VERSION"),
      maintainer: env!("PROJECT_MAINTAINER"),
      repository: env!("CARGO_PKG_REPOSITORY"),
      commit: env!("GIT_COMMIT"),
      date: env!("GIT_DATE"),
      license: env!("APP_COPYRIGHT"),
      site: env!("PROJECT_SITE"),
    };
    println!("{}", &credits_string(&credits_info));
    return;
  }
  println!("tilder: coming soon!");
}
