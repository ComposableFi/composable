# Picasso OpenGov tracks

Describes initial parameters [OpenGov](https://medium.com/polkadot-network/gov2-polkadots-next-generation-of-decentralised-governance-4d9ef657d11b) Picasso tracks settings. 

Definition of header and support/approval curves visualization are available in [Parity](https://wiki.polkadot.network/docs/maintain-guides-opengov) and [Moonbeam](https://moonbeam.network/blog/opengov/) documents. 
Tracks configurations and naming are keep close to [Parity](https://github.com/paritytech/polkadot/blob/master/runtime/kusama/src/governance/tracks.rs) and [Moonbeam](https://github.com/PureStake/moonbeam/blob/master/runtime/moonriver/src/governance/tracks.rs) setups too.

| Track/Origin            | Description                                       | Submission Deposit | Prepare Period | Decision Deposit | Max Deciding | Decision Period | Confirmation Period | Approval (min=50%)                    | Support                                     | Min Enactment Period |
| ----------------------- | ------------------------------------------------- | ------------------ | -------------- | ---------------- | ------------ | --------------- | ------------------- | ------------------------------------- | ------------------------------------------- | -------------------- |
| **Root**                | As SUDO, high gate, slowest, single               | 1K PICA            | 2h             | 500K PICA        | 1            | 7D              | 1D                  | inverse(max=100%, kink=T25%** to 80%) | linear(max=50%, decay=100%)                 | 1d                   |
| **WhitelistedCaller**   | As SUDO, fast, but with 1/2 of Fellowship         | 1K PICA            | 30m            | 5K PICA          | 2            | 4D              | 10m                 | inverse(max=100%, kink=T25% to 90%)   | linear(max=50%, decay=100%, min=5%)         | 10m                  |
| FellowshipAdmin         | Manage Fellowship                                 | 1K PICA            | 2h             | 2K PICA          | 3            | 2D              | 1D                  | inverse(max=100%, kink=T25% to 80%)   | inverse(max=100%, kink=T25% to 80%, min=0%) | 10m                  |
| **ReferendumCanceller** | Cancel referenda/enactment                        | 1K PICA            | 30m            | 5K PICA          | 5            | 3D              | 1h                  | linear(max=100%, decay=100%)          | inverse(max=50%, kink=T25% to 1%, min=0%)   | 10m                  |
| SmallSpender            | Spend up to 10k PICA from Treasury                | 1K PICA            | 2h             | 1K PICA          | 10           | 6h              | 1h                  | inverse(max=100%, kink=T25% to 80%)   | inverse(max=100%, kink=T25% to 80%, min=0%) | 1m                   |
| MediumSpender           | Spend up to 100k PICA from Treasury               | 1K PICA            | 2h             | 2K PICA          | 5            | 1D              | 2h                  | inverse(max=100%, kink=T25% to 80%)   | inverse(max=100%, kink=T25% to 80%, min=0%) | 10m                  |
| BigSpender              | Spend up to 1M PICA from Treasury                 | 1K PICA            | 2h             | 2K PICA          | 3            | 2D              | 3h                  | inverse(max=100%, kink=T25% to 80%)   | inverse(max=100%, kink=T25% to 80%, min=1%) | 10m                  |
| **Treasurer**           | Spend any amount from Treasury                    | 1K PICA            | 2h             | 5K PICA          | 2            | 3D              | 12h                 | inverse(max=100%, kink=T25% to 80%)   | linear(max=50%, decay=100%, min=5%)         | 1h                   |
| PabloAdmin              | Pablo operation which are not risk to platform    | 1K PICA            | 2h             | 2K PICA          | 3            | 1D              | 2h                  | inverse(max=100%, kink=T25% to 80%)   | inverse(max=100%, kink=T25% to 80%, min=2%) | 1h                   |
| StakingAdmin            | Staking operation which are not risk to platform  | 1K PICA            | 2h             | 2K PICA          | 3            | 1D              | 2h                  | inverse(max=100%, kink=T25% to 80%)   | inverse(max=100%, kink=T25% to 80%, min=1%) | 1h                   |
| **BridgesAdmin**        | Bridges operations which are not risk to platform | 1K PICA            | 2h             | 2K PICA          | 3            | 1D              | 2h                  | inverse(max=100%, kink=T25% to 80%)   | inverse(max=100%, kink=T25% to 80%, min=2%) | 1h                   |
| AssetsAdmin             | Assets operations which are not risk to platform  | 1K PICA            | 2h             | 2K PICA          | 3            | 1D              | 2h                  | inverse(max=100%, kink=T25% to 80%)   | inverse(max=100%, kink=T25% to 80%, min=2%) | 1h                   |

** T25% - means that kind will happen after 25% of `Decision Period`

One picture worth of 1000 tables, so flowing this table from left to right as time goes along with visual plot of support/approval curves best way to grasp the concept.

Up to date detailed reference and migration guide can be found [here](https://wiki.polkadot.network/docs/learn-opengov)
