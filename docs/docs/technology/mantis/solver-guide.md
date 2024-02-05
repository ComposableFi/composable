# Solver Guide
This tutorial describes how to run a solver node and how users can post problems to this.

### Deployments

| **Chain** | **Stage** | **ID** |
| -------- | -------- | -------- |
| centauri-1     | mainnet     | centauri1lnyecncq9akyk8nk0qlppgrq6yxktr68483ahryn457x9ap4ty2sthjcyt    |
| osmosis-1     | mainnet     |      |


### Problem Submission
An example of a problem that the user can post is here:
```js
          {
            "@type": "/cosmwasm.wasm.v1.MsgExecuteContract",
            "sender": "centauri1mgnu00vn0feumu660y6p7ty5mv58txvhgkr2lu",
            "contract": "centauri1lnyecncq9akyk8nk0qlppgrq6yxktr68483ahryn457x9ap4ty2sthjcyt",
            "msg": {
              "order": {
                "msg": {
                  "wants": {
                    "denom": "ppica",
                    "amount": "1000000"
                  },
                  "transfer": null,
                  "timeout": 2506928,
                  "min_fill": null
                }
              }
            },
            "funds": [
              {
                "denom": "ibc/EF48E6B1A1A19F47ECAEA62F5670C37C0580E86A9E88498B7E393EB6F49F33C0",
                "amount": "1"
              }
            ]
          }
```

An example of a mainnet transaction is available [here](https://ping.pub/composable/tx/CA9489EC961BA97AB514A74EEC6BF3B6CD9900C00A031AA3BB80DC343CE85F2D).

You can try it using live code with [this](https://github.com/ComposableFi/composable/blob/main/code/cvm/mantis.ts).

### Deploying Your Own Contracts
Here is how you can deploy your own contracts:
```
$BINARY tx wasm store $ORDER_WASM_FILE --from dz --gas=auto
$BINARY tx wasm instantiate 18 '{"admin": "centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k", "cvm_address" : "centauri1wpf2szs4uazej8pe7g8vlck34u24cvxx7ys0esfq6tuw8yxygzuqpjsn0d"}' --label "mantis_order_1" --admin centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k --gas=auto --from=dz
```
### Running a Solver Node
Documentation on how to run a solver node is located [here](https://github.com/ComposableFi/composable/blob/06b2b265a4fb0e866faaf76af4ab94ba580560dd/docs/docs/technology/mantis/solver-tutorial.md#L4). 

### Problem/Solution Format
See on-chain indexer/explorer for the Problem Solver contract. It will show all in JSON format.

### Querying the Order Book
How to query the order book on MANTIS is shown below:
- https://order-book.composablenodes.tech/mantis/orders
- https://order-book.composablenodes.tech/mantis/orders?orderId=ORDER_ID 

### CVM Interface
An example of how to convert the solution into a CVM program algorithm is depicted [here](https://github.com/ComposableFi/composable/blob/main/tests/examples/cvm.ts).

### Fees
There are no fees; users just need to pay gas. 

### Contracts
The full MANTIS contracts GitHub repo is [here](https://github.com/ComposableFi/cvm/tree/main/contracts/cosmwasm/order).

### Contract to add the Problem to MANTIS
The MANTIS contract to add the problem can be viewed [here](https://github.com/ComposableFi/composable/blob/main/code/cvm/mantis.ts).
