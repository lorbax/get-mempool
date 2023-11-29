extern crate bitcoincore_rpc;

mod lib;

use lib::minimal_rpc;

use bitcoincore_rpc::{Auth, Client, Error, RpcApi};
use hex::decode;
use bitcoin::Transaction;
use bitcoin::consensus::encode::deserialize as consensus_decode;


async fn main_result() -> Result<(), Error> {
    let url = "http://127.0.0.1:18332".to_string();
    let username = "username".to_string(); 
    let password = "password".to_string(); 
    let auth = Auth::UserPass(username.clone(), password.clone());

    let rpc = Client::new(&url, auth)?;

    let mempool = rpc.get_raw_mempool()?;
    let result = minimal_rpc::get_raw_mempool_(&url, &username, &password).await; 
    let result = result.unwrap();
    let result_ = minimal_rpc::get_raw_mempool_modified(&url, &username, &password).await; 
    let txid: &str = &result[0];
    let transaction_hex = minimal_rpc::get_raw_transaction(&url, &username, &password, txid).await;
    let transaction_bytes = decode(transaction_hex.unwrap()).expect("Decoding failed"); 
    let transaction: Transaction = consensus_decode(&transaction_bytes).expect("Deserialization failed");
    let txid: bitcoin::Txid = transaction.txid();
    
    if mempool.is_empty() {
        println!("La mempool è vuota.");
    } else {
        println!("Transazioni nella mempool:");
        for txid in mempool {
            println!("{}", txid);
        }
    };
    println!("Transazioni richiesta manuale: {:?}", result);
    println!("Transazioni richiesta manuale modificata: {:?}", result_.unwrap());
    println!("Transazione specifica: {:?}", transaction);
    println!("Transazione specifica: {:?}", txid); 

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = main_result().await {
        eprintln!("Errore: {:?}", e);
    }
}
