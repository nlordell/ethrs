//! Module contains implementation for instantiating `namespace`s with a little
//! help from macros.

/// Define a JSON RPC namespace with support for Ethereum specific types.
#[macro_export]
macro_rules! api {
    ($($x:tt)*) => {};
}

api! {
    namespace Web3 [
        eth => Eth,
    ] {
        web3_clientVersion as client_version() -> String;
        web3_sha3 as sha3(bytes: DATA) -> DATA;
    }

    namespace Net [] {
        net_version as version() -> String;
        net_listening as listening() -> Boolean;
        net_peerCount as peer_count() -> QUANTITY[usize];
    }

    namespace Eth [
        net => Net,
    ] {
        eth_protocolVersion as protocol_version() -> String;
        eth_syncing as syncing() -> Syncing|Boolean;
    }
}
