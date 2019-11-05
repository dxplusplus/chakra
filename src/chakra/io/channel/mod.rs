pub mod stdio;
pub mod tcp;
pub mod udp;
pub mod unix;

use crate::chakra::io::channel::stdio::STDIOChannel;
use crate::chakra::io::channel::tcp::TCPChannel;
use crate::chakra::io::channel::udp::UDPChannel;
use crate::chakra::io::channel::unix::UnixChannel;

pub enum Channel {
    STDIO(STDIOChannel),
    TCP(TCPChannel),
    UDP(UDPChannel),
    Unix(UnixChannel)
}

impl From<STDIOChannel> for Channel {
    fn from(channel: STDIOChannel) -> Self {
        Channel::STDIO(channel)
    }
}

impl From<TCPChannel> for Channel {
    fn from(channel: TCPChannel) -> Self {
        Channel::TCP(channel)
    }
}

impl From<UDPChannel> for Channel {
    fn from(channel: UDPChannel) -> Self {
        Channel::UDP(channel)
    }
}

impl From<UnixChannel> for Channel {
    fn from(channel: UnixChannel) -> Self {
        Channel::Unix(channel)
    }
}

impl Channel {
    pub fn write(&self, content: String) {
        match &*self {
            Channel::STDIO(channel) => channel.write(content),
            Channel::TCP(channel) => channel.write(content),
            Channel::UDP(channel) => channel.write(content),
            Channel::Unix(channel) => channel.write(content)
        }
    }
}
