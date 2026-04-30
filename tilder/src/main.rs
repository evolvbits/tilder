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

  #[arg(short = 'c', long = "credits", action = ArgAction::SetTrue, help = "Show credits")]
  credits: bool,
}

fn main() {
  let cli = Cli::parse();
  if cli.credits {
    let credits_info = CreditsInfo {
      name: &"tilder".capitalize(),
      version: constants::APP_VERSION,
      maintainer: constants::APP_MAINTAINER,
      repository: constants::APP_REPOSITORY,
      license: constants::APP_LICENSE,
      commit: constants::APP_COMMIT,
      date: constants::APP_LAST_UPDATE,
      copyright: constants::APP_COPYRIGHT,
      homepage: constants::APP_HOMEPAGE,
    };
    println!("{}", &credits_string(&credits_info));
    return;
  }
  println!("{}: Coming Soon!", constants::APP_NAME.capitalize());
}
