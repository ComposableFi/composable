# Bug Bounty Programs

## Immunefi Bounties
As a part of Composable’s ongoing commitment to the security of our dual parachains, 
we are proud to announce the renewal of our bug bounty program in collaboration with Immunefi. 
Immunefi is the blockchain industry’s leading bug bounty program, having secured more than $60 billion in user funds. 
There have been some adjustments to the scope and rewards of this program which will be outlined in this article.

Immunefi classifies bugs based on a [5-level] scale ranging from: none, low, medium, high, and critical. 
In addition, Immunefi has separate scales for websites/apps, smart contracts, and blockchain/DLT. 
As a team building infrastructure across 2 parachains in the form of pallets, 
which are incorporated directly into the runtime of our blockchains, 
we are primarily concerned with blockchain/DLT related bugs. 
As a result we will be excluding website/app and smart contract related bugs from the scope of this program.

## Program details
Immunefi classifies bugs based on a 5-level scale ranging from: none, low, medium, high, and critical. 
In addition, Immunefi has separate scales for websites/apps, smart contracts, and blockchain/DLT. 
As a team building infrastructure across 2 parachains in the form of pallets, 
which are incorporated directly into the runtime of our blockchains, 
we are primarily concerned with blockchain/DLT related bugs. 
As a result we will be excluding website/app and smart contract related bugs from the scope of this program.

### Assets in scope
The [Picasso Parachain] is the primary focus of this bounty program. You can find a link to our GitHub [here], 
but keep in mind that only blockchain/DLT related bugs will be accepted for the purpose of this program. 
Rewards will be provided for medium, high, and critical vulnerabilities. 
A breakdown of this reward structure can be found below:

### Blockchain/DLT bounties
Critical ($50,000 - $75,000)
- Network not being able to confirm new transactions (Total network shutdown)
- Unintended permanent chain split requiring hard fork (Network partition requiring hard fork)
- Direct loss of funds
- Permanent freezing of funds (fix requires hardfork)
- RPC API crash
- Ability to halt the chain or alter block productions / the network by providing bad input data

High ($10,000 - $30,000)
- Unintended chain split (Network partition)
- Transient consensus failures

Medium ($2,000 - $5,000)
- High compute consumption by validator/mining nodes
- Attacks against thin clients

All Critical and High Blockchain/DLT bug reports must come with a PoC 
with an end-effect impacting an asset-in-scope in order to be considered for a reward. 
Explanations and statements are not accepted as PoC and code is required. 
Reward maximums vary case-by-case with direct regard to the impact of the threat presented and scoped.

Additionally, KYC must be completed for all bug bounty hunters submitting a report in order to collect their reward. 
The information needed are 2 forms of Photo Identification and Proof of Address.
The collection of this information will be done by the Composable Finance team.

## Prohibited activities
As mentioned previously, 
any vulnerabilities regarding smart contracts or web applications are excluded from the scope of this program. 
In addition, the following activities are prohibited by this program:

- Any testing with mainnet or public testnet contracts; all testing should be done on private testnets
- Attacks that the reporter has already exploited themselves, leading to damage
- Attacks requiring access to leaked keys/credentials
- Attacks requiring access to privileged addresses (governance, strategist)
- Any testing with pricing oracles or third party smart contracts
- Attempting phishing or other social engineering attacks against our employees and/or customers
- Any testing with third party systems and applications (e.g. browser extensions) as well as websites (e.g. SSO 
  providers, advertising networks)
- Any denial of service attacks
- Automated testing of services that generates significant amounts of traffic
- Public disclosure of an unpatched vulnerability in an embargoed bounty


[5-level]: https://immunefi.com/immunefi-vulnerability-severity-classification-system-v2-2/
[Picasso Parachain]: https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/explorer
[here]: https://github.com/ComposableFi