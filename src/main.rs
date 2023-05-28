mod app;
mod renderer;

use app::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    App::default().run()
}
