//! Module contains implementation for instantiating `namespace`s with a little
//! help from macros.

/// Define a JSON RPC namespace with support for Ethereum specific types.
#[macro_export]
macro_rules! api {
    ($(
        $(#[$attr:meta])*
        module $ns:ident [$(
            $(#[$subns_attr:meta])*
            $subns:ident => $subns_type:ty,
        )*] {$(
            $(#[$method_attr:meta])*
            $method_name:ident as $method:ident ($(
                $param:ident : $param_type:ty $([ $param_serde:ty ])?
            ),* $(,)?) -> $result_type:ty $([ $result_serde:ty ])?;
        )*}
    )*) => {$(
        $(#[$attr])*
        pub struct $ns;

        impl $ns {$(
            $(#[$subns_attr])*
            pub fn $subns(&self) -> $subns_type {
                todo!()
            }
        )* $(
            $(#[$method_attr])*
            pub async fn $method($(
                $param: $param_type,
            )*) -> Result<$result_type, ()> {
                $(
                    let _ = $param;
                )*
                todo!()
            }
        )*}
    )*};
}

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

pub struct Syncing {
    pub starting_block: u64,
    pub current_block: u64,
    pub highest_block: u64,
}
