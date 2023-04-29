extern crate log;
extern crate hostname;
use clap::{Args, Parser, Subcommand};

pub mod setup_log;
pub mod sanity;

use crate::stdlib::genid;

pub mod krabbe_version;

pub fn init() {
    let cli = Cli::parse();
    setup_log::setup_logger();
    log::debug!("KRABBE messaging system");
    log::trace!("init() reached");
    sanity::check_sanity(&cli);
    match &cli.command {
        Commands::Version(_) => {
            krabbe_version::run_version(&cli);
        }
    }
}

#[derive(Parser, Clone)]
#[clap(name = "krabbe")]
#[clap(author = "Vladimir Ulogov <vladimir@ulogov.us>")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "KRABBE messaging system", long_about = None)]
pub struct Cli {
    #[clap(short, long, action = clap::ArgAction::Count, help="Increase verbosity")]
    pub debug: u8,

    #[clap(short, long, default_value_t = String::from(genid::generate_host_id()), help="Instance name")]
    pub name: String,

    #[clap(help="Hostname for krabbe", long, default_value_t = String::from(hostname::get().unwrap().into_string().unwrap()))]
    pub hostname: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Display version details")]
struct Version {

}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Version(Version),
}
