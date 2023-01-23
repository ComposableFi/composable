# How to transfer USDT from Statemine to Picasso

The Statemine parachain acts as the issuer of fungible and non-fungible tokens on Kusama, 
this means if you want to transfer non-native assets on Kusama to use on Picasso, 
they first need to be transferred from the Statemine to your Picasso wallet.
You can find more details on how to move assets to the Statemine in this [article from Polkadot].

[article from Polkadot]: https://support.polkadot.network/support/solutions/articles/65000181634-how-to-withdraw-usdt-from-bitfinex-on-statemine

:::note
USDT is a “sufficient” asset, which means you don't need KSM in your Statemine account to receive USDT.
However, you do need KSM in the Statemine account in order to transfer USDT, since transaction fees are paid in KSM.
The process is the same as for USDT transfers from Bitfinex to Statemine.
:::

Read our guides on how to create and connect a [Talisman] or [Polkadot.js] wallet on Picasso.
Watch the video of the following guide here: https://youtu.be/SmTsDK1pI3Q

![picasso_homepage](./images-usdt-statemine-picasso-transfer/frontpage.png)

![picasso_transfers](./images-usdt-statemine-picasso-transfer/transfers.png)

1. Make sure your wallet is connected to Picasso by clicking the button “Wallets” in the top right corner. 
   If a wallet is already connected it reads “Connected”, 
   just make sure that in this case the intended wallet for the transfer is connected.
2. You should watch out that you have “Keep alive” toggle enabled. More about that later.
3. Click “Transfers” in the main menu to the left.

## Transfer Details

On the transfers page you will see the transfer details, including the transfer fee given in KSM. 
In the top right corner, next to your wallet, you can see that we have chosen to pay our transfer fees (Gas) in USDT, 
this will be important in a moment. For now just remember that we will be paying fees in USDT instead of KSM.

![transfer_details](./images-usdt-statemine-picasso-transfer/transfer-details.png)

1. Using the dropdown menus set the transfer to go from “Statemine” to “Picasso”
2. Choose the currency and amount you want to transfer, in our case we choose to transfer 100 USDT.
3. Make sure to keep your ED (existential deposit), using the “Keep Alive” toggle to avoid account deletion during reaping.
   Visit the Polkadot Wiki to learn more.
4. Click “Transfer”

Once you click transfer, you will be asked to sign your transaction. 
You will see the connected wallet from where the transaction originates and some technical details in this window.

![sign_statemine_transfer](./images-usdt-statemine-picasso-transfer/sign-statemine-transaction.png)

1. Sign the transaction using your password.
2. Click “sign the transaction”

You should see a notification pop-up at the bottom of the screen, 
clicking it will lead you to https://picasso.subscan.io/ where you can observe the chain processing your transaction. 
Depending on the current load of the network, this might take a moment.

![transfer_initiated](./images-usdt-statemine-picasso-transfer/transfer-initiated.png)

![transfer_confirmed](./images-usdt-statemine-picasso-transfer/transfer-confirmed.png)

Checking our transaction in the Polkadot.js explorer, 
we can see that the amount deposited is a little lower than 100 USDT 
because we paid our transaction fees in USDT, which in this case was deducted from the transfer amount.

Going back to Picasso the “Transfer successful” notification confirms your transaction. 
You can now use the funds we just transferred on Picasso.

[Legal Disclosures & Disclaimers](../faqs/legal-disclaimer-disclosures.md)

[Talisman]: https://docs.composable.finance/user-guides/talisman-create-account
[Polkadot.js]: https://docs.composable.finance/user-guides/polkadotjs-extension-create-account
