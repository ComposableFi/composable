# What is the IBC Protocol?

## Issues in cross-chain Infrastructure 

As new ecosystems rose in prominence during the 2020-2021 market cycle, we saw many bridging protocols rise and fall to accomodate the need for liquidity. Most (if not all) of these bridges were based on optimistic, fraud-sensitive architectures, where in essence relies on trusted third parties, oracles and multi-sigs. Besides asset transfers, these bridges sometimes support message passing as well, which can be used as a building block for cross-chain applications. However, these bridges have already proven to be security risks to DeFi, with over 2+ billion in bridgings hacks and also, difficult to build protocols on top off due to the lack of complex features provided by the message passing.

In a trusted bridging setup, we identify the following actors: 

- Relayer: pays for execution on the destination chain.
- Oracle: provides authorized mint and burn access to the contracts on origin and destination side.
- User: a contract, protocol or actual user directly interacting with the bridge. 

In this generic architecture, we choose to keep the relayer and oracle as separate actors, although in many actual implementations, these are the same entity. 

Designs used in Wormhole, Axelar and centralized exchanges use one or more accounts determine the state of the origin chain, destination chain, and based on that co-sign a message to mint/unlock funds on the destination side. 

### Trusted Bridging Recapped

We will briefly recapture the architectures of pessimistic and optimistic bridges. 

### Pessimistic bridging 

Pessimistic bridges require the oracle to pre-validate withdrawals, assuming that relayers will commit fraud as soon as they can.

The oracle assumes multiple responsibilities here:

1. It ensures that the event data is correct.
2. It ensures that the event data is final.

For many chains, including Ethereum, 2. cannot yet be ensured, and thus the oracle service is taking on a large risk. Usually this is reduced by waiting for a number of confirmations (blocks built on top of the current block).

From the on-chain perspective, funds are managed assuming that the oracle is honest about 1. and 2. Fraud, hacks or misconfigurations can lead to the oracle's authority being used to incorrectly release funds, as occured in Wormhole, Nomad, Raku etc...

Different protocols attempt to reduce this risk by sharding the private key using multi party computation, or simply multisig.

For a secure bridging operation, the transaction time $t$ is given by:

$$ t := t_{finality} + t_{submission} $$

where $ t_{finality} $ is the average duration for block finality, and $ t_{submission} $ the length of time of block inclusion on the destination side.

### Optimistic bridging

Optimistic bridges such as Nomad assume that the relayer is usually honest, and fraud rarely occurs. The relayer/oracle algorithm is relatively identical to the algorithm. On the contract side however, the mint/unlock action is delayed, ensuring that fishermen have sufficient time to dispute the message.

Message acceptance is incredibly simple:

\begin{algorithm}[H]
\SetAlgoLined
\BlankLine
\If{message is from relayer}{
    store(message)
}
\caption{Message acceptance protocol for optimistic trusted bridges}
\end{algorithm} 

However, actually enacting the message, which leads to mints and unlocks, has a time delay, referred to as the dispute window.

\begin{algorithm}[H]
\SetAlgoLined
\BlankLine
\If{message received time is above wait threshold}{
  If{message is undisputed} {
    enact(message)
  }
}
\caption{Unlock protocol for optimistic trusted bridges}
\end{algorithm} 

Optimistic bridging trades in some advantages of pessimistic bridging, such as faster transaction times, in favour for more decentralized security. Dispute setllement remains an issue however. Usually protocols resolve to token based governance or a council.

For a secure bridging operation, the transaction time $t$ is given by:

$$ t := t_{finality} + t_{submission} + t_{dispute window} $$

where $t_{finality}$ is the average duration for block finality, $t_{submission}$ the length of time of block inclusion on the destination side, and $t_{dispute window}$ the amount of time that the relayed transaction may be disputed on the destination side.

Relayers can choose to combine $t_{finality}$ and $t_ {dispute window}$, at the risk of needing to dispute their own submissions. This improves UX but brings in a significant risk to the relayer, and in practise is not performed.

## Economic Risks in Trusted Bridging {#sec:economics-trusted-bridging}

Bridging in general brings in security risks, both on a technical level as mentioned above, and on an economic level. A wrapped token in essence is a debt obligation between the bridging protocol and the token holder, guaranteeing that on redemption of the wrapped token, the original token will be returned. The value of the wrapped token is thus the value of the underlying minus the risk of the bridging protocol not fulfilling the debt. Currently the market does not correctly price wrapped tokens (usually the valuation is equal to the underlying). This leads to a greater economic impact when a trusted bridge is unable to fulfill the debt, such as after losing the underlying in a hack.

For protocols relying on trusted bridging ecosystem, the only way to deal with this underlying economic risk is to price wrapped tokens differently. Although this mitigates economic impact, liquidity and UX suffer. It also reduces the utility of cross-chain protocols, as the actual cost of using a trusted bridge is the fee and wrapped token premium.