# How to create a Polkadot.js account for Picasso

[The Polkadot.js browser extension](https://polkadot.js.org/extension/) is a means to manage accounts and sign transactions in the DotSama ecosystem. 
Unlike Metamask, Polkadot.js doesn’t provide a full-featured wallet. Instead, 
its primary function is to inject your Polkadot.js account into other apps, such as Picasso.

This guide will show you how to set up a Polkadot address with the Polkadot.js extension for use in Picasso. 
We recommend reading through the complete instructions at least once before following along.

## Download the Polkadot.js extension
Click this link to download the extension for your preferred browser
[https://polkadot.js.org/extension/](https://polkadot.js.org/extension/).

Once downloaded and installed, you will see this screen.

![add_account](./images-polkadotjs-extension-create-account/add-account.png)

1. Click “Understood, let me continue”

To avoid the extension minimizing or canceling the account creation if you navigate outside it, 
you can open it in a new window.

![extension_windowed](./images-polkadotjs-extension-create-account/extension-windowed.png)

1. Navigate to the ⚙️ dropdown menu
2. Click “Open in new window”

## Create a new account

Once you have accepted the terms and conditions, you will be greeted by the extension window asking you to create an account.

![create_new_account](./images-polkadotjs-extension-create-account/create-new-account.png)

1. Click on “+” in the top right corner
2. Next, click on "Create new account"

## Secure your passphrase

This opens the “Create an account” window, showing a twelve-word mnemonic seed (passphrase). 
You can use it to restore your wallet, 
and we recommend you to keep your passphrase safe using a password manager of your choice to prevent losing your assets.

Please DO NOT share your passphrase with anyone. 
Your seed grants full access to ALL funds stored in that account and all accounts derived from it.

![mnemonic_seed](./images-polkadotjs-extension-create-account/mnemonic-seed-polkadotjs.png)

1. Once you have secured your passphrase, check the corresponding box
2. Click "Next Step"

## Account details

On the next page, fill in the form as follows:

![account_credentials](./images-polkadotjs-extension-create-account/account-credentials.png)

1. In this case, we simply choose to use it on any DotSama chain for ease of use.
   You can select the network you will use with the account.
   For more details on how accounts work in Polkadot/Kusama, please read the Polkadot documentation.
2. Add a display name for the account (e.g., Picasso)
3. Set a password. You will sign transactions with this password. Please note: 
   This password does not protect your seed phrase. If someone can get access to your mnemonic seed, 
   they can gain control over your account even if they do not know your password.
4. Confirm the password
5. Click “Add the account with the generated seed” to complete the account generation

Congratulations! You can now integrate your Polkadot.js account into Picasso, 
initiate transactions, sign them using your password and submit them. 
So let’s continue by integrating this new account into Picasso.

## Connect to Picasso
Connect to Picasso:
To use the complete feature set of Picasso, a wallet needs to be connected. 
You can use the Polkadot.js account we just created or a Talisman account. 
Please read our guide on [how to make a Talisman account](./talisman-create-account.md) for Picasso.

![picasso_frontpage](./images-polkadotjs-extension-create-account/frontpage.png)

1. Click "Wallets" in the top right corner
2. Click "Polkadot"
3. Select a wallet of your choice. In this case, we use the Polkadot.js account we created earlier

You will be prompted to allow Picasso access to your account's addresses. 
Make sure it identifies as Picasso and its origin "app.picasso.xyz"

![picasso_access_request](./images-polkadotjs-extension-create-account/access-request.png)

1. Click "Yes, allow this application access"

![choose_your_wallet](./images-polkadotjs-extension-create-account/choose-your-wallet.png)

On the following page, you will see all the wallets associated with the connected account. 
If you create/derive more wallets in the future, they will show up here.

1. Click the wallet we created earlier, in our example named “Picasso Demo”
2. Click “confirm account”

And we are done. You can use Picasso as you please using the accounts and their respective wallets we just connected, 
or you can try connecting an Ethereum wallet using Metamask.

You can follow along with the video below: 
[![How to create a Polkadot.js account](https://img.youtube.com/vi/tRrrF37MxBc/maxresdefault.jpg)](https://www.youtube.com/watch?v=tRrrF37MxBc)