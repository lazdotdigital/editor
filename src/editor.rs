use syntect::highlighting::Theme;

use crate::Buffer;

pub struct Editor {
    buffer: Buffer,
    theme: Theme,
}

impl Editor {
    pub fn new(buffer: Buffer, theme: Theme) -> Self {
        Self { buffer, theme }
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}
