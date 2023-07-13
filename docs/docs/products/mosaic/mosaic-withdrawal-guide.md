# Mosaic Withdrawal Guide

:::note

Mosaic has officially been deprecated, and the front-end and its peripheral services have been taken offline in 2023. 
All funds and NFTs remain safe within their respective L1 and L2 Ethereum smart contract vaults 
and can still be manually withdrawn by the account holders via Etherscan following the instructions of this guide 
Additionally, if you need further assistance, feel free to contact our community managers on [Discord].

:::

[Discord]: https://discord.com/invite/composable

1. Open this link to the smart contract on etherscan in your browser: 
    https://etherscan.io/address/0xef4439f0fae7db0b5ce88c155fc6af50f1b38728#writeProxyContract

2. Navigate to the "Contract" tab towards the "Write as Proxy" tab

2. Connect your wallet to etherscan by clicking on "Connect to web3" and on the following prompt press "OK"

![connect_wallet](./images-mosaic-withdrawal-guide/contract-write-as-proxy.png)

3. Go to method `17. withdraw` and insert the address of the token you want to withdraw.

![method_withdraw](./images-mosaic-withdrawal-guide/method-withdraw.png)

- USDC - 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48

- wETH - 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2

- aUSDC - 0xBcca60bB61934080951369a648Fb03DF4F96263C

- SLP-USDT - 0x06da0fd433C1A5d7a4faa01111c044910A184553

- SLP-USDC - 0xBcca60bB61934080951369a648Fb03DF4F96263C

4. Click the "write" button and verify the transaction details.

:::caution

Remember to have sufficient funds available for gas fees!

:::

5. Approve the transaction in your wallet by clicking "Confirm".

    Your withdrawal is now complete
