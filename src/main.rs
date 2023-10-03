//extern crate bitcoincore_rpc;
//use bitcoincore_rpc::{Client, Auth};
////use bitcoincore_rpc::jsonrpc::Result;
//use bitcoincore_rpc::RpcApi;
//use std::error::Error;
//
//fn main() -> Result<(), Box<dyn Error>> {
//    let rpc_url = "http://188.166.104.240:8333".to_string(); // Cambia l'indirizzo e la porta se necessario
//    let client = Client::new(&rpc_url, Auth::None).unwrap();
//
//    // Richiedi i valori della mempool
//    let blockchain_info = client.get_blockchain_info()?;
//    
//    //if mempool.is_empty(){ 
//    //    println!("vuota");
//    //};
//
//    //// Stampa le transazioni nella mempool
//    //for (txid, info) in mempool {
//    //    println!("Transazione ID: {}", txid);
//    //    println!("Informazioni: {:?}", info);
//    //}
//    
//    println!("number of blocks {}", blockchain_info.blocks);
//
//    Ok(())
//}
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! A very simple example used as a self-test of this library against a Bitcoin
//! Core node.
extern crate bitcoincore_rpc;

use bitcoincore_rpc::{Auth, Client, Error, RpcApi};

fn main_result() -> Result<(), Error> {
    //let mut args = std::env::args();

    //let _exe_name = args.next().unwrap();

    //let url = args.next().expect("Usage: <rpc_url> <username> <password>");
    //let user = args.next().expect("no user given");
    //let pass = args.next().expect("no pass given");
    let url = "http://34.125.157.90:8333".to_string(); 

    let rpc = Client::new(&url, Auth::None).unwrap();

    //let _blockchain_info = rpc.get_blockchain_info()?;

    //let best_block_hash = rpc.get_best_block_hash()?;
    //println!("best block hash: {}", best_block_hash);
    //let bestblockcount = rpc.get_block_count()?;
    //println!("best block height: {}", bestblockcount);
    //let best_block_hash_by_height = rpc.get_block_hash(bestblockcount)?;
    //println!("best block hash by height: {}", best_block_hash_by_height);
    //assert_eq!(best_block_hash_by_height, best_block_hash);

    //let bitcoin_block: bitcoin::Block = rpc.get_by_id(&best_block_hash)?;
    //println!("best block hash by `get`: {}", bitcoin_block.header.prev_blockhash);
    //let bitcoin_tx: bitcoin::Transaction = rpc.get_by_id(&bitcoin_block.txdata[0].txid())?;
    //

    let mempool = rpc.get_raw_mempool().unwrap();
    println!("tx by `get`: {}", mempool[0]);

    Ok(())
}

fn main() {
    main_result().unwrap();
}
