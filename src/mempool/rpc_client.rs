// TODO
//  - WHEN UPGRADING TO 1.0.1 Client is in hyper-utils:
//    Struct hyper_util::client::legacy::Client
//  - use https for security reasons
//  - manage id in RpcResult messages
use base64::Engine;
use bitcoin::consensus::encode::deserialize as consensus_decode;
use bitcoin::Transaction;
use bytes::Bytes;
use hex::decode;
use http_body_util::{BodyExt, Full};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::Request;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::BlockHash;

#[derive(Clone, Debug)]
pub struct RpcClient {
    client: Client<HttpConnector, Full<Bytes>>,
    //url: &'a str,
    url: String,
    auth: Auth,
}

impl RpcClient {
    pub fn new(url: String, auth: Auth) -> RpcClient {
        let client: Client<_, Full<Bytes>> = Client::builder(TokioExecutor::new()).build_http();
        RpcClient { client, url, auth }
    }

    pub async fn get_raw_transaction(
        &self,
        txid: &String,
        block_hash: Option<&BlockHash>,
    ) -> Result<Transaction, RpcError> {
        let response = match block_hash {
            Some(hash) => {
                self.send_json_rpc_request("getrawtransaction", json!([txid, false, hash]))
            }
            None => self.send_json_rpc_request("getrawtransaction", json!([txid, false])),
        }
        .await;
        match response {
            Ok(result_hex) => {
                let result_deserialized: JsonRpcResult<String> = serde_json::from_str(&result_hex)
                    .map_err(|e| {
                        RpcError::Deserialization(e.to_string()) // TODO manage message ids
                    })?;
                let transaction_hex: String = result_deserialized
                    .result
                    .ok_or_else(|| RpcError::Other("Result not found".to_string()))?;
                let transaction_bytes = decode(transaction_hex).expect("Decoding failed");
                Ok(consensus_decode(&transaction_bytes).expect("Deserialization failed"))
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_raw_mempool_verbose(&self) -> Result<Vec<String>, RpcError> {
        let response = self.send_json_rpc_request("getrawmempool", json!([])).await;
        match response {
            Ok(result_hex) => {
                let result_deserialized: JsonRpcResult<Vec<String>> =
                    serde_json::from_str(&result_hex).map_err(|e| {
                        RpcError::Deserialization(e.to_string()) // TODO manage message ids
                    })?;
                let mempool: Vec<String> = result_deserialized
                    .result
                    .ok_or_else(|| RpcError::Other("Result not found".to_string()))?;
                Ok(mempool)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn submit_block(&self, block_hex: &str) -> Result<(), RpcError> {
        let response = self
            .send_json_rpc_request("submitblock", json!([block_hex]))
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    async fn send_json_rpc_request(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<String, RpcError> {
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
            Err(e) => return Err(RpcError::Serialization(e.to_string())),
        };

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
            .body(Full::<Bytes>::from(request_body))
            .map_err(|e| RpcError::Http(e.to_string()))?;

        let response = client
            .request(req)
            .await
            .map_err(|e| RpcError::Http(e.to_string()))?;

        let status = response.status();
        let body = response
            .into_body()
            .collect()
            .await
            .map_err(|e| RpcError::Http(e.to_string()))?
            .to_bytes()
            .to_vec();

        if status.is_success() {
            String::from_utf8(body).map_err(|e| {
                RpcError::Deserialization(e.to_string()) // TODO manage message ids
            })
        } else {
            let error_result: Result<JsonRpcResult<_>, _> = serde_json::from_slice(&body);
            match error_result {
                Ok(error_response) => Err(error_response.into()),
                Err(e) => Err(RpcError::Deserialization(e.to_string())),
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

#[derive(Debug, Deserialize)]
pub enum RpcError {
    JsonRpc(JsonRpcResult<JsonRpcError>),
    Deserialization(String),
    Serialization(String),
    Http(String),
    Other(String),
}

impl From<JsonRpcResult<JsonRpcError>> for RpcError {
    fn from(error: JsonRpcResult<JsonRpcError>) -> Self {
        Self::JsonRpc(error)
    }
}
