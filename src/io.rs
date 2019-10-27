pub mod channel;
pub mod encoding;

use crate::io::channel::Channel;
use crate::io::encoding::Encoding;

pub struct IO {
    channel: Channel,
    encoding: Encoding
}

impl IO {
    pub fn new(channel: Channel, encoding: Encoding) -> Self {
        Self { channel, encoding }
    }

    pub fn write(&self, content: String) {
        self.channel.write(content);
    }
}