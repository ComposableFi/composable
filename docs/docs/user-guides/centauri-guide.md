# How to LP DOT on Pablo via Centauri

[Centauri](../products/centauri-overview.md), as the first transfer protocol between Kusama (KSM) and Polkadot (DOT), unlocks the potential for a seamless flow of liquidity between parachains on both networks. Token transfers are a prime example of this, enabling increased liquidity across protocols on both chains. This guide outlines the steps involved in transferring DOT from Polkadot to Picasso using XCM and IBC to provide liquidity on [Pablo](../products/pablo-overview.md).

:::tip DOT Pools on Pablo are live!

At present, there are three liquidity pools on [Pablo](https://app.pablo.finance/provide-liquidity/) that are paired with DOT.

- PICA/DOT
- KSM/DOT
- DOT/USDT

:::

There are three steps required to complete the process:

1. XCM DOT from Polkadot to [Composable](../parachains/composable-parachain-overview.md)
2. IBC DOT from Composable to [Picasso](../parachains/picasso-parachain-overview.md)
3. Provide liquidity using DOT paired with either KSM or USDT on Pablo


## XCM DOT from Polkadot to Composable

Head to [app.trustless.zone](https://app.trustless.zone/) and click on "Select network".

![select_network](./images-centauri-guide/centauri-guide-1.png)

Connect your wallet to the "Polkadot" network as the source chain. Make sure the intended wallet for the transfer is connected, you can navigate across different wallets connected in the proxy wallet.

![connect_to_polkadot](./images-centauri-guide/centauri-guide-2.png)

Set Composable as the destination chain.

![select_composable](./images-centauri-guide/centauri-guide-3.png)

Enter the amount of DOT you wish to transfer to Composable. Keep in mind to leave at least 1 DOT for the existential deposit. Your receiving wallet address will be auto-populated.

![enter_dot_amount](./images-centauri-guide/centauri-guide-4.png)

Confirm the transaction.

![confirm_xcm](./images-centauri-guide/centauri-guide-5.png)

A pop-up will appear asking you to sign the transaction using your password. Once the transaction is signed, wait for the XCM transfer to finalize.

![xcm_success_fast](./images-centauri-guide/centauri-guide-6.png)

You have successfully received DOT on the Composable parachain.

## IBC DOT from Composable to Picasso

Select Composable as the source chain and Picasso as the destination chain.

![composie_pica_one](./images-centauri-guide/centauri-guide-7.png)

Enter the amount of DOT you want to transfer and **adjust the gas fees for the IBC transfer to DOT in the settings**.

![composie_pica_two](./images-centauri-guide/centauri-guide-8.png)

Confirm the transaction.

![confirm_ibc](./images-centauri-guide/centauri-guide-9.png)

Sign the transaction using your wallet and wait for the IBC transfer to finalize. 

![sign_ibc](./images-centauri-guide/centauri-guide-10.png)

At this stage, you have successfully received DOT on Picasso.

## Provide liquidity to DOT pairs on Pablo

On [app.pablo.finance](https://app.pablo.finance/), connect your wallet and navigate to the "Pool" page.

Choose one of the available pools (in this guide, we will use PICA/DOT).

![pablo-1](./images-centauri-guide/pablo-lp-1.png)

Select the "Add Liquidity" button and enter the amount you wish to deposit into the liquidity pool. Click "Add Liquidity" on the following page and confirm the transaction by clicking "Confirm". Sign the transaction with your password. 

![pablo-2](./images-centauri-guide/pablo-lp-2.png)

A "Transaction success" notification will appear and you can click the notification to view the transaction on Subscan.

Congratulations! You have successfully added liquidity to a pool on Pablo.
