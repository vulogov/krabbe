use better_panic;
pub mod cmd;
pub mod stdlib;

fn main() {
    better_panic::install();
    cmd::init();
}
