// Explanation for main(...) args - https://doc.rust-lang.org/beta/unstable-book/language-features/start.html
// *const reference - https://doc.rust-lang.org/std/primitive.pointer.html

#[macro_use]
extern crate log;

mod game;
mod server;

// Run as Executable (e.g. Linux)
fn main() {
    game::get_args();
    game::setup_logger();
    debug!("Launched as binary...");

    game::launch();
}