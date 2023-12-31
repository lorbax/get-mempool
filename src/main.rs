use base64::Engine;
use bitcoin::consensus::encode::deserialize as consensus_decode;
use bitcoin::Transaction;
use bytes::Bytes;
use hex::decode;
use http_body_util::{BodyExt, Full};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::Request;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() {
    let response = send_json_rpc_request("getrawmempool", json!([])).await;
    match response {
        Ok(result) => {
            let response_deserialized: JsonRpcResult<Vec<String>> =
                serde_json::from_str(&result).unwrap();
            let result_inner = response_deserialized.result.unwrap();
            println!("Transactions: {:?}", result_inner);
            let txid = &result_inner[0];
            let response_tx_hex = send_json_rpc_request("getrawtransaction", json!([txid, false]))
                .await
                .unwrap();
            let response_tx: JsonRpcResult<String> =
                serde_json::from_str(&response_tx_hex).unwrap();
            let transaction_hex = response_tx.result.unwrap();
            let transaction_bytes = decode(transaction_hex).expect("Decoding failed");
            let transaction: Transaction =
                consensus_decode(&transaction_bytes).expect("Deserialization failed");
            println!("First transaction: {:?}", transaction);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

async fn send_json_rpc_request(
    method: &str,
    params: serde_json::Value,
) -> Result<String, JsonRpcError> {
    let url = "http://127.0.0.1:18332".parse::<hyper::Uri>().unwrap();
    let (username, password) = ("username", "password");
    let client: Client<_, Full<Bytes>> = Client::builder(TokioExecutor::new()).build_http();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: method.to_string(),
        params,
        id: 1,
    };

    let request_body = serde_json::to_string(&request).unwrap();

    let req = Request::builder()
        .method("POST")
        .uri(url)
        .header(CONTENT_TYPE, "application/json")
        .header(
            AUTHORIZATION,
            format!(
                "Basic {}",
                base64::engine::general_purpose::STANDARD
                    .encode(format!("{}:{}", username, password))
            ),
        )
        .body(Full::<Bytes>::from(request_body))
        .unwrap();

    let response = client.request(req).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();

    if status.is_success() {
        Ok(String::from_utf8(body.to_vec()).unwrap())
    } else {
        match serde_json::from_slice(&body) {
            Ok(error_response) => Err(error_response),
            Err(e) => Err(JsonRpcError {
                code: -1,
                message: format!("Deserialization error {:?}", e),
            }),
        }
    }
}
#[derive(Debug, Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: u64,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcResult<T> {
    result: Option<T>,
    error: Option<JsonRpcError>,
    id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JsonRpcError {
    code: i32,
    message: String,
}
