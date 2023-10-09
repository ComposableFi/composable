# Composable Strategies Withdrawal Process

:::info

Composable Strategies has officially been deprecated. 
The front-end and its peripheral services have been taken offline in 2023. 
All funds remain safe within their respective Ethereum smart contract vaults 
and can still be withdrawn manually by the account holders via Etherscan following the guide on this page. 
Additionally, if you need further assistance, feel free to contact our community managers on Discord.

:::

1. GENERIC WITHDRAW of **TOKEN  ( = DAI, USDC, USDT)**

For withdrawing **DAI**, go to the strategy address:
```
https://etherscan.io/address/0x4A03ea61E543eC7141a3f90128B0c0c9514F8737
```

For withdrawing **USDC**, go to the strategy address: 
```
https://etherscan.io/address/0xF12dA8470E2643cCb39a157e8577D9AA586a488f
```

For withdrawing **USDT**, go to the strategy address:  
```
https://etherscan.io/address/0x1941441d31809e9E1828Da0cE6d44175F657E215
```

2. Go to ‘Contract’ Tab and click ‘Read as Proxy’ 

![contract_read_as_proxy](./images-composable-strategies-withdrawal-guide/contract-read-as-proxy.png)

3. Go to method `19 - userInfo`. Input your wallet address and select ‘Query’ and copy the ‘amountfToken’ value.

![query_amountfToken](./images-composable-strategies-withdrawal-guide/query-amountfToken.png)

4. Next, go to the ‘Write as Proxy’ tab under Contract and select “Connect to Web3” and press OK to connect with metmask.

![contract_write_as_proxy](./images-composable-strategies-withdrawal-guide/contract-write-as-proxy.png)

5. Go to method `20 - withdraw` and fill in the following:

| Field    | Input                 | Details                           |
|----------|-----------------------|-----------------------------------|
| amount   | amountfToken          | value of amountfToken from step 3 |
| deadline | 100000000000000000000 | use this exact value              |
| slippage | 50                    | this is 0.5%                      |

- ethPerToken (uint256): detailed at 5. a)
- ethPerFarm (uint256): detailed at 5. b)
- tokensPerEth (uint256): detailed at 5. c)

  Go the to UniswapV2 Router contract used by the strategy at
  https://etherscan.io/address/0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D 
  
   You can check for this address under the contract from step 1 at method `14 - sushiswapRouter`

![sushiswapRouter_account_address](./images-composable-strategies-withdrawal-guide/sushiswapRouter-account-address.png)

On this address, select 'Contract' and then 'Read Contract' and select method `6 - getAmountsOut`

![contract_read_getAmountsOut](./images-composable-strategies-withdrawal-guide/contract-read-getAmountsOut.png)

Token, Address and Decimals for reference in the following steps:

| TOKEN | Generic Withdraw Address                   | Decimals |
|-------|--------------------------------------------|----------|
| DAI   | 0x6B175474E89094C44Da98b954EedeAC495271d0F | 18       |
| USDC  | 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 | 6        |
| USDT  | 0xdAC17F958D2ee523a2206206994597C13D831ec7 | 6        |

5. a) To get the ethPerToken

   In the input field called `amountIn` insert value 1 * 10 to the power of TOKEN decimal places.
   (18 decimals for DAI, 6 for USDC and 6 for USDT)

   In the second field called path `address[]` you need to enter the address of TOKEN
   (address of DAI, USDC or USDT) then a comma with no space followed by the contract address of wETH
   (0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2) and press Query.

![ethPerToken_getAmountsOut](./images-composable-strategies-withdrawal-guide/ethPerToken-getAmountsOut.png)

The value after the decimal point is the input value for field 5.a


5. b) To get the ethPerFarm

   In the input field called `amountIn` insert value 1 * 10 to the power of FARM token decimal places.
   https://etherscan.io/token/0xa0246c9032bC3A600820415aE600c6388619A14D?a=0x4A03ea61E543eC7141a3f90128B0c0c9514F8737  
   (18 decimals)

   In the second field called path `address[]` you need to enter the address of FARM 
   (0xa0246c9032bC3A600820415aE600c6388619A14D) then a comma with no space followed by the contract address of wETH 
   (0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2) and press Query.

![ethPerFarm_getAmountsOut](./images-composable-strategies-withdrawal-guide/ethPerFarm-getAmountsOut.png)

The value after the decimal point is the input value for field 5.b

5. c) To get the tokensPerEth

   In the first input field called `amountIn` input the value 1 * 10 at power of 18. 


   In the second field called path `address[]` you need to enter the first contract address of wETH 
   (0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2) and then add a comma with no space followed by the address of TOKEN 
   (address of DAI, USDC or USDT) and press Query.

![tokensPerEth_getAmountsOut](./images-composable-strategies-withdrawal-guide/tokensPerEth-getAmountsOut.png)

The value after the decimal point is the input value for field 5.c

6. With all the above data filled in just press WRITE and sign the transaction with your Metmask wallet.  

   Be careful we do recommend to set Gas limit to at least 1.5 mil as this is going to be a complex Tx.
   Check to have enough funds to execute it.

![confirm_transaction](./images-composable-strategies-withdrawal-guide/confirm-transaction.png)

:::caution

If your metmask displays the message below
You need to check all the above data as some input data is wrong and the transaction will fail automatically. 

:::

![troubleshoot_transaction](./images-composable-strategies-withdrawal-guide/troubleshoot-transaction.png)
