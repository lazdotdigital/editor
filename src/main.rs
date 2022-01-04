use std::env;

use buffer::Buffer;
use editor::Editor;
use std::error::Error;
use syntect::highlighting::ThemeSet;

mod buffer;
mod editor;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args().nth(1).unwrap();
    let buffer = Buffer::new(path).unwrap();

    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["InspiredGitHub"];
    let editor = Editor::new(buffer, theme.to_owned());

    ui::run(editor)?;

    Ok(())
}

