mod render;
mod calendar;
mod utils;

use color_eyre::Result;
use ratatui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let instance = calendar::Calendar::new();
    let result = instance.run(terminal);
    ratatui::restore();
    return result;
}

