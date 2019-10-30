mod chakra;

use crate::chakra::io::IO;
use crate::chakra::io::channel::Channel;
use crate::chakra::io::channel::stdio::STDIOChannel;
use crate::chakra::io::encoding::Encoding;
use crate::chakra::io::encoding::jsonrpc::JSONRPCEncoding;
use crate::chakra::server::Server;
use std::env::{args, Args};
use std::env::{vars, Vars};
use std::io::Result;

fn main() -> Result<()> {
    let env: Vars = vars();
    let args: Args = args();
    // @TODO: Get all user settings
    // let config = ...readConfiguration(env, args);

    // @TODO: Determine channel + encoding instead of assuming stdio+jsonrpc
    // let channel = ...determineIOChannel(config);
    // let encoding = ...determineIOEncoding(config);

    let channel: Channel = Channel::from(STDIOChannel{});
    let encoding: Encoding = Encoding::from(JSONRPCEncoding{});
    let io: IO = IO::new(channel, encoding);

    let server: Server = Server::new(io);

    server.run();

    Ok(())
}
