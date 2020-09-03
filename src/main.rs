mod api;
mod color;
mod config;
mod request;

fn run() -> Result<i32, i32> {
    request::request()
}

fn main() {
    if let Err(code) = run() {
        std::process::exit(code);
    }
}
