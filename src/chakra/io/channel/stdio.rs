extern crate tokio;

pub struct STDIOChannel;

use tokio::io::stdout;
use tokio::io::Write;

impl STDIOChannel {
    pub fn write(&self, content: String) {
        stdout().write(&content.into_bytes());
    }
}
