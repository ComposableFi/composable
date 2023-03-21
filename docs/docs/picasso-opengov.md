# Picasso OpenGov tracks

Describes initial parameters [OpenGov](https://medium.com/polkadot-network/gov2-polkadots-next-generation-of-decentralised-governance-4d9ef657d11b) Picasso tracks settings. 

Definition of header and support/approval curves visualization are available in [Parity](https://wiki.polkadot.network/docs/maintain-guides-opengov) and [Moonbeam](https://moonbeam.network/blog/opengov/) documents. 
Tracks configurations and naming are keep close to [Parity](https://github.com/paritytech/polkadot/blob/master/runtime/kusama/src/governance/tracks.rs) and [Moonbeam](https://github.com/PureStake/moonbeam/blob/master/runtime/moonriver/src/governance/tracks.rs) setups too.


| Track/Origin        | Description                                       | Submission Deposit | Prepare Period | Decision Deposit | Max Deciding | Decision Period |     | Confirmation Period | Min Approval | Min Support | Min Enactment Period |
| ------------------- | ------------------------------------------------- | ------------------ | -------------- | ---------------- | ------------ | --------------- | --- | ------------------- | ------------ | ----------- | -------------------- |
| Root                | As SUDO, high gate, slowest, single               | 100 PICA           | 1 D            | 2 days           | 1            | 500K Pica       |     | 3 days              | 1            |             | 1                    |
| WhitelistedCaller   | As SUDO, fast, but with 1/2 of Fellowship         | 100 PICA           | 1 D            | 2 days           |              | 500K Pica       |     | 3 days              | 1            |             | 1                    |
| FellowshipAdmin     | Manage Fellowship                                 | 100 PICA           | 1 D            | 2 days           |              | 500K Pica       |     | 3 days              | 1            |             | 1                    |
| ReferendumCanceller | Cancel referenda/enactment                        | 100 PICA           | 1 D            | 2 days           |              | 500K Pica       |     | 3 days              | 1            |             | 1                    |
| SmallSpender        | Spend up to 10k PICA from Treasury                | 100 PICA           | 1 D            | 2 days           |              | 500K Pica       |     | 3 days              | 1            |             | 1                    |
| MediumSpender       | Spend up to 100k PICA from Treasury               | 100 PICA           | 1 D            | 2 days           |              | 500K Pica       |     | 3 days              | 1            |             | 1                    |
| BigSpender          | Spend up to 1M PICA from Treasury                 | 100 PICA           | 1 D            | 2 days           |              | 500K Pica       |     | 3 days              | 1            |             | 1                    |
| Treasurer           | Spend any amount from Treasury                    | 100 PICA           | 1 D            | 2 days           |              | 500K Pica       |     | 3 days              | 1            |             | 1                    |
| PabloAdmin          | Pablo operation which are not risk to platform    | 100 PICA           | 1 D            | 2 days           |              | 500K Pica       |     | 3 days              | 1            |             | 1                    |
| StakingAdmin        | Staking operation which are not risk to platform  | 100 PICA           | 1 D            | 2 days           |              | 500K Pica       |     | 3 days              | 1            |             | 1                    |
| BridgesAdmin        | Bridges operations which are not risk to platform | 100 PICA           | 1 D            | 2 days           |              | 500K Pica       |     | 3 days              | 1            |             | 1                    |
| AssetsAdmin         | Assets operations which are not risk to platform  | 100 PICA           | 1 D            | 2 days           |              | 500K Pica       |     | 3 days              | 1            |             | 1                    |

Up to date detailed reference and migration guide can be found [here](https://wiki.polkadot.network/docs/learn-opengov)
