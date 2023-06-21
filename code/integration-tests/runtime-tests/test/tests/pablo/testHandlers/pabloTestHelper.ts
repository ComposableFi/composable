import { sendAndWaitForSuccess, sendWithBatchAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { u128 } from "@polkadot/types-codec";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";
import { CustomRpcCurrencyId, PalletPabloPoolId } from "@composable/types/interfaces";
import { ApiPromise } from "@polkadot/api";

/**
 *Contains handler methods for the pabloPallet Tests.
 * StableSwap ConstantProduct and LiquidityBootstrapping Pools
 */

let constantProductK: bigint;
let baseAmountTotal: bigint;
let quoteAmountTotal: bigint;
let mintedLPTokens: bigint;
baseAmountTotal = BigInt(0);
quoteAmountTotal = BigInt(0);
mintedLPTokens = BigInt(0);

/**
 * Creates Constant Product Pool
 * @param api
 * @param sudoKey
 * @param owner
 * @param baseAssetId
 * @param quoteAssetId
 * @param fee
 * @param ownerFee
 */
export async function createConsProdPool(
  api: ApiPromise,
  sudoKey: KeyringPair,
  owner: KeyringPair,
  baseAssetId: number,
  quoteAssetId: number,
  fee: number,
  baseWeight: number
): Promise<number> {
  const pool = api.createType("PalletPabloPoolInitConfiguration", {
    ConstantProduct: {
      owner: api.createType("AccountId32", owner.address),
      pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: api.createType("u128", baseAssetId),
        quote: api.createType("u128", quoteAssetId)
      }),
      fee: api.createType("Permill", fee),
      baseWeight: api.createType("Permill", baseWeight)
    }
  });
  const {
    data: [resultPoolId]
  } = await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.pablo.PoolCreated.is,
    api.tx.sudo.sudo(api.tx.pablo.create(pool))
  );
  return resultPoolId.toNumber();
}

export async function addFundsToThePool(
  api: ApiPromise,
  poolId: number,
  walletId: KeyringPair,
  baseAmount: bigint,
  quoteAmount: bigint
) /*
    // ToDo (D. Roth): Re- enable after pablo rework!
    : Promise<{
  returnedLPTokens: u128;
  baseAdded: u128;
  quoteAdded: u128;
  walletIdResult: AccountId32;
}> */ {
  const pool = api.createType("u128", poolId);
  const baseAmountParam = api.createType("u128", baseAmount);
  const quoteAmountParam = api.createType("u128", quoteAmount);
  const keepAliveParam = api.createType("bool", true);
  const minMintAmountParam = api.createType("u128", 0);
  // ToDo (D. Roth): Re- enable after pablo rework!
  /*
  const {
    data: [walletIdResult, addedPool, baseAdded, quoteAdded, returnedLPTokens]
  } = await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.pablo.LiquidityAdded.is,
    api.tx.pablo.addLiquidity(pool, baseAmountParam, quoteAmountParam, minMintAmountParam, keepAliveParam)
  );
  mintedLPTokens += BigInt(returnedLPTokens.toString(10));
  baseAmountTotal += BigInt(baseAdded.toString(10));
  quoteAmountTotal += BigInt(quoteAdded.toString(10));
  return { walletIdResult, baseAdded, quoteAdded, returnedLPTokens };*/
}

export async function buyFromPool(
  api: ApiPromise,
  poolId: number,
  walletId: KeyringPair,
  assetId: number,
  amountToBuy: bigint
) /*
    // ToDo (D. Roth): Re- enable after pablo rework!
  : Promise<{
  accountId: AccountId32;
  ownerFee: u128;
  expectedConversion: bigint;
  quoteAmount: u128;
  baseAmount: u128;
}>*/ {
  const poolIdParam = api.createType("u128", poolId);
  const assetIdParam = api.createType("u128", assetId);
  const amountParam = api.createType("u128", amountToBuy);
  const keepAlive = api.createType("bool", true);
  const minMintAmount = api.createType("u128", 0);
  constantProductK = baseAmountTotal * quoteAmountTotal;
  const expectedConversion = constantProductK / (baseAmountTotal - amountToBuy) - quoteAmountTotal;

  // ToDo (D. Roth): Re- enable after pablo rework!
  /*
  const {
    data: [retPoolId, accountId, baseArg, quoteArg, baseAmount, quoteAmount, ownerFee]
  } = await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.pablo.Swapped.is,
    api.tx.pablo.buy(poolIdParam, assetIdParam, amountParam, minMintAmount, keepAlive)
  );
  return { accountId, baseAmount, quoteAmount, expectedConversion, ownerFee: ownerFee.fee };*/
}

export async function sellToPool(
  api: ApiPromise,
  poolId: number,
  walletId: KeyringPair,
  assetId: number,
  amount: bigint
): Promise<AccountId32> {
  const poolIdParam = api.createType("u128", poolId);
  const assetIdParam = api.createType("u128", assetId);
  const amountParam = api.createType("u128", amount);
  const minMintAmount = api.createType("u128", 0);
  const keepAliveParam = api.createType("bool", false);
  const {
    data: [result, ownerAccount, ...rest]
  } = await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.pablo.Swapped.is,
    api.tx.pablo.sell(poolIdParam, assetIdParam, amountParam, minMintAmount, keepAliveParam)
  );
  return ownerAccount;
}

export async function removeLiquidityFromPool(
  api: ApiPromise,
  poolId: number,
  walletId: KeyringPair,
  lpTokens: bigint
) /*ToDo: : Promise<{ resultBase: u128; resultQuote: u128 }> */ {
  const poolIdParam = api.createType("u128", poolId);
  const lpTokenParam = api.createType("u128", lpTokens);
  const minBaseParam = api.createType("u128", 0);
  const minQuoteAmountParam = api.createType("u128", 0);
  // ToDo (D. Roth): Re- enable after pablo rework!
  /*
  const {
    data: [resultPoolId, resultWallet, resultBase, resultQuote, remainingLpTokens]
  } = await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.pablo.LiquidityRemoved.is,
    api.tx.pablo.removeLiquidity(poolIdParam, lpTokenParam, minBaseParam, minQuoteAmountParam)
  );
  return { resultBase, resultQuote };*/
}

export async function swapTokenPairs(
  api: ApiPromise,
  poolId: number,
  wallet: KeyringPair,
  baseAssetId: number,
  quoteAssetId: number,
  quoteAmount: bigint,
  minReceiveAmount = 0
) /*ToDo: : Promise<{ returnedBaseAmount: u128; returnedQuoteAmount: u128 }> */ {
  const poolIdParam = api.createType("u128", poolId);
  const currencyPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
    base: api.createType("CurrencyId", baseAssetId),
    quote: api.createType("CurrencyId", quoteAssetId)
  });
  const quoteAmountParam = api.createType("u128", quoteAmount);
  const minReceiveParam = api.createType("u128", minReceiveAmount);
  const keepAliveParam = api.createType("bool", true);
  // ToDo (D. Roth): Re- enable after pablo rework!
  /*
  const {
    data: [resultPoolId, resultWallet, baseAsset, quoteAsset, returnedBaseAmount, returnedQuoteAmount, feeInfo]
  } = await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.pablo.Swapped.is,
    api.tx.pablo.swap(poolIdParam, currencyPair, quoteAmountParam, minReceiveParam, keepAliveParam)
  );
  return { returnedBaseAmount, returnedQuoteAmount };*/
}

export async function createMultipleCPPools(api: ApiPromise, wallet: KeyringPair) {
  const tx = [];
  for (let i = 0; i < 300; i++) {
    const owner = wallet.derive("/test/ConstantProduct/deriveWallet");
    const pool = api.createType("PalletPabloPoolInitConfiguration", {
      ConstantProduct: {
        owner: api.createType("AccountId32", owner.address),
        pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
          base: api.createType("u128", Math.floor(Math.random() * 10000)),
          quote: api.createType("u128", Math.floor(Math.random() * 10000))
        }),
        fee: api.createType("Permill", Math.floor(Math.random() * 100000)),
        baseWeight: api.createType("Permill", Math.floor(Math.random() * 100000))
      }
    });
    tx.push(api.tx.sudo.sudo(api.tx.pablo.create(pool)));
  }
  await sendWithBatchAndWaitForSuccess(api, wallet, api.events.pablo.PoolCreated.is, tx, false);
}

export async function getUserTokens(api: ApiPromise, walletId: KeyringPair, assetId: number): Promise<u128> {
  const { free } = await api.query.tokens.accounts(walletId.address, assetId);
  return free;
}

export async function getPoolInfo(api: ApiPromise, poolType: string, poolId: number): Promise<{ lpTokenId: number }> {
  const result = await api.query.pablo.pools(api.createType("u128", poolId));
  const pool = result.unwrap();
  const poolS = "as" + poolType;
  const lpTokenId = pool[poolS].lpToken.toNumber();
  return { lpTokenId };
}

export async function rpcPriceFor(
  api: ApiPromise,
  poolId: PalletPabloPoolId,
  baseAssetId: CustomRpcCurrencyId,
  quoteAssetId: CustomRpcCurrencyId
) {
  return await api.rpc.pablo.pricesFor(
    poolId,
    baseAssetId,
    quoteAssetId,
    api.createType("CustomRpcBalance", 10000 /* unit */)
  );
}

export async function getPoolAddress(
  api: ApiPromise,
  poolId: number,
  walletId: KeyringPair,
  baseAmount: bigint,
  quoteAmount: bigint
) /*ToDo: : Promise<string> */ {
  const pool = api.createType("u128", poolId);
  const baseAmountParam = api.createType("u128", baseAmount);
  const quoteAmountParam = api.createType("u128", quoteAmount);
  const keepAliveParam = api.createType("bool", true);
  const minMintAmountParam = api.createType("u128", 0);
  // ToDo (D. Roth): Re- enable after pablo rework!
  /*
  const {
    data: [, AccountId]
  } = await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.tokens.Endowed.is,
    api.tx.pablo.addLiquidity(pool, baseAmountParam, quoteAmountParam, minMintAmountParam, keepAliveParam)
  );
  return AccountId.toString();*/
}

export async function getPoolBalance(api: ApiPromise, poolAddress: string, assetId: number): Promise<u128> {
  const { free } = await api.query.tokens.accounts(poolAddress, assetId);
  return free;
}

export async function transferTokens(
  api: ApiPromise,
  sender: KeyringPair,
  receiver: KeyringPair,
  assetId: number,
  amount: bigint
): Promise<string> {
  const {
    data: [, accountId]
  } = await sendAndWaitForSuccess(
    api,
    sender,
    api.events.tokens.Endowed.is,
    api.tx.assets.transfer(
      api.createType("u128", assetId),
      api.createType("MultiAddress", {
        id: api.createType("AccountId", receiver.address.toString())
      }),
      api.createType("u128", amount),
      api.createType("bool", false)
    )
  );
  return accountId.toString();
}
