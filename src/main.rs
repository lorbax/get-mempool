extern crate bitcoincore_rpc;

use bitcoincore_rpc::{Auth, Client, Error, RpcApi};

fn main_result() -> Result<(), Error> {
    let url = "http://127.0.0.1:18332".to_string();
    let username = "username".to_string(); 
    let password = "password".to_string(); 
    let auth = Auth::UserPass(username, password);

    let rpc = Client::new(&url, auth)?;

    let mempool = rpc.get_raw_mempool()?;
    
    if mempool.is_empty() {
        println!("La mempool è vuota.");
    } else {
        println!("Transazioni nella mempool:");
        for txid in mempool {
            println!("{}", txid);
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("Errore: {:?}", e);
    }
}
