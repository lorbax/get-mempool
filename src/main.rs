extern crate bitcoincore_rpc; 
use bitcoincore_rpc::{Auth, Client, RpcApi}; 
use bitcoin::Transaction;

mod mempool;

#[tokio::main]
async fn main() {
    let url = "http://127.0.0.1:18332".to_string();
    let username = "username".to_string(); 
    let password = "password".to_string(); 
    let auth = Auth::UserPass(username.clone(), password.clone());

    let rpc = Client::new(&url, auth).unwrap();

    let mempool = rpc.get_raw_mempool().unwrap();
    let result = mempool::minimal_rpc::get_raw_mempool(&url, &username, &password).await; 
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
            println!("Mempool with manual request: {:?}", result_);
            let txid: &str = &result_[0];
            let transaction: Transaction = mempool::minimal_rpc::get_raw_transaction(&url, &username, &password, txid).await.unwrap();
            let txid: bitcoin::Txid = transaction.txid();
            println!("First Transaction: {:?}", transaction);
            println!("Id of first Transaction: {:?}", txid); 
        },
        Err(e) => println!("Error: {:?}", e),
    }
}

