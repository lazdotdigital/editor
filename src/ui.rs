mod dom;
mod event;
mod handler;

use crate::Editor;
use event::Event;
use web_view::{Content, WVResult};

const CONTENT: Content<&str> = Content::Html(include_str!("./ui/content.html"));

pub fn run(editor: Editor) -> WVResult<Editor> {
    web_view::builder()
        .title("Editor")
        .content(CONTENT)
        .size(800, 600)
        .resizable(true)
        // .frameless(true)
        .debug(true)
        .user_data(editor)
        .invoke_handler(|wv, arg| serde_json::from_str::<Event>(arg).unwrap().dispatch(wv))
        .run()
}
