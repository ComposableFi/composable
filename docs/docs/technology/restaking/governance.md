# Governance
 
The onboarding process for operators occurs following approval by PICA governance on Picasso Cosmos. Similarly, AVSs are required to undergo onboarding via PICA governance. The inclusion of asset types within the restaking layer also follows the same governance process. Any assets subjected to slashing will be redirected to the community pool on Picasso Cosmos.

Initially, the owner of the contracts on Picasso Cosmos will be set to a multisig. In the next stage of governance, the owner of the contracts on will be designated to the Upgrade Authority address. This ensures that PICA governance retains control over the contracts for the purpose of upgradability.

The restaking vault on Solana is currently governed by a multisig until decentralised governance is established. This is composed of a 7-of-9 multisig at address `JD4dNpiv9G24jmq8XQMuxbQPKN4rYV7kue2Hzi1kNT4Q`. As we move toward the launch of SOL IBC, we will look to expand this multisig in the greater pursuit of further decentralization. The **Admin & Upgradability multisig** is responsible for the following:

- Whitelisting tokens
- Setting the staking cap
- Setting if the guest chain is initialised or not
- Upgrading the contract
  
Signers for the multisig are as follows:

- Miguel Matos — Board member, Composable Foundation. Professor at the Universidade de Lisboa & Researcher at INESC-ID.
- Dan Edlebeck — Advisor, Composable
- Blas Rodriguez — CTO, Composable
- Joe DeTommaso — Head of Strategy, Composable
- Jafar Azam — Product Owner, Composable
- Dhruv D Jain — Research Analyst, Composable
- SolBlaze — bSOL LSD
- Don Cryptonium — Community Member
- Polkachu — Validator Operator

### Deployed Contracts
Currently, the Solana vault contract has been deployed on mainnet and is accepting restaked Solana LSTs. The contract address is `BoNnRkatYrN7ckA9oAU4e7cHYfwKPgLEuzkKg4LWaHeH`.