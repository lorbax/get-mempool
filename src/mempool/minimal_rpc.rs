// TODO
//  - WHEN UPGRADING TO 1.0.1 Client is in hyper-utils:
//    Struct hyper_util::client::legacy::Client
//  - use https for security reasons
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

#[derive(Clone, Debug)]
pub struct RpcClient<'a> {
    client: Client<HttpConnector>,
    url: &'a str,
    auth: Auth<'a>,
}

impl<'a> RpcClient<'a> {
    pub fn new(url: &'a str, auth: Auth<'a>) -> RpcClient<'a> {
        let client = Client::<HttpConnector>::new();
        RpcClient { client, url, auth }
    }

    pub async fn get_raw_transaction(&self, txid: &str) -> Result<Transaction, RpcError> {
        let response = self
            .send_json_rpc_request("getrawtransaction", json!([txid, false]))
            .await;
        match response {
            Ok(result) => {
                let result: serde_json::Value = result.result;
                let transaction_hex: String = match serde_json::from_value(result) {
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
                let response_: Vec<String> = match serde_json::from_value(result.result) {
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
            id: 1,
        };

        let request_body = match serde_json::to_string(&request) {
            Ok(body) => body,
            Err(e) => return Err(RpcError::Deserialization(e.to_string())),
        };

        let req = Request::builder()
            .method("POST")
            .uri(self.url)
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
        dbg!(&req);

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
                RpcError::JsonRpc(JsonRpcError {
                    code: -1,
                    message: format!("Deserialization error {:?}", e),
                })
            })
        } else {
            match serde_json::from_slice(&body) {
                Ok(error_response) => Err(error_response),
                Err(e) => Err(RpcError::JsonRpc(JsonRpcError {
                    code: -1,
                    message: format!("Deserialization error {:?}", e),
                })),
            }
        }
    }

    //async fn send_json_rpc_request_<T: for<'de> Deserialize<'de> + std::fmt::Debug>(
    //    &self,
    //    method: &str,
    //    params: serde_json::Value,
    //) -> Result<JsonRpcResult<T>, RpcError> {
    //    let client = reqwest::Client::new();
    //    let (username, password) = self.auth.clone().get_user_pass();
    //    let request = JsonRpcRequest {
    //        jsonrpc: "2.0".to_string(),
    //        method: method.to_string(),
    //        params,
    //        id: 1,
    //    };
    //    let url = self.url;

    //    let response = client
    //        .post(url)
    //        .basic_auth(username, Some(password))
    //        .json(&request)
    //        .send()
    //        .await;
    //    match response {
    //        Ok(response_) => {
    //            if response_.status().is_success() {
    //                response_.json().await.map_err(|e| {
    //                    JsonRpcError {
    //                        code: -1,
    //                        message: format!("Deserialization error {:?}", e),
    //                    }
    //                    .into()
    //                })
    //            } else {
    //                match response_.json().await {
    //                    Ok(error_response) => Err(error_response),
    //                    Err(e) => Err(JsonRpcError {
    //                        code: -1,
    //                        message: format!("Deserialization error {:?}", e),
    //                    }
    //                    .into()),
    //                }
    //            }
    //        }
    //        Err(e) => Err(RpcError::Http(e.to_string())),
    //    }
    //}
}

#[derive(Clone, Debug)]
pub struct Auth<'a> {
    username: &'a str,
    password: &'a str,
}

impl<'a> Auth<'a> {
    pub fn get_user_pass(self) -> (&'a str, &'a str) {
        (self.username, self.password)
    }
    pub fn new(username: &'a str, password: &'a str) -> Auth<'a> {
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
struct JsonRpcResult<T> {
    result: T,
    id: u64,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    code: i32,
    message: String,
}

#[derive(Debug, Deserialize)]
pub enum RpcError {
    JsonRpc(JsonRpcError),
    Deserialization(String),
    Http(String),
}

impl From<JsonRpcError> for RpcError {
    fn from(error: JsonRpcError) -> Self {
        Self::JsonRpc(error)
    }
}
