extern crate clap;
use clap::{App, Arg};

extern crate lemoned_naglfar;
use lemoned_naglfar::renderer;

const VERSION_STR: &'static str = env!("CARGO_PKG_VERSION");

fn main() {

    let app = App::new("lemoned_naglfar")
        .version(VERSION_STR)
        .author("mk-tool")
        .about("lemoned_naglfar is a skeleton naglfar by uint256_t")
        .arg(Arg::with_name("FILE").help("Input file").index(1));
    let _app_matches = app.get_matches();

    renderer::f();
}