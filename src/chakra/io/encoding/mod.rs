pub mod jsonrpc;

use jsonrpc::JSONRPCEncoding;

pub enum Encoding {
    JSONRPC(JSONRPCEncoding)
}

impl From<JSONRPCEncoding> for Encoding {
    fn from(encoding: JSONRPCEncoding) -> Self {
        Encoding::JSONRPC(encoding)
    }
}
