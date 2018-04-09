use std::path::PathBuf;

use cumulus_sync::Options;
use cumulus_sync::api::Auth;

#[derive(Debug, StructOpt)]
#[structopt(name = "cumulus_sync", about = "Files sync for Cumulus")]
pub struct Cli {
  #[structopt(short = "l", long = "login")]
  pub login: String,

  #[structopt(short = "p", long = "password")]
  pub password: String,

  #[structopt(short = "f", long = "folder", parse(from_os_str))]
  pub folder: PathBuf,

  #[structopt(short = "s", long = "server")]
  pub server: String,
}

impl From<Cli> for Options {
  fn from(cli: Cli) -> Options {
    Options {
      server_url: cli.server,
      auth: Auth::new(&cli.login, &cli.password),
      folder: cli.folder,
    }
  }
}