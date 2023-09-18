# Internals

This document documents internals of CVM on CW.

1. CVM on CW Cosmos to Cosmos uses ICS20 for value transfers. Each ICS20 channel must be upserted into CVM config.
2. CVM does not have any hardcoded requirement for bridge to be trustless or trustful. 
3. CVM uses ICS-20 assets transfers on Cosmos chains. On Polkadot and Ethereum it uses escrow/mint, path dependant semantics compatible with ICS-20.
