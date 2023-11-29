use reqwest;
use serde_json::json;
use serde::{Serialize, Deserialize};
use bitcoin::Transaction;

pub async fn get_raw_mempool_(url: &str, username: &str, password: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response: serde_json::Value = send_json_rpc_request(
        url,
        username,
        password,
        "getrawmempool",
        json!([]),
    ).await?;
    let response_: Vec<String> = serde_json::from_value(response).unwrap();
    Ok(response_)
}
pub async fn get_raw_mempool_modified(url: &str, username: &str, password: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "getrawmempool".to_string(),
        params: serde_json::Value::Array(Vec::new()),
        id: 1,
    };
    println!("RICHIESTA MODIFICATA");
    dbg!(&request);
    let response = client.post(url)
        .basic_auth(username, Some(password))
        .json(&request)
        .send()
        .await;
    match response {
        Ok(response) => {
            let json_response: Result<JsonRpcResponse<Vec<String>>, reqwest::Error> = response.json().await;
            match json_response {
                Ok(res) => {
                    match res.result {
                        Some(result) => {
                            println!("BBBBBB");
                            //let result_: Vec<String> = serde_json::from_value(result).unwrap();
                            Ok(result)
                        },
                        None => {
                            println!("CCCCCC");
                            return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("JSON-RPC Error: {:?}", res.error))));
                        },
                    }
                },
                Err(e) => {
                    Err(Box::new(e))
                },
            }
        },
        Err(e) => {
            Err(Box::new(e))
        },
    }
}

pub async fn get_raw_transaction(url: &str, username: &str, password: &str, txid: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response: serde_json::Value = send_json_rpc_request(
        url,
        username,
        password,
        "getrawtransaction",
        json!([txid, false]),
    ).await?;
    let response_: String = serde_json::from_value(response).unwrap();
    Ok(response_)
}

async fn send_json_rpc_request<T: for<'de> Deserialize<'de> + std::fmt::Debug>(
    url: &str, 
    username: &str,
    password: &str,
    method: &str, 
    params: serde_json::Value,
) -> Result<T, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: method.to_string(),
        params,
        id: 1,
    };
    println!("RICHIESTA");
    dbg!(&request);

    let response = client.post(url)
        .basic_auth(username, Some(password))
        .json(&request)
        .send()
        .await;
    match response {
        Ok(response) => {
            //match response.text().await {
            //    Ok(text) => {
            //        println!("Risposta grezza: {}", text);
            //        todo!()
            //    },
            //    Err(e) => {
            //        println!("Errore nella lettura del testo della risposta: {}", e);
            //        Err(Box::new(e))
            //    },
            //}
            let json_response = response.json::<JsonRpcResponse<T>>().await;

            match json_response {
                Ok(res) => {
                    match res.result {
                        Some(result) => Ok(result),
                        None => Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("JSON-RPC Error: {:?}", res.error),
                        ))),
                    }
                },
                Err(e) => Err(Box::new(e)),
            }
        },
        Err(e) => Err(Box::new(e)),
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
struct JsonRpcResponse<T> {
    result: Option<T>,
    error: Option<JsonRpcError>,
    id: u64,
}

#[derive(Debug, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}
