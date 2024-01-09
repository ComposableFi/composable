# How to LP & stake DOT on Pablo via Trustless Zone

The [IBC](../technology/ibc.md) DOT-KSM connection serves as the first transfer protocol between Kusama (KSM) and Polkadot (DOT), unlocks the potential for a seamless flow of liquidity between parachains on both networks. Token transfers are a prime example of this, enabling increased liquidity across protocols on both chains. It is important to note that IBC powers functionality beyond token transfers and unlocks composability between Polkadot and Kusama. One could possibly send XCM messages using IBC as a transport layer. This would enable XCM execution on Kusama or a Kusama parachain sent from Polkadot or a Polkadot parachain. 

This guide outlines the steps involved in transferring DOT from Polkadot to Picasso using XCM and IBC to provide liquidity on [Pablo](../technology/pablo-overview.md).

There are two steps required to complete the process:

1. Multihop DOT from Polkadot to [Picasso](../networks/picasso-parachain-overview.md)
2. Provide liquidity & stake using DOT paired with various pools on Pablo


## Multihop DOT from Polkadot to Picasso

Head to [app.trustless.zone](https://app.trustless.zone/) and select `Polkadot` and `Picasso` as the source and destination chains respectively. Enter the amount you wish to transfer, press 'Send' and sign the transaction.

![trustless-1](./images-dot-lp-guide/ibc-dot-picasso.png)
## Provide liquidity to DOT pairs on Pablo

On [app.pablo.finance](https://app.pablo.finance/), connect your wallet and navigate to the "Provide liquidity" page.

Choose one of the available pools (in this guide, we will use PICA/DOT).

![pablo-1](./images-dot-lp-guide/pablo-lp-1.png)

Enter the PICA/DOT pool page and select "Add Liquidity". Enter the amount you wish to deposit and select "Add Liquidity" once more. 

A pop-up will appear to confirm the transaction by clicking "Confirm" and then sign the transaction. 

A "Transaction success" notification will appear and you can click the notification to view the transaction on Subscan.


![pablo-2](./images-dot-lp-guide/pablo-lp-2.png)

Head back to the pool overview page and click "Stake". Enter the amount of LP tokens you wish to stake and click "Stake PICA-DOT" to start earning rewards on your LP position. You will be asked to sign the transaction with your password.

![pablo_3](./images-dot-lp-guide/stake-3.png)


Congratulations! You have successfully staked liquidity on Pablo.