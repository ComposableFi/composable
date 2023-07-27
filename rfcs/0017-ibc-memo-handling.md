# Overview

Describes how ICS20 memo is used in IBC ecosystem to do arbitrary anonymous execution of cross chain code with multihop.  

With lack of packets batching and official multihop IBC transactions, memo of ICS20 is the only option to do extended fuctional in after singe user wallet signature.

Resonable batching and custom multihop would rely on same IBC host infrastucture to be presented.

Any discrepancy from de facto standard implementations in Cosmos would need to be handled as special cases.

## IBC protocol

### Memo

Is arbitrary data, extended by any chain as it wants. 
Middlware must not consider that they will be able to parse whole memo,
so it is resonable to parse part of message and propagate remains as it is.
So middleware can handle memo, preventin other middlewares from handling it to at same time.

### Transactions and rollbacks
According existing stanard IBC implementations, 
application module and its middlewares either fully execute or fully rollback.

In case of middleware failure, changed made by module and other middlewares are rolled back.
Packet callbacks with error ACK to sender chain.

Specifially if middleware handles memo and fails with error

### Async asckowledgmets

Receiver chain is not oblidges to acskolwedge packet upon receive. 
It can do multi block and multi chain execution of packet,
and askowledge or timeout packet later. 

That is basis for forwarding packets for multi hop execution and automatic rollacks. 

## Error handling

Given above protocol features, 
user can rely on trasactionality of their packets executuion and automatic rollbacks.

If hosts do not conform in various way, special handling should be arranged.

### Terminations

`memo` may contain invocation of contract, like CosmWasm or Parity XCM.

In this case immediate processing of local transaction can error, 
and make error handling just to be normal part of host execution (transaction/rollback).

Also it can use async askoledgment feature to propagate messages futher.

This way most automatic and cheapset way of execution happens, 
and the only option to reach chains which cannot be extended with arbitrary contracts.

`memo` can be used to organize several packets into one transactions.
Well defiend and consisten rules about transactionality of single packets,
allow to define common consisten rules of assembling packets into high level flow
(multi chain program), and resonably cancel out program in case of failure of some of its parts.

### XCM integration

[XCM-IBC](../0016-permissionless-assets-for-ibc-and-cw.md) can stably operation iff `memo` handling is part of packet transaction
so party sending message can rely on multi hop IBC.

Also in case of success or fully fail, XCM can rely on fact that if XCM message was stored into output queue and tracked,
both ICS20 receive and memo handling succeded, and XCM can resonably sends funds foward from chain origin.

If Substrate does not try to parse all memo parts is not up to, but only relevant for processing part, 
it will not fail when some protocols intorduce new parts, like batching.

