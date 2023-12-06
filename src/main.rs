extern crate bitcoincore_rpc; 
use bitcoincore_rpc::{Auth, Client, RpcApi}; 
use bitcoin::Transaction;
use mempool::minimal_rpc::{RpcClient, Auth as MiniAuth};

mod mempool;

#[tokio::main]
async fn main() {
    let url = "http://127.0.0.1:18332";
    let username = "username".to_string(); 
    let password = "password".to_string(); 
    let auth = Auth::UserPass(username.clone(), password.clone());

    let rpc = Client::new(&url, auth).unwrap();

    let mempool = rpc.get_raw_mempool().unwrap();
    let auth = MiniAuth::new(username.clone(), password.clone());
    let mini_rpc = RpcClient::new(url, auth); 
    let result = mini_rpc.clone().get_raw_mempool().await; 
    //let result_ = mempool::minimal_rpc::get_raw_memgTol_modified(&url, &username, &password).await; 
    
    if mempool.is_empty() {
        println!("Empty mempool");
    } else {
        println!("Transactions in mempool:");
        for txid in mempool {
            println!("{}", txid);
        }
    };
    match result {
        Ok(result_) => {
            let first_transaction: Transaction = mini_rpc.get_raw_transaction(&result_[0]).await.unwrap(); 
            let first_txid: bitcoin::Txid = first_transaction.txid();
            println!("Mempool with manual request: {:?}", result_);
            println!("First Transaction: {:?}", first_transaction);
            println!("Id of first Transaction: {:?}", first_txid); 
        },
        Err(e) => println!("Error: {:?}", e),
    }
}

