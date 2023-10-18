import {ApiPromise} from "@polkadot/api";
import {KeyringPair} from "@polkadot/keyring/types";
import child_process from "child_process";
import util from "node:util"
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

export async function waitForSeconds(seconds: number) {
  return new Promise(resolve => {
    setTimeout(resolve, seconds * 1000)
  })
}

export async function queryTotalIssuanceOfTokenOnOsmosis(denom: string) {
  const {stdout} = await exec(`binaries/osmosisd query bank total --denom ${denom} --output json`);
  const formattedBalance = JSON.parse(stdout);
  return formattedBalance.amount;
}

export async function queryTotalIssuanceOfTokenOnCentauri(denom: string){
  const {stdout} = await exec(`binaries/centaurid query bank total --denom ${denom} --output json`);
  const formattedBalance = JSON.parse(stdout);
  return formattedBalance.amount;
}

export async function queryBalanceOnOsmosis(address: string, denom: string) {
  const {stdout} = await exec(`binaries/osmosisd query bank balances ${address} --denom ${denom} --output json`);
  const formattedBalance = JSON.parse(stdout);
  return formattedBalance.amount;
}

export async function queryBalanceOnCentauri(address: string, denom: string){
  const {stdout} = await exec(`binaries/centaurid query bank balances ${address} --denom ${denom} --output json`);
  const formattedBalance = JSON.parse(stdout);
  return formattedBalance.amount;
}

export async function queryNativeBalance(api: ApiPromise, walletAddress: string) {
  const address = api.createType('AccountId32', walletAddress);
  const {data} = await api.query.system.account(address);
  return data.free.toString();
}

export async function queryTokenBalance(api: ApiPromise, walletAdress: string, assetId: string) {
  const {free} = await api.query.tokens.accounts(walletAdress, assetId);
  return free.toString();
}


