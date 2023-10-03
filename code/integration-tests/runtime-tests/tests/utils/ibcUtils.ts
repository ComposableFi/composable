import {ApiPromise} from "@polkadot/api";
import {KeyringPair} from "@polkadot/keyring/types";
import BigNumber from "bignumber.js";
import {
  centauriAddress,
  dotOnCentauri,
  dotOncomposable,
  ksmOncentauri,
  ksmOnComposable,
  picaIdOnComposable,
  picassoFeeAddress,
  usdtOnCentauri,
  usdtOnComposable
} from "./constants";
import child_process from "child_process";
import util from "node:util"
import {IPreBalances, IPreBalancesOnPicasso} from "./types";
import {SubmittableExtrinsic} from "@polkadot/api/types";
import {sendWithBatchAndWaitForSuccess} from "./txClient";

const exec = util.promisify(child_process.exec);

export async function addFeelessChannels(api: ApiPromise, sudoKey: KeyringPair) {
  const txs = [
    api.tx.sudo.sudo(api.tx.ibc.addChannelsToFeelessChannelList(0, 1)),
    api.tx.sudo.sudo(api.tx.ibc.addChannelsToFeelessChannelList(1, 0)),
    api.tx.sudo.sudo(api.tx.ics20Fee.addChannelsToFeelessChannelList(0, 1)),
    api.tx.sudo.sudo(api.tx.ics20Fee.addChannelsToFeelessChannelList(1, 0)),
  ]
  await sendWithBatchAndWaitForSuccess(
    api,
    sudoKey,
    api.events.utility.BatchCompleted.is,
    txs,
    false
  );
}

export function createIbcTransferExtrinsic(
  api: ApiPromise, receiver: string,
  asset: number | string,
  amount: string,
  channelId: number,
  toChannel: string,
  withMemo?: boolean
): SubmittableExtrinsic<"promise"> {
  let params;
  if (toChannel === 'centauri') {
    params = api.createType('PalletIbcTransferParams', {
      to: api.createType('PalletIbcMultiAddress', {
        Raw: api.createType('Bytes', receiver),
      }),
      sourceChannel: api.createType('u64', channelId),
      timeout: api.createType('IbcPrimitivesTimeout', {
        Offset: {
          timestamp: '30000',
          height: api.createType('Option<u64>', 50),
        }
      })
    });
  } else {
    params = api.createType('PalletIbcTransferParams', {
      to: api.createType('PalletIbcMultiAddress', {
        id: api.createType('AccountId32', receiver)
      }),
      sourceChannel: api.createType('u64', channelId),
      timeout: api.createType('IbcPrimitivesTimeout', {
        Offset: {
          timestamp: '30000',
          height: api.createType('Option<u64>', 50),
        }
      })
    });
  }
  const assetId = api.createType('u128', asset);
  const amountTobeSent = api.createType('u128', amount);
  const memo = '';
  return api.tx.ibc.transfer(params, assetId, amountTobeSent, memo);
}

export async function queryAllBalanceOnCentauri(address: string) {
  const {stdout} = await exec(`~/go/bin/centaurid query bank balances ${address} --output json`);
  const formattedBalance = JSON.parse(stdout);
  const balances = parseBalancesFromCentauri(formattedBalance.balances);
  return balances;
}

export async function initiateIbcFromCentauri(sender: string, receiverAddress: string) {
  await
    exec(`~/go/bin/centaurid tx ibc-transfer transfer transfer channel-0 "${receiverAddress}" 100000000000ibc/7880ADBE343B13930823D0A186951351AA0A3FA900F13AEA6A3E0746B02FE521 --from ${sender} --gas-adjustment=1.5 -y;`);
  await waitForSeconds(8);
  await
    exec(`~/go/bin/centaurid tx ibc-transfer transfer transfer channel-0 "${receiverAddress}" 10000000000000ibc/4859E46FF89C3A8D361A2F69CB04A2F5CD9D2CB01171E9D91B9E36405B89318A --from ${sender} --gas-adjustment=1.5 -y;`);
  await waitForSeconds(8);
  await
    exec(`~/go/bin/centaurid tx ibc-transfer transfer transfer channel-0 "${receiverAddress}" 10000000ibc/449BE1BB6F508F0CBE44BCBD810C7A9A1DCEC41B1F4880802A7A79FF4FA8DE53 --from ${sender} --gas-adjustment=1.5 -y;`);
  await waitForSeconds(8);
  await
    exec(`~/go/bin/centaurid tx ibc-transfer transfer transfer channel-0 "${receiverAddress}" 100000000000000ppica --from ${sender} --gas-adjustment=1.5 -y;`);
}

export function parseBalancesFromCentauri(balances: []): IPreBalances {
  const retBalances = {
    "dot": '0',
    "usdt": '0',
    "ksm": '0',
    "pica": '0',
  };
  balances.map((balance: { denom: string, amount: string }) => {
    if (balance.denom === dotOnCentauri) retBalances.dot = balance.amount;
    if (balance.denom === usdtOnCentauri) retBalances.usdt = balance.amount;
    if (balance.denom === ksmOncentauri) retBalances.ksm = balance.amount;
    if (balance.denom === 'ppica') retBalances.pica = balance.amount;
  });
  return retBalances;
}

export async function queryNativeBalance(api: ApiPromise, walletAddress: string) {
  const address = api.createType('AccountId32', walletAddress);
  const {data} = await api.query.system.account(address);
  return data.free.toString();
}

export async function queryAllBalanceOnPicasso(
  api: ApiPromise, assets: number[] | string [], walletAddress: string): Promise<IPreBalancesOnPicasso> {
  const retBalances: IPreBalancesOnPicasso = {"1": '0', "4": '0', "6": '0', "130": '0'};
  await Promise.all(assets.map(async assetId => {
    if (assetId === 1) {
      const balance = await queryNativeBalance(api, walletAddress);
      retBalances[assetId.toString() as keyof IPreBalancesOnPicasso] = balance;
    } else {
      const balance = await queryTokenBalance(api, walletAddress, assetId as string);
      retBalances[assetId.toString() as keyof IPreBalancesOnPicasso] = balance;
    }
  }));
  return retBalances;
}

export async function queryTotalIssuanceOfTokensOnCentauri() {
  const {stdout} = await exec(' ~/go/bin/centaurid query bank total --output json');
  const formattedBalance = JSON.parse(stdout);
  const totalIssuances = parseBalancesFromCentauri(formattedBalance.supply);
  return totalIssuances;
}

export async function queryTotalIssuanceOfTokensOnPicasso(api: ApiPromise) {
  const totalIssuance = {"pica": '0', "ksm": '0', "dot": '0', "usdt": '0'};
  [totalIssuance.pica, totalIssuance.ksm, totalIssuance.dot, totalIssuance.usdt] = await Promise.all([
    (await api.query.balances.totalIssuance()).toString(),
    (await api.query.tokens.totalIssuance(4)).toString(),
    (await api.query.tokens.totalIssuance(6)).toString(),
    (await api.query.tokens.totalIssuance(130)).toString(),
  ]);
  return totalIssuance;
}

export async function queryTotalIssuanceOncomposable(api: ApiPromise) {
  const totalIssuance = {"pica": '0', "ksm": '0', "dot": '0', "usdt": '0'};
  [totalIssuance.pica, totalIssuance.ksm, totalIssuance.dot, totalIssuance.usdt] = await Promise.all([
    (await api.query.tokens.totalIssuance(picaIdOnComposable)).toString(),
    (await api.query.tokens.totalIssuance(ksmOnComposable)).toString(),
    (await api.query.tokens.totalIssuance(dotOncomposable)).toString(),
    (await api.query.tokens.totalIssuance(usdtOnComposable)).toString(),
  ]);
  return totalIssuance;
}

export function calculateExpectedDifference(fee: number, ...transferAmounts: string[]) {
  const diffObj = {};
  const assets = ['ksm', 'usdt', 'pica', 'dot'];
  transferAmounts.map((transferAmount, index) => {
    const diff = (new BigNumber(transferAmount).minus(new BigNumber(transferAmount).multipliedBy(fee))).toString();
    Object.assign(diffObj, {[assets[index]]: diff});
  })
  return diffObj;
}

export function getActualandExpectedDiff(afterBalanceToken: string, preBalanceToken: string, transferAmount: string) {
  const expectedDiff = new BigNumber(transferAmount).minus(new BigNumber(transferAmount).multipliedBy(0.004));
  const actualDiff = new BigNumber(afterBalanceToken).minus(new BigNumber(preBalanceToken));
  return {expectedDiff, actualDiff};
}

export async function getStateOfFundsForPicassoToCentauri(api: ApiPromise, assets: number[], wallet: KeyringPair) {
  const preTotalIssuanceOnCentauri = await queryTotalIssuanceOfTokensOnCentauri();
  const preTotalIssuanceOnPicasso = await queryTotalIssuanceOfTokensOnPicasso(api);
  const preBalanceOnCentauri = await queryAllBalanceOnCentauri(centauriAddress);
  const preBalanceOnPicasso = await queryAllBalanceOnPicasso(api, assets, wallet.address.toString());
  const feeAccountPreBalance = await queryAllBalanceOnPicasso(api, assets, picassoFeeAddress);
  return {
    preTotalIssuanceOnCentauri,
    preTotalIssuanceOnPicasso,
    preBalanceOnCentauri,
    preBalanceOnPicasso,
    feeAccountPreBalance
  };
}

export async function getStateOfFundsForPicassoToComposable(
  composableApi: ApiPromise,
  picassoApi: ApiPromise,
  wallet: KeyringPair
) {
  const preTotalIssuanceOnComposable = await queryTotalIssuanceOncomposable(composableApi);
  const preTotalIssuanceOnPicasso = await queryTotalIssuanceOfTokensOnPicasso(picassoApi);
  const preBalancesOnComposable = await queryAllBalanceOnComposable(
    composableApi,
    [picaIdOnComposable, ksmOnComposable, dotOncomposable, usdtOnComposable],
    wallet.address);
  const preBalanceOnPicasso = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], wallet.address);
  const feeAccountPreBalance = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], picassoFeeAddress);
  return {
    preTotalIssuanceOnComposable,
    preTotalIssuanceOnPicasso,
    preBalancesOnComposable,
    preBalanceOnPicasso,
    feeAccountPreBalance
  };
}

export async function queryAllBalanceOnComposable(api: ApiPromise, assets: string[], walletAddress: string) {
  const retObject = {'1': '', '4': '0', '6': '0', '130': '0'};
  const objKeys = Object.keys(retObject);
  await Promise.all(assets.map(async (assetId, index) => {
    const balance = await api.query.tokens.accounts(walletAddress, assetId);
    retObject[objKeys[index] as keyof IPreBalancesOnPicasso] = balance.free.toString();
  }));
  return retObject;
}

export async function queryTokenBalance(api: ApiPromise, walletAdress: string, assetId: string) {
  const {free} = await api.query.tokens.accounts(walletAdress, assetId);
  return free.toString();
}

export function createIbcTxsForBatch(
  api: ApiPromise, amounts: string[], assets: number[] | string [], receiver: string, toChain: string): SubmittableExtrinsic<"promise">[] {
  let channelId: number;
  if (toChain === 'centauri') {
    channelId = 0;
  } else if (toChain === 'picasso') {
    channelId = 0;
  } else {
    channelId = 1;
  }
  const txsArr = assets.map((asset, index) => {
    return createIbcTransferExtrinsic(api, receiver, asset, amounts[index], channelId, toChain);
  })
  return txsArr;
}

export async function waitForSeconds(seconds: number) {
  return new Promise(resolve => {
    setTimeout(resolve, seconds * 1000)
  })
}