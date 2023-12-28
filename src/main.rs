use base64::Engine;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::Request;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioIo;
use hyper_util::rt::TokioExecutor;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpStream;
use hyper_util::client::legacy::Client;

#[tokio::main]
async fn main() {
    let response = send_json_rpc_request("getrawmempool", json!([])).await;
    match response {
        Ok(result) => {
            let response_: Result<Vec<String>, ()> =
                match serde_json::from_value(result.result.unwrap()) {
                    Ok(response_inner) => response_inner,
                    Err(_) => {
                        println!("Deserialization error");
                        Err(())
                    }
                };
            println!("The mempool is: {:?}", response_.unwrap());
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

async fn send_json_rpc_request<T: for<'de> Deserialize<'de> + std::fmt::Debug>(
    method: &str,
    params: serde_json::Value,
) -> Result<JsonRpcResult<T>, JsonRpcError> {
    let url = "http://127.0.0.1:18332".parse::<hyper::Uri>().unwrap();
    //let host = url.host().expect("uri has no host");
    //let port = url.port_u16().unwrap_or(18332);
    //let address = format!("{}:{}", host, port);
    //let stream = TcpStream::connect(address).await.unwrap();
    //let io = TokioIo::new(stream);
    //let (mut sender, _conn) = hyper::client::conn::http1::handshake(io).await.unwrap();
    let (username, password) = ("username", "password");
    //let client: Client<HttpConnector, Full<>
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
        // try also from(Bytes), with request_body = match serde_json::to_vec(&request)
        .body(Full::<Bytes>::from(request_body))
        .unwrap();
    dbg!(&req);

    println!("A");
    let response = client.request(req).await.unwrap();

    println!("B");
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    //let frame = body.collect();
    //let chunck = frame.
    //let body = hyper::body::to_bytes(body)
    //    .await
    //    .map_err(|e| RpcError::Http(e.to_string()))?;

    if status.is_success() {
        serde_json::from_slice(&body).unwrap()
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
