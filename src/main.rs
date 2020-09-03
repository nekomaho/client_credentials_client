mod api;
mod color;
mod config;
mod request;

fn run() -> Result<i32, i32> {
    request::request()
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(code) => {
            std::process::exit(code);
        }
    };
}
