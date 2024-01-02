pub mod rpc_client;

use bitcoin::Transaction;
use bitcoin::Txid;
use rpc_client::{Auth, RpcClient, RpcError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash([u8; 32]);

#[derive(Clone, Deserialize)]
pub struct Amount(f64);

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockHash(Hash);

#[derive(Clone, Debug)]
pub struct TransacrtionWithHash {
    id: Txid,
    tx: Transaction,
}

#[derive(Clone, Debug)]
pub struct JDsMempool {
    pub mempool: Vec<TransacrtionWithHash>,
    auth: Auth,
    url: String,
}

impl JDsMempool {
    pub fn get_client(&self) -> Option<RpcClient> {
        let url = self.url.clone();
        if url.contains("http") {
            Some(RpcClient::new(url, self.auth.clone()))
        } else {
            None
        }
    }

    pub fn new(url: &str, username: String, password: String) -> Self {
        let auth = Auth::new(username, password);
        let empty_mempool: Vec<TransacrtionWithHash> = Vec::new();
        JDsMempool {
            mempool: empty_mempool,
            auth,
            url: url.to_string(),
        }
    }

    pub async fn update_mempool(&mut self) -> Result<(), JdsMempoolError> {
        let mut mempool_ordered: Vec<TransacrtionWithHash> = Vec::new();
        let client = JDsMempool::get_client(self).ok_or(JdsMempoolError::NoClient)?;
        let new_mempool: Result<Vec<TransacrtionWithHash>, JdsMempoolError> = {
            let mempool: Vec<String> = match client.get_raw_mempool_verbose().await {
                Ok(mempool_inner) => mempool_inner,
                Err(e) => return Err(e.into()),
            };
            for id in &mempool {
                let tx: Result<Transaction, _> = client.get_raw_transaction(id, None).await;
                if let Ok(tx) = tx {
                    let id = tx.txid();
                    mempool_ordered.push(TransacrtionWithHash { id, tx });
                }
            }
            if mempool_ordered.is_empty() {
                Err(JdsMempoolError::EmptyMempool)
            } else {
                Ok(mempool_ordered)
            }
        };

        match new_mempool {
            Ok(new_mempool_) => {
                self.mempool = new_mempool_;
                Ok(())
            }
            Err(a) => Err(a),
        }
    }
}
#[derive(Debug)]
pub enum JdsMempoolError {
    EmptyMempool,
    RpcError(RpcError),
    NoClient,
}

impl From<RpcError> for JdsMempoolError {
    fn from(error: RpcError) -> Self {
        Self::RpcError(error)
    }
}
