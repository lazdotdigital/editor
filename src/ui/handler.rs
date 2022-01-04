use super::dom;
use crate::Editor;
use syntect::highlighting::{Color, Theme};
use web_view::{WVResult, WebView};

pub fn init(wv: &mut WebView<Editor>) -> WVResult {
    let editor = wv.user_data();

    let css = theme_bg_css(editor.theme());
    let js = dom::set_buffer(editor);

    wv.inject_css(&css)?;
    wv.eval(&js)?;

    Ok(())
}

fn theme_bg_css(theme: &Theme) -> String {
    let bg = theme.settings.background.unwrap_or(Color::WHITE);
    format!(
        "body{{background-color:#{:02x}{:02x}{:02x};}}",
        bg.r, bg.g, bg.b
    )
}

pub fn buffer_key_down(wv: &mut WebView<Editor>, key: &str, ctrl: bool) -> WVResult {
    let editor = wv.user_data_mut();
    let buffer = editor.buffer_mut();

    if !ctrl && key.len() == 1 {
        buffer.insert(key);
    } else {
        match key {
            "w" if ctrl => buffer.move_up(),
            "a" if ctrl => buffer.move_left(),
            "s" if ctrl => buffer.move_down(),
            "d" if ctrl => buffer.move_right(),
            "S" if ctrl => {
                if let Err(_) = buffer.save() {
                    unimplemented!()
                }
            }
            "Enter" => buffer.insert("\n"),
            "Tab" => buffer.insert("\t"),
            "Backspace" => buffer.delete(),
            _ => {}
        }
    };

    let js = dom::set_buffer(editor);
    wv.eval(&js)?;

    Ok(())
}

pub fn buffer_click(wv: &mut WebView<Editor>, offset: usize) -> WVResult {
    let editor = wv.user_data_mut();

    let buffer = editor.buffer_mut();
    buffer.set_pos(offset);

    let js = dom::set_buffer(editor);
    wv.eval(&js)?;

    Ok(())
}
