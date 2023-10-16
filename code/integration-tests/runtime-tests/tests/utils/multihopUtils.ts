import {Asset} from "./types";
import BigNumber from "bignumber.js";
import {ApiPromise} from "@polkadot/api";
import {
  queryBalanceOnCentauri,
  queryBalanceOnOsmosis,
  queryNativeBalance,
  queryTokenBalance,
  queryTotalIssuanceOfTokenOnCentauri,
  queryTotalIssuanceOnOsmosis,
  waitForSeconds
} from "./ibcUtils";
import assetData from "./assets.json";
import {sendAndWaitForSuccess} from "./txClient";
import {KeyringPair} from "@polkadot/keyring/types";
import {AnyTuple, IEvent} from "@polkadot/types/types";
import {
  centauriAddress,
  osmosisAddress,
  picassoCentauriEscrowAddress,
  picassoEscrowAddress,
  picassoFeeAddress
} from "./constants";
import {bech32} from "bech32";
import util from "node:util";
import child_process from "child_process";

const exec = util.promisify(child_process.exec);


export function createAssets(chain: string) {
  const assets: Asset[] = [];
  Object.entries(assetData).map(([assetName, assetDetails]) => {
    let asset: Asset;
    if (assetName === 'pica' && chain === 'picasso') {
      asset = createAsset(assetDetails.id[chain], assetDetails.decimals, chain, true);
    } else {
      // @ts-ignore
      asset = createAsset(assetDetails.id[chain], assetDetails.decimals, chain, false)
    }
    assets.push(asset);
  })
  return assets;
}

export function createAsset(assetId: string, decimals: number, chain: string, native: boolean = false): Asset {
  return {
    decimals: decimals,
    id: assetId,
    balance: new Map(),
    isNative: native,
    chain: chain,
  }
}

/**
 *
 * @param asset An array or a single asset
 * @param addresses Array of wallet addresses
 * @param ecosystem either dotsama or cosmos
 * @param api apiPromise for requested chain. Only available for dotsama
 */
export async function getBalance(asset: Asset | Asset[], addresses: string[], ecosystem: string, api?: ApiPromise) {
  if (Array.isArray(asset)) {
    await Promise.all(asset.map(async specAsset => {
      await Promise.all(addresses.map(async (address) => {
        if (ecosystem === 'dotsama' && api !== undefined) {
          if (specAsset.isNative) {
            specAsset.balance.set(address, new BigNumber(await queryNativeBalance(api, address)));
          } else {
            specAsset.balance.set(address, new BigNumber(await queryTokenBalance(api, address, specAsset.id)));
          }
        } else if (ecosystem === 'cosmos') {
          if (specAsset.chain === 'centauri') {
            specAsset.balance.set(address, new BigNumber(await queryBalanceOnCentauri(address, specAsset.id)));
          } else {
            specAsset.balance.set(address, new BigNumber(await queryBalanceOnOsmosis(address, specAsset.id)));
          }
        }
      }));
    }));
  } else {
    await Promise.all(addresses.map(async address => {
      if (ecosystem === 'dotsama' && api !== undefined) {
        if (asset.isNative) {
          asset.balance.set(address, new BigNumber(await queryNativeBalance(api, address)));
        } else {
          asset.balance.set(address, new BigNumber(await queryTokenBalance(api, address, asset.id)));
        }
      } else if (ecosystem === 'cosmos') {
        if (asset.chain === 'centauri') {
          asset.balance.set(address, new BigNumber(await queryBalanceOnCentauri(address, asset.id)));
        } else {
          asset.balance.set(address, new BigNumber(await queryBalanceOnOsmosis(address, asset.id)));
        }
      }
    }))
  }
}

export async function getTotalIssuance(asset: Asset | Asset[], ecosystem: string, api?: ApiPromise) {
  if (Array.isArray(asset)) {
    await Promise.all(asset.map(async specAsset => {
      if (ecosystem === 'dotsama' && api !== undefined) {
        if (specAsset.isNative) {
          specAsset.totalIssuance = new BigNumber((await api.query.balances.totalIssuance()).toString());
        } else {
          specAsset.totalIssuance = new BigNumber((await api.query.tokens.totalIssuance(specAsset.id)).toString());
        }
      } else if (ecosystem === 'cosmos') {
        if (specAsset.chain === 'centauri') {
          specAsset.totalIssuance = new BigNumber(await queryTotalIssuanceOfTokenOnCentauri(specAsset.id));
        } else {
          specAsset.totalIssuance = new BigNumber(await queryTotalIssuanceOnOsmosis(specAsset.id));
        }
      }
    }));
  } else {
    if (ecosystem === 'dotsama' && api !== undefined) {
      if (asset.isNative) {
        asset.totalIssuance = new BigNumber((await api.query.balances.totalIssuance()).toString());
      } else {
        asset.totalIssuance = new BigNumber((await api.query.tokens.totalIssuance(asset.id)).toString())
      }
    } else if (ecosystem === 'cosmos') {
      if (asset.chain === 'centauri') {
        asset.totalIssuance = new BigNumber(await queryTotalIssuanceOfTokenOnCentauri(asset.id));
      } else {
        asset.totalIssuance = new BigNumber(await queryTotalIssuanceOnOsmosis(asset.id));
      }
    }
  }
}

export function mapRoutesAndChannels(startChain: string, endChain: string) {
  const channels = returnChannels();
  // @ts-ignore
  const chann = channels.find(chains => {
    if (chains.from === startChain && chains.to === endChain) {
      return chains;
    }
  })
  if (chann) return chann;
  throw new Error('Channel not found');
}

function isSubstrate(startChain: string, endChain: string) {
  if (startChain === 'polkadot' || endChain === 'polkadot') return false;
  if (startChain === 'kusama' || endChain === 'kusama') return false;
  if (endChain === 'centauri' || endChain === 'osmo') return false;
  return true;
}

function isCosmos(startChain: string, endChain: string) {
  if (endChain === 'centauri' || endChain === 'osmo') return true;
  return false;
}

function returnChannels() {
  return [
    {from: 'picasso', to: 'composable', channelId: 1},
    {from: 'composable', to: 'picasso', channelId: 0},
    {from: 'picasso', to: 'centauri', channelId: 0},
    {from: 'centauri', to: 'picasso', channelId: 0},
    {from: 'centauri', to: 'osmo', channelId: 1}
  ]
}

export async function createRoute(api: ApiPromise, sudoKey: KeyringPair, routeId: number, channels: string [][]) {
  let routeOrder = 0;
  let chainOrder = 0;
  const hops = channels.map(channel => {
    const fromChain = channel[0];
    const toChain = channel[1];
    chainOrder++;
    routeOrder++
    let chainId = api.createType('u32', chainOrder);
    let route = api.createType('u8', routeOrder);
    let channelId;
    let paraId = api.createType('Option<u64>', null);
    let name = api.createType('Bytes', toChain);
    let timestamp;
    let height;
    let retries;
    let timeout;
    let chainHop;
    if (isSubstrate(fromChain, toChain)) {
      channelId = api.createType('u64', (mapRoutesAndChannels(fromChain, toChain)).channelId);
      timestamp = api.createType('Option<u64>', 10000);
      height = api.createType(' Option<u64>', 1000);
      retries = api.createType('u8', 0);
      timeout = api.createType('Option<u64>', 6000000000000);
      chainHop = api.createType('ComposableTraitsXcmMemoChainHop', 'SubstrateIbc');
    } else if (isCosmos(fromChain, toChain)) {
      channelId = api.createType('u64', (mapRoutesAndChannels(fromChain, toChain)).channelId);
      timestamp = api.createType('Option<u64>', 600);
      height = api.createType(' Option<u64>', 600);
      retries = api.createType('u8', 0);
      timeout = api.createType('Option<u64>', 0);
      chainHop = api.createType('ComposableTraitsXcmMemoChainHop', 'CosmosIbc');
    } else {
      chainHop = api.createType('ComposableTraitsXcmMemoChainHop', 'Xcm');
    }
    const xcmChanInfo = api.createType('ComposableTraitsXcmMemoChainInfo', {
      chainId,
      route,
      channelId,
      timestamp,
      height,
      retries,
      timeout,
      chainHop,
      paraId
    });
    return api.createType('(ComposableTraitsXcmMemoChainInfo,Bytes)', [xcmChanInfo, name]);
  });
  const routeIdParam = api.createType('u128', routeId);
  await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.palletMultihopXcmIbc.addRoute(routeIdParam, hops)),
    false
  )
}

export async function initiateXcmTransfer(
  api: ApiPromise,
  parachainId: number,
  routeId: number,
  wallet: KeyringPair,
  nbOfHops: number,
  transferAmount: BigNumber,
  toFail: boolean,
  toCosmos = false
) {
  const dest = setDestForXcmTransfer(api, parachainId);
  let beneficiary;
  if (!toFail && toCosmos) {
    beneficiary = setBeneficiary(api, nbOfHops, routeId, wallet, toCosmos);
  } else if (toFail && toCosmos) {
    beneficiary = setBeneficiary(api, nbOfHops, routeId, wallet, false);
  } else if (!toFail && !toCosmos) {
    beneficiary = setBeneficiary(api, nbOfHops, routeId, wallet, false);
  } else {
    beneficiary = setBeneficiary(api, nbOfHops, routeId, wallet, true);
  }
  const assets = setNativeAssetForXcm(api, transferAmount);
  const feeAssetItem = api.createType('u32', 0);
  await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.system.ExtrinsicSuccess.is,
    api.tx.xcmPallet.reserveTransferAssets(dest, beneficiary, assets, feeAssetItem),
    false
  )
}

export function setBeneficiary(api: ApiPromise, nbOfHops: number, routeId: number, wallet: KeyringPair, toCosmos: boolean) {
  const networkHops = [];
  for (let i = 0; i < nbOfHops; i++) {
    const type = api.createType("XcmV2Junction", {
      AccountId32: {
        network: api.createType("XcmV2NetworkId", "Any"),
        id: wallet.publicKey,
      }
    });
    networkHops.push(type);
  }
  if (toCosmos) {
    networkHops[1] = api.createType("XcmV2Junction", {
      AccountId32: {
        network: api.createType("XcmV2NetworkId", "Any"),
        id: api.createType('AccountId32', bech32.decode(centauriAddress).words),
      }
    });
    networkHops[2] = api.createType("XcmV2Junction", {
      AccountId32: {
        network: api.createType("XcmV2NetworkId", "Any"),
        id: api.createType('AccountId32', bech32.decode(osmosisAddress).words),
      }
    });
  }

  if (nbOfHops === 2) {
    return api.createType('XcmVersionedMultiLocation', {
      V2: api.createType('XcmV2MultiLocation', {
        parents: api.createType('u8', 0),
        interior: api.createType('XcmV2MultilocationJunctions', {
          X4: [
            api.createType("XcmV2Junction", {
              PalletInstance: api.createType("u8", 192)
            }),
            api.createType('XcmV2Junction', {
              GeneralIndex: api.createType('Compact<u128>', routeId)
            }),
            ...networkHops,
          ]
        })
      })
    })
  } else if (nbOfHops === 3) {
    return api.createType('XcmVersionedMultiLocation', {
      V2: api.createType('XcmV2MultiLocation', {
        parents: api.createType('u8', 0),
        interior: api.createType('XcmV2MultilocationJunctions', {
          X5: [
            api.createType("XcmV2Junction", {
              PalletInstance: api.createType("u8", 192)
            }),
            api.createType('XcmV2Junction', {
              GeneralIndex: api.createType('Compact<u128>', routeId)
            }),
            ...networkHops,
          ]
        })
      })
    })
  }
  throw new Error('Modify path for more hops');
}

export async function waitForEvent<T extends AnyTuple>(api: ApiPromise, filterCall: (event: IEvent<AnyTuple>) => event is IEvent<T>) {
  return new Promise(async (resolve, reject) => {
    let index = 0;
    const unsubscribe = await api.query.system.events((events) => {
      index++;
      return events.forEach((record) => {
        const {event} = record;
        if (filterCall(event)) {
          unsubscribe();
          resolve(event);
        } else if (index > 70) {
          unsubscribe();
          reject('waited for 70 blocks');
        }
      });
    });
  })
}

export function toNumber(bignumberAmount: BigNumber, decimals: number) {
  return (bignumberAmount.dividedBy(10 ** decimals)).toNumber();
}

export async function getBalanceAndIssuanceStats(
  asset: Asset,
  walletAddress: string,
  api: ApiPromise,
  toChain: string
) {
  let escrowAddress: string;
  if (toChain === 'composable') {
    escrowAddress = picassoEscrowAddress;
  } else {
    escrowAddress = picassoCentauriEscrowAddress;
  }
  await getBalance(asset, [picassoFeeAddress, escrowAddress, walletAddress], 'dotsama', api);
  const preFeeAddressBalance = asset.balance.get(picassoFeeAddress) as BigNumber;
  await getTotalIssuance(asset, 'dotsama', api);
  const preTotalIssuance = asset.totalIssuance as BigNumber;
  const preEscrowAddressBalance = asset.balance.get(escrowAddress) as BigNumber;
  return [preFeeAddressBalance, preTotalIssuance, preEscrowAddressBalance];
}

/**
 * Polls centauri every 8 seconds to check if the centauri-osmosis opens
 */
export async function waitForChannelsToOpen(expectedChannelCount: number, targetChain: string, api?: ApiPromise) {
  if (targetChain === 'centauri') {
    await waitForChannelsOnCentauri(expectedChannelCount);
  } else {
    if (!api) {
      throw new Error('for chains other than centauri, you need to pass api promise');
    }
    await waitForChannelsOnPicasso(expectedChannelCount, api);
  }
}

export async function waitForChannelsOnCentauri(expectedChannelCount: number) {
  let {stdout} = await exec(`~/go/bin/centaurid query ibc channel channels --output json`);
  let parsed = JSON.parse(stdout);
  let channelsLength = parsed.channels.length;
  let index = 0;
  while (channelsLength < expectedChannelCount && index < 50) {
    ({stdout} = await exec(`~/go/bin/centaurid query ibc channel channels --output json`));
    parsed = JSON.parse(stdout);
    channelsLength = parsed.channels.length;
    index++;
    console.log('waiting for channels on osmosis to open');
    await waitForSeconds(8);
  }
}

export async function waitForChannelsOnPicasso(expectedChannelCount: number, api: ApiPromise) {
  let channelCount = await api.query.ibc.channelIds();
  let index = 0;
  while (channelCount.length < expectedChannelCount && index < 50) {
    channelCount = await api.query.ibc.channelIds();
    console.log('waiting for channels to Composable');
    await waitForSeconds(12);
  }
}

export function setNativeAssetForXcm(api: ApiPromise, transferAmount: BigNumber) {
  return api.createType("XcmVersionedMultiAssets", {
    V2: api.createType("XcmV2MultiassetMultiAssets", [
      api.createType('XcmV2MultiAsset', {
        id: api.createType("XcmV2MultiassetAssetId", {
          Concrete: api.createType("XcmV2MultiLocation", {
            parents: api.createType("u8", 0),
            interior: api.createType("XcmV2MultilocationJunctions", "Here")
          })
        }),
        fun: api.createType("XcmV2MultiassetFungibility", {
          Fungible: api.createType("Compact<u128>", transferAmount.toString())
        })
      })
    ])
  });
}

export function setDestForXcmTransfer(api: ApiPromise, parachainId: number) {
  return api.createType("XcmVersionedMultiLocation", {
    V2: api.createType("XcmV2MultiLocation", {
      parents: api.createType("u8", 0),
      interior: api.createType("XcmV2MultilocationJunctions", {
        X1: api.createType("XcmV2Junction", {
          Parachain: api.createType("Compact<u32>", parachainId)
        }),
      })
    })
  });
}

export async function getNextSequenceForIbc(api: ApiPromise) {
  const nextHeader = await api.query.ibc.sequenceFee.entries();
  return nextHeader.length;
}