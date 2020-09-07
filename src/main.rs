mod api;
mod color;
mod config;
mod request;
use clap::{App, Arg};

fn run() -> Result<i32, i32> {
    let matches = App::new("clinet credentials client")
        .version("1.0")
        .author("nekomaho <nekosatoru@gmail.com>")
        .about("A API client that request parallel")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Set a custom config yaml file")
                .takes_value(true),
        )
        .get_matches();

    let config_file_name = match matches.value_of("config") {
        Some(config) => config,
        None => "",
    };

    request::request(config_file_name)
}

fn main() {
    if let Err(code) = run() {
        std::process::exit(code);
    }
}
