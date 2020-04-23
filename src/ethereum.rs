//! Module contains implementation for instantiating `namespace`s with a little
//! help from macros.

#[macro_use]
mod api;
mod model;
mod provider;

pub use self::api::{Api, ApiError};
pub use self::model::*;
pub use self::provider::Provider;
use crate::encoding::{Data, Quantity};

api! {
    module Web3 [
        eth => Eth,
    ] {
        web3_clientVersion as client_version() -> String;
        web3_sha3 as sha3(bytes: impl AsRef<[u8]> [ Data<_> ]) -> Vec<u8> [ Data<_> ];
    }

    module Net [] {
        net_version as version() -> String;
        net_listening as listening() -> bool;
        net_peerCount as peer_count() -> usize [ Quantity<_> ];
    }

    module Eth [
        net => Net,
    ] {
        eth_protocolVersion as protocol_version() -> String;
        eth_syncing as syncing() -> Option<Syncing>;
    }
}

#[cfg(test)]
mod tests {}
