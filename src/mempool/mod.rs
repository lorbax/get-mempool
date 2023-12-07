pub mod minimal_rpc;

use bitcoin::Transaction;
use bitcoin::Txid;
use minimal_rpc::{Auth, RpcClient, RpcError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash([u8; 32]);

#[derive(Clone, Deserialize)]
pub struct Amount(f64);

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockHash(Hash);

#[derive(Clone, Debug)]
pub struct TransactionWithHash {
    id: Txid,
    tx: Transaction,
}

#[derive(Clone, Debug)]
pub struct JDsMempool<'a> {
    pub mempool: Vec<TransactionWithHash>,
    auth: Auth<'a>,
    url: &'a str,
}

impl<'a> JDsMempool<'a> {
    pub fn get_client(&self) -> RpcClient<'a> {
        let url = self.url;
        RpcClient::new(url, self.auth.clone())
    }

    pub fn new(url: &'a str, username: &'a str, password: &'a str) -> Self {
        let auth = Auth::new(username, password);
        let empty_mempool: Vec<TransactionWithHash> = Vec::new();
        JDsMempool {
            mempool: empty_mempool,
            auth,
            url,
        }
    }

    pub async fn update_mempool(&mut self) -> Result<(), JdsMempoolError> {
        let mut mempool_ordered: Vec<TransactionWithHash> = Vec::new();
        let client = JDsMempool::get_client(self);
        let new_mempool: Result<Vec<TransactionWithHash>, JdsMempoolError> = {
            let mempool: Vec<String> = match client.get_raw_mempool().await {
                Ok(mempool_inner) => mempool_inner,
                Err(e) => return Err(e.into()),
            };
            for id in &mempool {
                let tx: Result<Transaction, _> = client.get_raw_transaction(id).await;
                if let Ok(tx) = tx {
                    let id = tx.txid();
                    mempool_ordered.push(TransactionWithHash { id, tx });
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
}

impl From<RpcError> for JdsMempoolError {
    fn from(error: RpcError) -> Self {
        Self::RpcError(error)
    }
}
