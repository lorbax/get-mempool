# CUSTOM MEMPOOL FOR JD SERVER OF SRI

In the SRI (Stratum V2 Reference Implementation), the implementation of the mempool for the JDS uses a vendorized version of the json-rpc.
This crate aims to remove vendorized software, and uses a custom version of json-rpc.
Things to do:
 - implement submit block logic
 - testing
 - include it in the SRI
 - testing again:

How this demo repo works:
1. get the mempool using the conventional way (bitcoincore-rpc)
2. get the mempool using custom code
3. compare that the first transaction of the mempool obtained in the custom way is the same as the
   first transaction of the mempool obtained with bitcoin json-rpc

For doing so, you must have a tested bitcoin node running with with config file

    [test]
    testnet=1
    server=1
    datadir= [path to your testnet blockchain]
    rpcuser=username
    rpcpassword=password
    rpcport=18332

