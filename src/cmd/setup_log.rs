extern crate log;
use sensible_env_logger;

pub fn setup_logger() {
    sensible_env_logger::safe_init!();
}
