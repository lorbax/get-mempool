use reqwest;
use serde_json::json;
use serde::{Serialize, Deserialize};
use bitcoin::Transaction;
use bitcoin::consensus::encode::deserialize as consensus_decode;
use hex::decode;

pub async fn get_raw_mempool(url: &str, username: &str, password: &str) -> Result<Vec<String>, RpcError> {
    let response = send_json_rpc_request(
        url,
        username,
        password,
        "getrawmempool",
        json!([]),
    ).await;
    match response {
        Ok(result) => {
            let response_: Vec<String> = serde_json::from_value(result.result).unwrap();
            Ok(response_)
        },
        Err(error) => Err(RpcError::JsonRpcError(error)),
    }
}

pub async fn get_raw_transaction(url: &str, username: &str, password: &str, txid: &str) -> Result<Transaction, RpcError> {
    let response = send_json_rpc_request(
        url,
        username,
        password,
        "getrawtransaction",
        json!([txid, false]),
    ).await;
    match response {
        Ok(result) => {
            let result: serde_json::Value = result.result;
            let transaction_hex: String = serde_json::from_value(result).unwrap();
            let transaction_bytes = decode(transaction_hex).expect("Decoding failed"); 
            let transaction: Transaction = consensus_decode(&transaction_bytes).expect("Deserialization failed");
            Ok(transaction)
        },
        Err(error) => Err(RpcError::JsonRpcError(error)),
    }
}

async fn send_json_rpc_request<T: for<'de> Deserialize<'de> + std::fmt::Debug>(
    url: &str, 
    username: &str,
    password: &str,
    method: &str, 
    params: serde_json::Value,
) -> Result<JsonRpcResult<T>, JsonRpcError> {
    let client = reqwest::Client::new();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: method.to_string(),
        params,
        id: 1,
    };

    let response = client.post(url)
        .basic_auth(username, Some(password))
        .json(&request)
        .send()
        .await;
    match response {
        Ok(response_) => {
            if response_.status().is_success() {
                response_.json().await.map_err(|e| 
                    JsonRpcError { code: -1, message: format!("Deserialization error {:?}", e)}
                )
            } else {
                match response_.json().await {
                    Ok(error_response) => Err(error_response),
                    Err(e) => Err(
                        JsonRpcError { code: -1, message: format!("Deserialization error {:?}", e)}
                    ),
                }
            }
        },
        Err(_) => todo!(),
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
    JsonRpcError(JsonRpcError),
    Other
}

//#[derive(Debug, Deserialize)]
//struct JsonRpcResponse<T> {
//    result: Option<T>,
//    error: Option<JsonRpcError>,
//    id: u64,
//}

// WITHIN THE FUNCTION send_json_rpc_request
    //match response {
    //    Ok(response) => {
    //        //match response.text().await {
    //        //    Ok(text) => {
    //        //        println!("Risposta grezza: {}", text);
    //        //        todo!()
    //        //    },
    //        //    Err(e) => {
    //        //        println!("Errore nella lettura del testo della risposta: {}", e);
    //        //        Err(Box::new(e))
    //        //    },
    //        //}
    //        let json_response = response.json::<JsonRpcResponse<T>>().await;

    //        match json_response {
    //            Ok(res) => {
    //                match res.result {
    //                    Some(result) => Ok(result),
    //                    None => Err(Box::new(std::io::Error::new(
    //                        std::io::ErrorKind::Other,
    //                        format!("JSON-RPC Error: {:?}", res.error),
    //                    ))),
    //                }
    //            },
    //            Err(e) => Err(Box::new(e)),
    //        }
    //    },
    //    Err(e) => Err(Box::new(e)),
    //}

//pub async fn get_raw_mempool_modified(url: &str, username: &str, password: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
//    let client = reqwest::Client::new();
//    let request = JsonRpcRequest {
//        jsonrpc: "2.0".to_string(),
//        method: "getrawmempool".to_string(),
//        params: serde_json::Value::Array(Vec::new()),
//        id: 1,
//    };
//    let response = client.post(url)
//        .basic_auth(username, Some(password))
//        .json(&request)
//        .send()
//        .await;
//    match response {
//        Ok(response) => {
//            let json_response: Result<JsonRpcResponse<Vec<String>>, reqwest::Error> = response.json().await;
//            match json_response {
//                Ok(res) => {
//                    match res.result {
//                        Some(result) => {
//                            //let result_: Vec<String> = serde_json::from_value(result).unwrap();
//                            Ok(result)
//                        },
//                        None => {
//                            return Err(Box::new(std::io::Error::new(
//                            std::io::ErrorKind::Other,
//                            format!("JSON-RPC Error: {:?}", res.error))));
//                        },
//                    }
//                },
//                Err(e) => {
//                    Err(Box::new(e))
//                },
//            }
//        },
//        Err(e) => {
//            Err(Box::new(e))
//        },
//    }
//}
