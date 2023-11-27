extern crate bitcoincore_rpc;

mod minimal_rpc;

use bitcoincore_rpc::{Auth, Client, Error, RpcApi};

async fn main_result() -> Result<(), Error> {
    let url = "http://127.0.0.1:18332".to_string();
    let username = "username".to_string(); 
    let password = "password".to_string(); 
    let auth = Auth::UserPass(username.clone(), password.clone());

    let rpc = Client::new(&url, auth)?;

    let mempool = rpc.get_raw_mempool()?;
    let result = minimal_rpc::get_raw_mempool_(&url, &username, &password).await; 
    
    if mempool.is_empty() {
        println!("La mempool è vuota.");
    } else {
        println!("Transazioni nella mempool:");
        for txid in mempool {
            println!("{}", txid);
        }
    };
    println!("Transazioni richiesta manuale: {:?}", result.unwrap());

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = main_result().await {
        eprintln!("Errore: {:?}", e);
    }
}
