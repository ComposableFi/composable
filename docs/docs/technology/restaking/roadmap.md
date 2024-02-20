# Roadmap

The rollout plan for Picasso Generalized restaking will proceed as follows:

1. Initial implementation of the [Restaking Vaults on Solana](../restaking/vaults.md).
2. Launch of the first [AVS for Solana IBC](../restaking/sol-ibc-avs.md).
3. Begin onboarding AVSs onto the Solana Restaking layer.
4. Expansion to include vaults for Cosmos ecosystem assets on Picasso.
5. Launch of Restaking Layer on Picasso including all the necessary contracts.
6. Migration of Solana IBC AVS slashing parameters to Picasso. As the generalized restaking layer contracts are still in the development phase, the slashing process is currently managed by the first AVS for Solana IBC. Upon the launch of the Orchestrator contract, slashing logic will transition to be overseen by the orchestrator on Picasso.
7. Validators of the AVS for Solana IBC act as operators of this AVS, they have the opportunity to operate other AVSes in the future.
8. Full launch of the generalized staking layer.

