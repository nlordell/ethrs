use ethrs_transport::Transport2;
use ethrs_transport_http::{Error, Http};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn chain_id() {
    let transport = Http::new(concat!(
        "https://mainnet.infura.io/v3/",
        env!("INFURA_PROJECT_ID"),
    ));
    let answer = transport
        .call(br#"{"jsonrpc":"2.0","id":42,"method":"eth_chainId","params":[]}"#)
        .await
        .unwrap();

    assert_eq!(answer, &br#"{"jsonrpc":"2.0","id":42,"result":"0x1"}"#[..]);
}

#[wasm_bindgen_test]
async fn http_error() {
    let transport = Http::new("invalid://url");
    let error = transport.call(b"").await.unwrap_err();

    assert_ne!(error, Error::unknown());
}
