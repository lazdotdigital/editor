use html_escape::encode_text as html_escape;
use web_view::escape as js_escape;

use crate::editor::Editor;

const CARET_OFFSET: &str = r#"<pre id="caret-offset"></pre>"#;

pub fn set_buffer(editor: &Editor) -> String {
    let buffer = editor.buffer();
    let contents = buffer.contents();

    let highlighted = buffer.highlight(&contents, editor.theme());
    let hl_buffer = set_inner_html_by_selector("#buffer", &(highlighted + CARET_OFFSET));

    let escaped_start = html_escape(contents.split_at(buffer.pos()).0).to_string();
    let caret_offset = set_inner_html_by_selector("#caret-offset", &escaped_start);

    hl_buffer + &caret_offset
}

fn set_inner_html_by_selector(selector: &str, html: &str) -> String {
    format!(
        "document.querySelector({}).innerHTML={};",
        js_escape(selector),
        js_escape(html)
    )
}
