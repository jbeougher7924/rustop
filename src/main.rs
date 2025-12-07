mod app;
mod cpu;
mod gpu;
mod ui;
mod utilities;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
