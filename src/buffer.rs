use ropey::Rope;
use std::{fs, io};
use syntect::{
    easy::HighlightLines,
    highlighting::Theme,
    html::{
        append_highlighted_html_for_styled_line, start_highlighted_html_snippet, IncludeBackground,
    },
    parsing::{SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};

pub struct Buffer {
    path: String,
    pos: usize,
    rope: Rope,

    syntax_set: SyntaxSet,
    syntax_reference: SyntaxReference,
}

impl Buffer {
    pub fn new(path: String) -> io::Result<Self> {
        let text = fs::read_to_string(&path)?;
        let rope = Rope::from_str(&text);

        let syntax_set = SyntaxSet::load_defaults_newlines();
        let extension = get_extension(&path);
        let syntax_reference = match syntax_set.find_syntax_by_extension(extension) {
            Some(sr) => sr,
            None => unimplemented!(),
        }
        .to_owned();

        Ok(Self {
            path,
            pos: 0,
            rope,
            syntax_set,
            syntax_reference,
        })
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos
    }

    pub fn move_up(&mut self) {
        let line_idx = self.rope.char_to_line(self.pos);
        if line_idx > 0 {
            let pos = self.rope.line_to_char(line_idx - 1);
            self.pos = pos
        }
    }

    pub fn move_down(&mut self) {
        let line_idx = self.rope.char_to_line(self.pos);
        if line_idx < self.rope.len_lines() {
            let pos = self.rope.line_to_char(line_idx + 1);
            self.pos = pos
        }
    }

    pub fn move_left(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.pos < self.rope.len_chars() {
            self.pos += 1;
        }
    }

    pub fn insert(&mut self, text: &str) {
        self.rope.insert(self.pos, text);
        self.pos += text.len();
    }

    pub fn delete(&mut self) {
        if self.pos > 0 {
            self.rope.remove(self.pos - 1..self.pos);
            self.pos -= 1;
        }
    }

    pub fn contents(&self) -> String {
        self.rope.to_string()
    }

    pub fn highlight(&self, s: &str, theme: &Theme) -> String {
        let mut highlighter = HighlightLines::new(&self.syntax_reference, theme);
        let (mut output, bg) = start_highlighted_html_snippet(theme);

        for (i, line) in LinesWithEndings::from(s).enumerate() {
            let regions = highlighter.highlight(line, &self.syntax_set);
            let line_html = format!(r#"<span class="line" num="{}"></span>"#, i + 1);
            output.push_str(&line_html);
            append_highlighted_html_for_styled_line(
                &regions[..],
                IncludeBackground::IfDifferent(bg),
                &mut output,
            )
        }
        output.push_str("</pre>\n");
        output
    }

    pub fn save(&self) -> io::Result<()> {
        fs::write(&self.path, self.contents())
    }
}

fn get_extension(path: &str) -> &str {
    path.split(".").last().unwrap()
}
