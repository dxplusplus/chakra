pub struct STDIOChannel;

use std::io::stdout;
use std::io::Write;

impl STDIOChannel {
    pub fn write(&self, content: String) {
        stdout().write(&content.into_bytes());
    }
}