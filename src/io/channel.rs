pub mod stdio;

use crate::io::channel::stdio::STDIOChannel;

pub enum Channel {
    STDIO(STDIOChannel)
}

impl From<STDIOChannel> for Channel {
    fn from(channel: STDIOChannel) -> Self {
        Self::STDIO(channel)
    }
}

impl Channel {
    pub fn write(&self, content: String) {
        match &*self {
            Self::STDIO(channel) => channel.write(content)
        }
    }
}
