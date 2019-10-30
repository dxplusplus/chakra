pub mod stdio;

use crate::chakra::io::channel::stdio::STDIOChannel;

pub enum Channel {
    STDIO(STDIOChannel)
}

impl From<STDIOChannel> for Channel {
    fn from(channel: STDIOChannel) -> Self {
        Channel::STDIO(channel)
    }
}

impl Channel {
    pub fn write(&self, content: String) {
        match &*self {
            Channel::STDIO(channel) => channel.write(content)
        }
    }
}
