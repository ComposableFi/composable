# Solvers & Solutions

## Intent Submission: Problems
Users will be able to submit their intention along the MANTIS FE/UI/UX to form a problem from their intention. This information is used to establish the problem that will be sent to solvers. The user intention is formatted as:

*I want to [function] [asset 1]...[asset n] for [asset 2]...[asset m].*

Beginning and destination chain/location(s) may also be specified by the user.  The user can further set limits for buy and sell orders.
Thus, fulfilling the submitted user intent requires solving:
- *[Token ID][Chain ID][Amount]1…n assets*
- Token ID and Chain ID are variable
- This is resolved to *Token ID Chain ID amount 1...n assets*

The output that must be solved for is therefore:
- *Asset [1…n]*
- *Asset [2…m]*

:::tip MANTIS Solvers 
The solver role is critical to MANTIS and the chain-agnostic execution of user intents. To summarize the role, a solver takes in data about users’ intents, comes up with a solution to fulfill these as transactions, and is incentivized to do so.
If you are interested in solving for MANTIS or simply want to learn more about how MANTIS solvers work, refer to the following:
- [Discovering MANTIS Orders](https://github.com/ComposableFi/composable/blob/main/docs/docs/technology/mantis/tutorial.md)
- [Solving for MANTIS](https://github.com/ComposableFi/composable/blob/main/docs/docs/technology/mantis/solver-tutorial.md)
- [Solver Integration](https://github.com/ComposableFi/composable/blob/main/docs/docs/technology/mantis/problem-solver-flow.md)
:::

## Discovering MANTIS orders

In an interface on MANTIS, an intent/problem is sent via a smart contract of the Composable Virtual Machine to solvers. Thus, solvers are able to compete to come up with the best solution.

The information that is provided to solvers is limited to just the user account and the user problem.

The format of the problem that is sent from the problem smart contract to solvers is:

```ts
const give = "1100000000"
const wants = "1000000000"

print("one side of want")
const ppica = Coin.fromPartial({ denom: "ppica", amount: give })
print(await client.order({
    msg: {
        timeout: 100,
        wants: {
            denom: "pdemo",
            amount: wants,
        },
    }
},
    "auto",
    null,
    [ppica]
))
```

- Available automated market makers and pools: Osmosis Pool Manager
- Preloaded tokens/incentives (described in more detail later)
- Any specified user limits on the order (i.e. maximum price)

Once solvers receive the user problem, they are able to query the escrow contract to determine sufficient user funds.

Within this interface, solvers are able to access the information they need to create and submit their solutions.


## Solving for MANTIS
Solvers implement a solution algorithm (which can be modified from Composable’s solution algorithm, which will be provided) that determines an optimal solution for each user intention. The user intention is framed as “*I want to [function] [asset 1]...[asset n] for [asset 2]...[asset m1]*”. Beginning and destination chain/location(s) may also be specified by the user. The user will also set limits for buy and sell orders. Once the user submits their transaction intent, their funds are moved into their virtual wallet in the Composable ecosystem.

Then, the solver is able to address the user intent with the aim of optimizing cross-chain intent settlement. This involves the solver solving a path between:
- State 1 = [Token ID][Chain ID][Amount]1…n assets
- State 2= Token ID and Chain ID are variable

The solution can be formed out of a combination of any of the following settlement/execution pathways:
- **Coincidence of Wants (CoWs)**
  - Intents can be fully or partially matched with other intents so that they effectively are used to settle each other’s transaction. This is done along the principle of CoWs: that user intents can coincidentally be the opposite of other user intents (i.e. one intent to swap A for B and another to swap B for A form a CoW).
- **Constant Function Market Makers (CFMMs)**
  - Solutions can be executed on CFMMs such as automated market makers (AMMs).
- **Market makers’ own liquidity**
  - The solver will further play a market maker role. Similar to Hashflow, solvers will settle funds themselves by sending funds to a contract and settling funds directly with the user.

Solvers will submit their proposed solution via an interface on MANTIS to the problem smart contract. The format of the solution is as follows:

- \_Swap(x for y)\_
- Bridge bribes + time to execution
- These calculations determine the net result: 
  - Total amount of assets [2..m] available, total lost to slippage and fees, and total time associated

Solvers’ solutions, once submitted, will be scored for maximization of volume cleared and user welfare (i.e. minimization of slippage). The optimal solution will be selected based on this score. The winning solver (e.g. the solver providing the best solution) is then given access to user funds and approved to execute the user's intention along their specified solution pathway. Winning solvers will receive incentives/rewards, as described in the following section.

### Protocol Bidding
An additional factor that solvers must consider in the auction process of determining the optimal solution is what we term “protocol bidding”. **This is a unique form of incentive wherein we enable protocols to bid for orderflow for transactions being settled from MANTIS.**

Essentially, protocols are able to provide a bid in the form of tokens to the MANTIS framework. The size of this bid is considered by solvers in the algorithm that they use to determine an optimal settlement route for the user transaction intent. 

The bid volume is also taken into account when potential solutions are being scored, with solutions including protocols participating in bidding being ranked higher. As a result, protocols that bid to MANTIS increase their chances that MANTIS intent transactions will be settled using their platforms. The tokens bidded by these protocols will be used to offset user gas fees. 

Protocols would participate in bidding via a 3-step UI process, loading tokens for solvers, and then loading up tokens for gas rebates for users. Then, the relayer (and solvers, if both roles are combined) can take from the contracts post-settlement (once they have proof of fulfillment).

Overall, this would work as follows: 
1. Protocols (such as DEXes) top up the problem smart contract with certain tokens into certain pools
2. Solvers take the above into account in their solution calculations
3. If protocols are successfully chosen as a component of the solution for the solver (and this solver has presented the best solution out of all solvers), then the order flow is routed through the protocol

This requires a problem smart contract within MANTIS, which would be able to receive pre-loaded tokens.
- Our "problem" contract would have `cvm-account` like any users
- All solution will be routed by MANTIS from the problem origin contract so it will have access to `cvm-account`` on each chain
- So, the problem contract will have access to any tokens on that account
- Reasonably, these accounts to have at least native tokens for gas fees and bridge fees
- Or, protocol can have `cvm-account` and do extended allowance to MANTIS sent from problem origin so the solution can peek into allowance and pay fees

This would also require the UI for protocols to initiate and submit tokens for their campaigns, and then to subsequently monitor these campaigns.

There are a number of benefits from such protocol bidding. First, protocols participating in campaigns will benefit from increased order flow (in fees, liquidity, etc.). This solution also improves the chances that a user’s transaction will be fulfilled, given that there are tokens readily available from these campaigns. 

Bidded tokens can also offset user gas fees, making the transaction less expensive for the user (or for the solver, if the solver is paying for users’ gas). Composable itself will benefit as well, as we can use this process to earn revenue before the rest of the stack is fully built up, allowing us to optimally continue to build our ecosystem.

### Solver Rewards
Solvers will earn rewards from performing their role in MANTIS, as described above. These rewards will be allocated from a percentage of user funds processed through MANTIS. Rewards for the winning solver for each intent auction are calculated as follows:

`solver rewards = observedQuality - referenceScore`

In this equation, `solver rewards` are the amount distributed to the winning solver of a particular intent.

The `referenceScore` is that of the second highest solution, as determined by our scoring system.

The `observedQuality` relates to the quality of the settlement of a winning solution; more specifically this is the sum of the surplus generated from users and fees paid to the MANTIS framework. If the settlement fails, then `observedQuality` is zero. In this case of settlement failure, then the solver could end up paying the protocol, incentivizing solvers to present only feasible solutions to MANTIS.

### Solver Algorithm
The MANTIS solver algorithm can be viewed [here](https://github.com/ComposableFi/cvm/tree/main/mantis/node/src/solver) 

The mathematical process with which we generated and tested this algorithm is detailed [here](https://github.com/ComposableFi/cvm/tree/main/mantis/simulation) 

