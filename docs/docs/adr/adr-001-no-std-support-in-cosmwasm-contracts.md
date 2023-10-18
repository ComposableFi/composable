# Problem

CosmWasm is far for being no_std. It requires to compile into wasm with std enabled.

Polkadot fails to compile in std into wasm.

In order to share code we forked CosmWasm and made it no_std.

And shared code.

I wrote new CosmWasm contract on std version, but it fails to compile using our shared code.

I did it because it is much easy to write contract using `sylvia` high level framework (like `Anchor`` in Solana).

Also I used BigNumber, which is std only, so simplify code writing and audit.

So how I can reuse code and yet write non shared contract at same time?

Also we cannot switch fully to std as will support Solana.

# Solution

There are 2 approaches which will be used at same time.

First, application, no shared code contracts, will use schema for generated contract and structures, so there is no compilation dependency. 
Same schema as TypeScript developers use.

Second, I will produce `cvm-primitives` crate. That crate is super low level, compiles in all combination.

Specifically wasm in std and wasm in not std.

Why it will work?

Applications of CVM must never depend to Polkadot code directly which fails wasm std.

And `primitives` will not too.

So we will have process of `no-std/std`-fication into primitives as time goes and pick bests tool with lowest maintain. 

