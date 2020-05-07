//! Module contains implementation for instantiating `namespace`s with a little
//! help from macros.

#[macro_use]
mod api;
pub mod encoding;
pub mod models;
mod provider;
pub mod serialization;

pub use self::api::{Api, ApiError};
use self::encoding::{Data, Quantity};
use self::models::*;
pub use self::provider::Provider;
use self::serialization::MaybeSyncing;

api! {
    module Web3 [
        eth => Eth,
    ] {
        web3_clientVersion as client_version() -> String;
        web3_sha3 as sha3(bytes: impl AsRef<[u8]> [ Data<_> ]) -> [u8; 32] [ Data<_> ];
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
        eth_syncing as syncing() -> Option<Syncing> [ MaybeSyncing ];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    api_test! {
        web3_clientVersion as Web3::client_version {
            (): json!(null) => json!("test"), == "test";
        }
        web3_sha3 as Web3::sha3 {
            ([4, 2]): json!(["0x0402"])
                => json!("0x0000000000000000000000000000000000000000000000000000000000000000"),
                == [0; 32];
        }

        net_version as Net::version {
            (): json!(null) => json!("42"), == "42";
        }
        net_listening as Net::listening {
            (): json!(null) => json!(true), == true;
        }
        net_peerCount as Net::peer_count {
            (): json!(null) => json!("0x2a"), == 42;
        }

        eth_protocolVersion as Eth::protocol_version {
            (): json!(null) => json!("0x42"), == "0x42";
        }
        eth_syncing as Eth::syncing {
            (): json!(null)
                => json!({
                    "startingBlock": "0x384",
                    "currentBlock": "0x386",
                    "highestBlock": "0x454",
                }),
                == Some(Syncing {
                    starting_block: 0x384,
                    current_block: 0x386,
                    highest_block: 0x454,
                });
            (): json!(null) => json!(false), == None;
        }
    }
}
