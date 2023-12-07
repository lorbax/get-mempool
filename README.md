# CUSTOM MEMPOOL FOR JD SERVER OF STRATUM V2

In the SRI (Stratum V2 Reference Implementation), the implementation of the mempool for the JDS uses a vendorized version of the json-rpc.
This crate aims to remove vendorized software, and uses a custom version of json-rpc.
Things to do:
 - implement submit block logic
 - testing
 - include it in the SRI
 - testing again:
