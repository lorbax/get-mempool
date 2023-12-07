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
    let mut mempool_ = mempool::JDsMempool::new(url, &username, &password);
    let _ = mempool_.update_mempool().await;
    //let result_ = mempool::minimal_rpc::get_raw_memgTol_modified(&url, &username, &password).await; 
    
    println!("Now we pull the mempool using the bitcoinrpc crate");
    if mempool.is_empty() {
        println!("Empty mempool");
    } else {
        println!("Transactions in mempool:");
        for txid in mempool {
            println!("{}", txid);
        }
    };
    println!("The first transaction of the mempool used custom software. Compare this transaction with the first transaction obtained with bitcoinrpc crate");
    let first_transaction = &mempool_.mempool[0]; 
    println!("First transaction with hash: {:?}", first_transaction);
    //match mempool_ {
    //    Ok(result_) => {
    //        let first_transaction: Transaction = mini_rpc.get_raw_transaction(&result_[0]).await.unwrap(); 
    //        let first_txid: bitcoin::Txid = first_transaction.txid();
    //        println!("Mempool with manual request: {:?}", result_);
    //        println!("First Transaction: {:?}", first_transaction);
    //        println!("Id of first Transaction: {:?}", first_txid); 
    //    },
    //    Err(e) => println!("Error: {:?}", e),
    //}
}

