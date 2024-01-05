# Interacting with Picasso governance

Picasso Kusama utilizes Polkassembly for on-chain governance participation. Follow the proposal roadmap if you wish to take part in initiating a proposal on Picasso.

## Proposal Roadmap

Any PICA token holder has the ability to create, vote and contribute to Referenda on Picasso. See [this section](../networks/picasso/governance.md) for further details on proposal parameters.

1. A proposal author should submit their idea to Picasso’s Polkassembly governance forum, where they should be open to community feedback for at least five days before moving forward
2. Taking into account feedback, the proposal author can submit their proposal on-chain
   - The proposer must first submit the preimage (if you need assistance with creating the preimage or would like secondary approval, reach out to our team on Discord)
   - Note: your preimage deposit will be returned once via unnoting after the proposal is submitted
   - The proposer then can submit the Referendum, and place the decision deposit (which covers the on-chain storage cost of the proposal)
3. Thus veins the lead-in period, where the community can begin voting
4. The proposal will then move to the decision period when the following are met:
   - The referenda waits the duration of the prepare period (ensuring enough time for discussion)
   - There is capacity in the chosen track
   - A decision deposit has been submitted and meets the minimum requirements of the track
5. During the decision period, voting continues and the referendum has a set amount of days to reach approval.
   - If the Referendum is rejected, the decision deposit will be returned
6. If the Referendum is approved, it enters the confirm period where it must remain approved for the duration of this period.
   - If the referendum fails to meet these requirements at any time, it moves back to the decide period; if it again meets these requirements, it moves back to the confirm period and the decide period is delayed until the end of the confirm period
7. If the referendum receives enough approval and support throughout the confirm period, it will be approved and move to the enactment period
8. Once the enactment period elapses, the referendum will be executed

## How to create a proposal

After you have left your proposal in discussion phase for at least 5 days, you can put the referendum on-chain via Polkadotjs. Feel free to reach out to the team on Telegram or Discord for assisting with posting proposals on-chain.

1. Head to https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.composablenodes.tech#/preimages and click 'Add preimage'

   - Choose the extrinsic you wish to execute. Your selection will be crucial for subsequent steps. In this example, the system pallet and the remark extrinsic are being used for a Root proposal. 
   - To submit the proposal later, copy the preimage hash, then click the "Submit preimage" button and sign the transaction. You can also view the preimage hash later on the Preimages page if it has been noted. 

![preimage](../user-guides/images-picasso-governance/submit-preimage.png)

2. Head to https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.composablenodes.tech#/referenda and click 'Submit proposal'. 

   - Select the relevant Origin class for your proposal's execution. Selecting an incorrect Track/Origin may lead to the failure of your proposal during execution.
   - Select the Track for proposal submission. The associated Origin within the Track must possess authority to execute the intended action.
   - Within the Origins dropdown, choose the specific Origin, which in this case is "System."
   - Input the preimage hash linked to the proposal. Next, choose the moment of enactment, either after a specified number of blocks or at a specific block.
   - Finally, click "Submit proposal" and proceed to sign the transaction.

![submission](../user-guides/images-picasso-governance/submit-proposal.png)

## How to vote on a proposal

There are two methods to vote on Referenda: 

1. Click on 'Cast Vote Now' on the referenda's Polkassembly page as highlighted in the screenshot below. You must be signed in to use this method.

![voteassembly](../user-guides/images-picasso-governance/voting.png)

2. Click on 'Vote' on Referenda displayed on https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.composablenodes.tech#/referenda 

![votepdjs](../user-guides/images-picasso-governance/voting-2.png)

When voting, select your voting preference for the referendum. Options include "Aye" to support the referendum, "Nay" to oppose it, or "Split" if you wish to allocate distinct values for both "Aye" and "Nay" votes. Then, enter your chosen vote value and the vote conviction.

Finally, click "Vote" and proceed to sign the transaction. 

## How to create a Polkassembly account
 
First, log in to Polkassembly via a web 3 wallet or a username/email. If you have used Polkassembly in the past you will still need to complete these steps to link your Picasso wallet:

1. Visit https://picasso.polkassembly.io/ and click Login on the top right corner 

2. Choose the account you would like to participate with and click sign-up

![polkassembly_choose_account](./images-picasso-governance/choose-account.png)

5. Once your account has been created you can sign in by clicking on the login button and entering your details.

6. When signed in, you can view all of the latest discussions, proposals, referenda, and upcoming events on Picasso.

![polkassembly_logged_in](./images-picasso-governance/logged-in.png)