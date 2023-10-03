extern crate bitcoincore_rpc;

use bitcoincore_rpc::{Auth, Client, Error, RpcApi};

fn main_result() -> Result<(), Error> {
    let url = "http://34.125.157.90:8333".to_string();
    let rpc = Client::new(&url, Auth::None).unwrap();

    let mempool = rpc.get_raw_mempool().unwrap();
    println!("tx by `get`: {}", mempool[0]);

    Ok(())
}

fn main() {
    main_result().unwrap();
}
