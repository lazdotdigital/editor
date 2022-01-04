use crate::Editor;
use serde::Deserialize;
use web_view::{WVResult, WebView};

use super::handler;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Event {
    Init,
    BufferKeyDown { key: String, ctrl: bool },
    BufferClick { offset: usize },
}

impl Event {
    pub fn dispatch(&self, wv: &mut WebView<Editor>) -> WVResult {
        match self {
            Self::Init => handler::init(wv),
            Self::BufferKeyDown { key, ctrl } => handler::buffer_key_down(wv, key, *ctrl),
            Self::BufferClick { offset } => handler::buffer_click(wv, *offset),
        }
    }
}
