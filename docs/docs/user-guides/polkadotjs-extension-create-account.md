# How to create a Polkadot.js account

[The Polkadot.js browser extension](https://polkadot.js.org/extension/) is a means to manage accounts and sign transactions in the DotSama ecosystem. 
Unlike Metamask, Polkadot.js doesn’t provide a full-featured wallet. Instead, 
its primary function is to inject your Polkadot.js account into other apps, such as Pablo.

This guide will show you how to set up a Polkadot address with the Polkadot.js extension for use on Composable Apps although it is recommended for users to interact via alternative user-friendly wallet solutions such as Nova, Talisman and SubWallet. 

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
2. Add a display name for the account (e.g., Pablo Picasso)
3. Set a password. You will sign transactions with this password. Please note: 
   This password does not protect your seed phrase. If someone can get access to your mnemonic seed, 
   they can gain control over your account even if they do not know your password.
4. Confirm the password
5. Click “Add the account with the generated seed” to complete the account generation

Congratulations! You can now integrate your Polkadot.js account into Composable applications to initiate transactions and sign them using your password. 