#[macro_use]
extern crate log;

#[macro_use]
extern crate cfg_if;

use log::LevelFilter;
use simple_logger::SimpleLogger;
use smol::io;

mod app;
use app::App;

fn main() -> io::Result<()> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .expect("A logger was already initialized");

    let mut app = App::new();
    loop {
        app.update();
    }
}
