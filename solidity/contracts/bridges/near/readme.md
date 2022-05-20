Official documentation: https://github.com/djsatok/bridge-testing#ethereum-to-near-transfer

## Bridge deploymnets

### Mainnet

#### Ethereum

1. Client
   address: [0x0151568af92125fb289f1dd81d9d8f7484efc362](https://etherscan.io/address/0x0151568af92125fb289f1dd81d9d8f7484efc362)
2. Prover
   address: [0x051ad3f020274910065dcb421629cd2e6e5b46c4](https://etherscan.io/address/0x051ad3f020274910065dcb421629cd2e6e5b46c4)
3. ERC20 connector
   address: [0x23ddd3e3692d1861ed57ede224608875809e127f](https://etherscan.io/address/0x23ddd3e3692d1861ed57ede224608875809e127f)

#### NEAR

1. Client account: [client.bridge.near](https://explorer.near.org/accounts/client.bridge.near)
2. Prover account: [prover.bridge.near](https://explorer.near.org/accounts/prover.bridge.near)
3. ERC20 connector account: [factory.bridge.near](https://explorer.near.org/accounts/factory.bridge.near)

### Testnet

#### Ropsten

1. Client
   address: [0xb289c6e6c98644dc9f6a03c044564bc8558b6087](https://ropsten.etherscan.io/address/0xb289c6e6c98644dc9f6a03c044564bc8558b6087)
2. Prover
   address: [0xb3df48b0ea3e91b43226fb3c5eb335b7e3d76faa](https://ropsten.etherscan.io/address/0xb3df48b0ea3e91b43226fb3c5eb335b7e3d76faa)
3. ERC20 connector
   address: [0xb48e6441524f261e141bc766a7ebd54b19ca7465](https://ropsten.etherscan.io/address/0xb48e6441524f261e141bc766a7ebd54b19ca7465)

#### NEAR Testnet

1. Client account: [client.ropsten.testnet](https://explorer.testnet.near.org/accounts/client.ropsten.testnet)
2. Prover account: [prover.ropsten.testnet](https://explorer.testnet.near.org/accounts/prover.ropsten.testnet)
3. ERC20 connector account: [f.ropsten.testnet](https://explorer.testnet.near.org/accounts/f.ropsten.testnet)

## Ethereum to NEAR transfer

1. **Approve transaction**. Send an `approve` transaction to ERC20 contract. This step implies setting an alowance for a
   connector contract, so it can withdraw the tokens from your account. Arguments that you should use for this
   transaction: `ConnectorAddress` and `DepositAmount`. A sample script that implements sending this transaction
   is `src/1-erc20-approve.ts`. To run it use the following comand: `$ node build/1-erc20-approve.js`. **Note**: In case
   you're doing multiple transfers of the same ERC20 token, you can combine approvals into a single approve transaction
   with the sum of the amounts. This will reduce the gas costs for this step.
2. **Locking transaction**. Send a `lock` transaction to `TokenLocker` contract. This step implies locking of
   a `DepositAmount` tokens in a locking contract, while specifying the NEAR `AccountID`, where bridged tokens should be
   transferred. Locking method emits an event, which will be used later to prove the fact of locking of funds. See the
   implementation [here](https://github.com/near/rainbow-token-connector/blob/master/erc20-connector/contracts/ERC20Locker.sol#L32-L35)
   . A sample script that implements sending this transaction is `src/2-connector-lock.ts`. To run it use the following
   CLI command: `$ node build/2-connector-lock.js`.
3. **Wait sufficiently long**. 20 confirmations for Ethereum<>NEAR mainnet deployment. This is needed to achieve
   finality of Ethereum block, which includes the locking transaction. The status of syncing of the bridge can be
   observed [here](http://34.94.229.96:8002/metrics). First metric (`near_bridge_eth2near_client_block_number`) should
   become more than the height of a block with transaction from the step 2 at least by 20, for successfull finalisation
   of the transfer.
4. **Finalisation of the transfer**. Call minting transaction in NEAR blockchain. This step implies calling a `deposit`
   method of the NEAR token factory contract. The method consumes [Borsh](https://github.com/near/borsh)-ified proof of
   the event, emitted during the step 2 transaction execution. The script that implements proof calculation is located
   at `src/generate-proof.js`, while the finalisation script is located at `src/3-finalise-deposit.ts`. To perform this
   step, find in the output of the step 2 a hash of the locking transaction, then use the following CLI
   command `$ node build/3-finalise-deposit.js <TransactionHash>`. **Note**: In case the token was not previously
   deployed to NEAR blockchain (was never bridged before), an additional call to `deploy_bridge_token` will be done
   automatically. This call deploys the bridged token contract and requires a deposit of 3.5 $NEAR.

## NEAR to Ethereum transfer

1. **Begin withdraw**. Send a `withdraw` transaction to the bridged token contract. During the execution, a token
   factory contract will be called and issue a receipt, which would be used during finalisation step to contruct the
   proof for the locking contract in Ethereum. This step is implemented in `src/4-begin-withdraw.ts`. To perform this
   step call: `$ node build/4-begin-withdraw.js`.
2. **Wait sufficiently long**. This takes around 12 hours for the Ethereum<>NEAR mainnet deployment. This is needed to
   relay NEAR block with the height higher than the block with transaction from previous step to Ethereum, plus wait a
   challenge period (4 hours). The status of syncing of the bridge can be
   observed [here](http://34.94.229.96:8001/metrics). First metric `near_bridge_near2eth_client_height` should become
   higher than the block height displayed in console during the previous step.
3. **Finalise withdraw**. Send an `unlock` transaction to the locking contract. After bridge syncing we are able to
   prove the fact of the withdrawal on NEAR. Script `src/5-finalise-withdraw.ts` implements calculation of the
   correspondent proof (with the help of `src/borshify-proof.js`) and sends this proof in the locking contract. To
   perform this step, find in the output of the step 1 a receipt of the transaction, then use the following CLI
   command `$ node build/4-finalise-withdraw.js <Receipt>`.

## Example of transfers

### Ethereum -> NEAR transfer

- [ERC20 Approve](https://etherscan.io/tx/0x98b0c7b977eb701769dfb22c18539bc4a87539ef67499fa49ad543bdfc3b8ef2) (44,046
  ETH Gas)
- [Lock](https://etherscan.io/tx/0x250607fdc1afab0ad183cf008e296839e8d4e3a5f14f2a290f27470f030ea80c) (56,088 ETH Gas)
- [Finish deposit](https://explorer.near.org/transactions/4MCiuNHSnkrHceFyw86PxgwGTANhJDEcCmBxuEzz6tjT) (63 NEAR TGas)

### NEAR -> Ethereum transfer

- [Withdraw](https://explorer.near.org/transactions/GkcKPbX8sRxUQBJNp71rhG7Ev93cqvDRiCvtPaEsG8pH) (13 TGas)
- [Finish withdraw](https://etherscan.io/tx/0x54d9a80a871663c0e94203fe423e61f0a3ee12f36ce1424cb87b5caad0656141) (286,232
  ETH Gas)
