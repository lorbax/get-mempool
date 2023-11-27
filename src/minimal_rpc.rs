use reqwest;
use serde_json::json;
use serde::{Serialize, Deserialize};

pub async fn get_raw_mempool_(url: &str, user: &str, password: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("AAAAAAAA");
    let response: serde_json::Value = send_json_rpc_request(
        url,
        user,
        password,
        "getrawmempool",
        json!([]),
    ).await?;
    let response_: Vec<String> = serde_json::from_value(response).unwrap();
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
                    println!("CCCCCC");

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
