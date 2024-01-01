extern crate bitcoincore_rpc;
use bitcoincore_rpc::{Auth, Client, RpcApi};

mod mempool;

#[tokio::main]
async fn main() {
    let url = "http://127.0.0.1:18332";
    let username = "username".to_string();
    let password = "password".to_string();
    let auth = Auth::UserPass(username.clone(), password.clone());

    let rpc = Client::new(url, auth).unwrap();

    let mempool = rpc.get_raw_mempool().unwrap();
    let mut mempool_ = mempool::JDsMempool::new(url.to_string(), username, password);

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
    match mempool_.update_mempool().await {
        Ok(_) => {
            let first_transaction = &mempool_.mempool[0];
            println!("First transaction with hash: {:?}", first_transaction);
        }
        Err(error) => println!("Error: {:?}", error),
    };
}
