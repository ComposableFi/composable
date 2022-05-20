Official documentation: https://docs.boba.network/developer-docs/bridging-l1-l2

Boba Network is an Optimistic Rollup that combines the great open source work done by Optimism with the research and development effort of the Enya & Boba team on swap-based onramp, fast exit and cross-chain bridging. We chose to build on Optimism because it is essentially a modified version of Ethereum, which makes it relatively easy to ensure EVM and Solidity compatibility, minimizing the efforts required to migrate smart contracts from L1 to L2.
Boba is maintained by the OMG and Enya teams.

```
// Pretend this is on L2
contract MyOMGXContract {
    doSomething() public {
        // ... some sort of code goes here
    }
}

// And pretend this is on L1
contract MyOtherContract {
    function doTheThing(address myOMGXContractAddress, uint256 myFunctionParam) public {
        ovmL1CrossDomainMessenger.sendMessage(
            myOMGXContractAddress,
            abi.encodeWithSignature(
                "doSomething(uint256)",
                myFunctionParam
            ),
            1000000 // use whatever gas limit you want
        )
    }
}
```
