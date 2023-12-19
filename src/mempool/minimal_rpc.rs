// TODO
//  - WHEN UPGRADING TO 1.0.1 Client is in hyper-utils:
//    Struct hyper_util::client::legacy::Client
//  - use https for security reasons
//  - manage id in RpcResult messages
use base64::Engine;
use bitcoin::consensus::encode::deserialize as consensus_decode;
use bitcoin::Transaction;
use hex::decode;
//use reqwest;
use hyper::{Body, Client, Request};
use serde::{Deserialize, Serialize};
use serde_json::json;
// use hyper::Response;
use hyper::client::HttpConnector;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};

use super::BlockHash;

#[derive(Clone, Debug)]
pub struct MiniRpcClient {
    client: Client<HttpConnector>,
    //url: &'a str,
    url: String,
    auth: Auth,
}

impl<'a> MiniRpcClient {
    pub fn new(url: String, auth: Auth) -> MiniRpcClient {
        let client = Client::<HttpConnector>::new();
        MiniRpcClient { client, url, auth }
    }

    pub async fn get_raw_transaction(&self, txid: &String, block_hash: Option<&BlockHash>) -> Result<Transaction, RpcError> {
        let response = match block_hash {
            Some(hash) => self.send_json_rpc_request::<serde_json::Value>("getrawtransaction", json!([txid, false, hash])),
            None => self.send_json_rpc_request::<serde_json::Value>("getrawtransaction", json!([txid, false]))
        }.await;
        match response {
            Ok(result) => {
                let result_inner: serde_json::Value = result
                    .result
                    .ok_or_else(|| RpcError::Other("Result not found".to_string()))?;
                let transaction_hex: String = match serde_json::from_value(result_inner) {
                    Ok(transaction) => transaction,
                    Err(e) => return Err(RpcError::Deserialization(e.to_string())),
                };
                let transaction_bytes = decode(transaction_hex).expect("Decoding failed");
                let transaction: Transaction =
                    consensus_decode(&transaction_bytes).expect("Deserialization failed");
                Ok(transaction)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_raw_mempool(&self) -> Result<Vec<String>, RpcError> {
        let response = self.send_json_rpc_request("getrawmempool", json!([])).await;
        match response {
            Ok(result) => {
                let result_inner = result
                    .result
                    .ok_or_else(|| RpcError::Other("Result not found".to_string()))?;
                let response_: Vec<String> = match serde_json::from_value(result_inner) {
                    Ok(response_inner) => response_inner,
                    Err(e) => return Err(RpcError::Deserialization(e.to_string())),
                };
                Ok(response_)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn submit_block(&self, block_hex: &str) -> Result<(), RpcError> {
        let response = self
            .send_json_rpc_request::<serde_json::Value>("submitblock", json!([block_hex]))
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    async fn send_json_rpc_request<T: for<'de> Deserialize<'de> + std::fmt::Debug>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<JsonRpcResult<T>, RpcError> {
        let client = &self.client;
        let (username, password) = self.auth.clone().get_user_pass();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: 1, //TODO manage message ids
        };

        let request_body = match serde_json::to_string(&request) {
            Ok(body) => body,
            Err(e) => return Err(RpcError::Deserialization(e.to_string())),
        };

        //url: &'a str,
        let req = Request::builder()
            .method("POST")
            .uri(self.url.as_str())
            .header(CONTENT_TYPE, "application/json")
            .header(
                AUTHORIZATION,
                format!(
                    "Basic {}",
                    base64::engine::general_purpose::STANDARD
                        .encode(format!("{}:{}", username, password))
                ),
            )
            .body(Body::from(request_body))
            .map_err(|e| RpcError::Http(e.to_string()))?;

        let response = client
            .request(req)
            .await
            .map_err(|e| RpcError::Http(e.to_string()))?;

        let status = response.status();
        let body = hyper::body::to_bytes(response.into_body())
            .await
            .map_err(|e| RpcError::Http(e.to_string()))?;

        if status.is_success() {
            serde_json::from_slice(&body).map_err(|e| {
                RpcError::JsonRpc(JsonRpcError::to_rpc_result(
                    &JsonRpcError {
                        code: -1,
                        message: format!("Deserialization error {:?}", e),
                    },
                    1,
                )) // TODO manage message ids
            })
        } else {
            let error_result: Result<JsonRpcResult<_>, _> = serde_json::from_slice(&body);
            match error_result {
                Ok(error_response) => Err(error_response.into()),
                Err(e) => Err(RpcError::JsonRpc(JsonRpcError::to_rpc_result(
                    &JsonRpcError {
                        code: -1,
                        message: format!("Deserialization error {:?}", e),
                    },
                    1,
                ))), // TODO mane message ids
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Auth {
    username: String,
    password: String,
}

impl Auth {
    pub fn get_user_pass(self) -> (String, String) {
        (self.username, self.password)
    }
    pub fn new(username: String, password: String) -> Auth {
        Auth { username, password }
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

impl JsonRpcError {
    fn to_rpc_result(&self, id: u64) -> JsonRpcResult<JsonRpcError> {
        JsonRpcResult {
            result: None,
            error: Some(self.clone()),
            id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum RpcError {
    JsonRpc(JsonRpcResult<JsonRpcError>),
    Deserialization(String),
    Http(String),
    Other(String),
}

impl From<JsonRpcResult<JsonRpcError>> for RpcError {
    fn from(error: JsonRpcResult<JsonRpcError>) -> Self {
        Self::JsonRpc(error)
    }
}
