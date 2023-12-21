import {KeyringPair} from "@polkadot/keyring/types";
import {
  createAsset,
  createAssets,
  createRoute,
  getAddressessOnOtherChains,
  getAddressessOnPicasso,
  getBalance,
  getBalanceAndIssuanceStats,
  getEndpoints,
  getNextSequenceForIbc,
  getTotalIssuance,
  initiateXcmTransfer,
  toNumber,
  waitForChannelsToOpen,
  waitForEvent
} from "./utils/multihopUtils";
import {getWallets, initializeApi} from "./utils/apiClient";
import {ApiPromise} from "@polkadot/api";
import {Asset} from "./utils/types";
import {waitForBlocks} from "./utils/txClient";
import BigNumber from "bignumber.js";
import {mintAssetsOnRelays, mintAssetsToWallets} from "./utils/mintingHelper";
import {addFeelessChannels, waitForSeconds} from "./utils/ibcUtils";

const chai = require('chai');
const expect = require('chai').expect;
chai.use(require('chai-bignumber')());
BigNumber.config({ROUNDING_MODE: BigNumber.ROUND_DOWN})


describe('MultiHop Tests', function () {
  this.timeout(30 * 60 * 1000);
  const picassoAssets = createAssets('picasso');
  const composableAssets = createAssets('composable');
  let centauriAssets = createAssets('centauri');
  let osmosisAssets = createAssets('osmosis');
  const ksmTransferAmount = new BigNumber('10000000000000');
  const ksmOnKusama = createAsset("1", 12, 'kusama', 'ksm', true);
  const {substrateEscrowAddress, cosmosEscrowAddress, feeAddress} = getAddressessOnPicasso();
  const {centauriAddress, osmosisAddress} = getAddressessOnOtherChains();
  const {picassoEndpoint, composableEndpoint, kusamaEndpoint} = getEndpoints();
  let ibcSentAmount: BigNumber;
  let sudoKey: KeyringPair;
  let testWallet: KeyringPair;
  let picassoApi: ApiPromise;
  let composableApi: ApiPromise;
  let kusamaApi: ApiPromise;
  let ibcEvent: any;

  before('Mint assets and add routes', async () => {
    picassoApi = await initializeApi(picassoEndpoint);
    composableApi = await initializeApi(composableEndpoint);
    kusamaApi = await initializeApi(kusamaEndpoint);
    ({sudoKey, testWallet} = getWallets('multihop'));
    await Promise.all([
      mintAssetsToWallets(picassoApi, testWallet, sudoKey, [1], '10000000000000000', 'picasso'),
      mintAssetsOnRelays([kusamaApi], sudoKey, testWallet.address),
    ]);
    await createRoute(picassoApi, sudoKey, 2, [['picasso', 'composable'], ['composable', 'picasso']]);
    await createRoute(picassoApi, sudoKey, 3, [['picasso', 'centauri']]);
    await Promise.all([
      getBalance(picassoAssets, [testWallet.address, feeAddress, substrateEscrowAddress, cosmosEscrowAddress], 'dotsama', picassoApi),
      getBalance(composableAssets, [testWallet.address], 'dotsama', composableApi),
      getBalance(ksmOnKusama, [testWallet.address], 'dotsama', kusamaApi),
      getBalance(centauriAssets, [centauriAddress], 'cosmos'),
      getBalance(osmosisAssets, [osmosisAddress], 'cosmos'),
    ]);
    await addFeelessChannels(composableApi, sudoKey);
  })

  before('Wait for channel to open on centauri to osmosis', async () => {
    await waitForChannelsToOpen(1, 'centauri');
  });

  it('Initiates a transfer for kusama => picasso => centauri', async () => {
    console.log('TESTS Transfer initiated');
    const preSequence = await getNextSequenceForIbc(picassoApi);
    const ksm = picassoAssets.find(asset => asset.symbol === 'ksm') as Asset;
    await waitForBlocks(picassoApi, 3);
    console.log('TESTS Waited for 2 blokcs');
    const [preFeeAddressBalance, preTotalIssuance, preEscrowAddressBalance] =
      await getBalanceAndIssuanceStats(
        ksm,
        testWallet.address,
        feeAddress,
        cosmosEscrowAddress,
        picassoApi);
    console.log('TESTS xcm initiated');
    ([ibcEvent,] = await Promise.all([
        waitForEvent(picassoApi, picassoApi.events.palletMultihopXcmIbc.SuccessXcmToIbc.is),
        initiateXcmTransfer(
          kusamaApi,
          2087,
          3,
          testWallet,
          2,
          ksmTransferAmount,
          false,
          true,
          centauriAddress,
          osmosisAddress),
      ]
    ));
    console.log('TESTS Events and got');
    await waitForBlocks(picassoApi, 2);
    const {data: [_origin, _to, amount, _assetId, _memo]} = ibcEvent;
    ibcSentAmount = new BigNumber((amount.toString().replaceAll(',', '')));
    const nextsequence = await getNextSequenceForIbc(picassoApi);
    const [afterFeeAddressBalance, afterTotalIssuance, afterEscrowAddressBalance] =
      await getBalanceAndIssuanceStats(
        ksm,
        testWallet.address,
        feeAddress,
        cosmosEscrowAddress,
        picassoApi);
    const feeCharged = new BigNumber((ibcSentAmount.multipliedBy(0.005)).toFixed(0));
    ibcSentAmount = ibcSentAmount.minus(feeCharged);
    const diffInTotalIssuance = afterTotalIssuance.minus(preTotalIssuance);
    expect(diffInTotalIssuance.toString()).to.be.eq(ksmTransferAmount.toString());
    expect(afterEscrowAddressBalance.minus(preEscrowAddressBalance)).to.be.bignumber.eq(ibcSentAmount);
    expect(afterFeeAddressBalance).to.be.bignumber.eq(preFeeAddressBalance.plus(feeCharged));
    expect(nextsequence).to.be.eq(preSequence + 1);
  });

  it('Waits for funds on centauri', async () => {
    const ksmOnCent = centauriAssets.find(asset => asset.symbol === 'ksm') as Asset;
    const ksmPreBal = ksmOnCent?.balance.get(centauriAddress) as BigNumber;
    await getTotalIssuance(ksmOnCent, 'cosmos');
    const preTotalIssuance = ksmOnCent.totalIssuance as BigNumber;
    let afterTotalIssuance = ksmOnCent.totalIssuance as BigNumber;
    while (preTotalIssuance.toString() === afterTotalIssuance.toString()) {
      await waitForSeconds(8);
      await getTotalIssuance(ksmOnCent, 'cosmos');
      afterTotalIssuance = ksmOnCent.totalIssuance as BigNumber;
    }
    await getBalance(ksmOnCent, [centauriAddress], 'cosmos');
    const ksmAfterBal = ksmOnCent.balance.get(centauriAddress);
    //validate that user balance increases
    expect(ksmAfterBal).to.be.bignumber.eq(ksmPreBal.plus(ibcSentAmount));
    //validate that total issuance increases
    expect(afterTotalIssuance).to.be.bignumber.eq(preTotalIssuance.plus(ibcSentAmount));
  });

  it('waits for funds on osmosis', async () => {
    const ksmOnOsmo = osmosisAssets.find(asset => asset.symbol === 'ksm') as Asset;
    const ksmPreBal = ksmOnOsmo?.balance.get(osmosisAddress) as BigNumber;
    await getTotalIssuance(ksmOnOsmo, 'cosmos');
    const preTotalIssuance = ksmOnOsmo.totalIssuance as BigNumber;
    let ksmAfterBal = ksmOnOsmo.balance.get(osmosisAddress) as BigNumber;
    while (ksmAfterBal.toString() === ksmPreBal.toString()) {
      await waitForSeconds(8);
      await getBalance(osmosisAssets, [osmosisAddress], 'cosmos');
      ksmAfterBal = ksmOnOsmo.balance.get(osmosisAddress) as BigNumber;
    }
    await getTotalIssuance(ksmOnOsmo, 'cosmos');
    const afterTotalIssuance = ksmOnOsmo.totalIssuance as BigNumber;
    expect(ksmAfterBal).to.be.bignumber.eq(ksmPreBal.plus(ibcSentAmount));
    expect(afterTotalIssuance).to.be.bignumber.eq(preTotalIssuance.plus(ibcSentAmount));
  });

  it('Tx fails at the first hop', async () => {
    const route = 3;
    const preSequence = await getNextSequenceForIbc(picassoApi);
    const ksm = picassoAssets.find(asset => asset.id === "4") as Asset;
    const [preFeeAddressBalance, preTotalIssuance, preEscrowAddressBalance] =
      await getBalanceAndIssuanceStats(
        ksm,
        testWallet.address,
        feeAddress,
        cosmosEscrowAddress,
        picassoApi);
    const userPreBalance = ksm.balance.get(testWallet.address) as BigNumber;
    ([ibcEvent,] = await Promise.all([
        waitForEvent(picassoApi, picassoApi.events.palletMultihopXcmIbc.FailedCallback.is),
        initiateXcmTransfer(
          kusamaApi,
          2087,
          route,
          testWallet,
          3,
          ksmTransferAmount,
          true,
          true,
          centauriAddress,
          osmosisAddress),
      ]
    ))
    ;
    await waitForBlocks(picassoApi, 1);
    const {data: [_originAddress, routeId, _reason]} = ibcEvent;
    expect(routeId.toString()).to.be.eq(route.toString())
    const [afterFeeAddressBalance, afterTotalIssuance, afterEscrowAddressBalance] =
      await getBalanceAndIssuanceStats(
        ksm,
        testWallet.address,
        feeAddress,
        cosmosEscrowAddress,
        picassoApi);
    const userAfterBalance = ksm.balance.get(testWallet.address) as BigNumber;
    const diffInTotalIssuance = afterTotalIssuance.minus(preTotalIssuance);
    const afterSequence = await getNextSequenceForIbc(picassoApi);
    expect(diffInTotalIssuance.toString()).to.be.eq(ksmTransferAmount.toString());
    expect(afterEscrowAddressBalance).to.be.bignumber.eq(preEscrowAddressBalance);
    expect(afterFeeAddressBalance).to.be.bignumber.eq(preFeeAddressBalance);
    expect(userAfterBalance).to.be.bignumber.gt(userPreBalance);
    expect(afterSequence).to.be.eq(preSequence);
  });

  it('Wait for channel to open on picasso to composable', async () => {
    await waitForChannelsToOpen(2, 'picasso', picassoApi);
  });

  it('Initiate a transfer for kusama => picasso => composable => picasso', async () => {
    const preSequence = await getNextSequenceForIbc(picassoApi);
    const ksm = picassoAssets.find(asset => asset.symbol === 'ksm') as Asset;
    const [preFeeAddressBalance, preTotalIssuance, preEscrowAddressBalance] =
      await getBalanceAndIssuanceStats(
        ksm,
        testWallet.address,
        feeAddress,
        substrateEscrowAddress,
        picassoApi);
    ([ibcEvent,] = await Promise.all([
        waitForEvent(picassoApi, picassoApi.events.palletMultihopXcmIbc.SuccessXcmToIbc.is),
        initiateXcmTransfer(
          kusamaApi,
          2087,
          2,
          testWallet,
          3,
          ksmTransferAmount,
          false,
          false,
          centauriAddress,
          osmosisAddress),
      ]
    ));
    const {data: [_origin, _to, amount, _assetId, _memo]} = ibcEvent;
    let sentAmount = new BigNumber((amount.toString().replaceAll(',', '')));
    await waitForBlocks(picassoApi, 2);
    const [afterFeeAddressBalance, afterTotalIssuance, afterEscrowAddressBalance] =
      await getBalanceAndIssuanceStats(
        ksm,
        testWallet.address,
        feeAddress,
        substrateEscrowAddress,
        picassoApi);
    const diffInTotalIssuance = afterTotalIssuance.minus(preTotalIssuance);
    const afterSequence = await getNextSequenceForIbc(picassoApi);
    const feeCharged = new BigNumber((sentAmount.multipliedBy(0.004)).toFixed(0));
    sentAmount = sentAmount.minus(feeCharged);
    expect(diffInTotalIssuance.toString()).to.be.eq(ksmTransferAmount.toString());
    expect(afterEscrowAddressBalance.minus(preEscrowAddressBalance)).to.be.bignumber.eq(sentAmount);
    expect(afterFeeAddressBalance).to.be.bignumber.gt(preFeeAddressBalance);
    expect(afterSequence).to.be.eq(preSequence + 1);
  });

  it('Validate multihop event', () => {
    const id = picassoApi.registry.createType("AccountId", testWallet.address);
    const {data: [origin, to, _amount, assetId, memo]} = ibcEvent;
    expect(origin.toHex()).to.be.eq(id.toHex());
    expect(to.toHex()).to.be.eq(id.toHex());
    expect(assetId.toString()).to.be.eq('4');
    const memoParsed = JSON.parse(memo.toString());
    expect(memoParsed.forward.receiver).to.be.eq(id.toHex());
    expect(memoParsed.forward.channel).to.be.eq('channel-0');
  });

  it('Wait for funds to arrive at composable', async () => {
    const ksmOnComp = composableAssets.find(asset => asset.symbol === 'ksm') as Asset;
    const ksmPreBal = ksmOnComp?.balance.get(testWallet.address) as BigNumber;
    await getTotalIssuance(ksmOnComp, 'dotsama', composableApi);
    const preTotalIssuance = ksmOnComp.totalIssuance as BigNumber;
    ibcEvent = await waitForEvent(composableApi, composableApi.events.ibc.ExecuteMemoIbcTokenTransferSuccess.is);
    await waitForBlocks(composableApi, 1);
    await getBalance(ksmOnComp, [testWallet.address], 'dotsama', composableApi);
    await getTotalIssuance(ksmOnComp, 'dotsama', composableApi);
    const afterTotalIssuance = ksmOnComp.totalIssuance as BigNumber;
    let ksmAfterBal = ksmOnComp.balance.get(testWallet.address) as BigNumber;
    expect(ksmAfterBal).to.be.bignumber.eq(ksmPreBal);
    expect(afterTotalIssuance).to.be.bignumber.eq(preTotalIssuance);
    const {data: [_origin, _to, _assetId, amount, _channel, _nextMemo]} = ibcEvent;
    const amountRaw = amount.toString() as string;
    ibcSentAmount = new BigNumber(amountRaw.replaceAll(',', ''));
  });

  it('Wait for funds to arrive at composable', async () => {
    const ksm = picassoAssets.find(asset => asset.id === '4');
    const preUserBalance = ksm?.balance.get(testWallet.address) as BigNumber;
    const preEscrowBalance = ksm?.balance.get(substrateEscrowAddress) as BigNumber;
    const preTotalIssuance = ksm?.totalIssuance;
    const feeCharged = ibcSentAmount.multipliedBy(0.004);
    await waitForEvent(picassoApi, picassoApi.events.ibc.TokenReceived.is);
    await waitForBlocks(picassoApi, 1);
    await getBalance(
      picassoAssets,
      [testWallet.address, feeAddress, substrateEscrowAddress],
      'dotsama',
      picassoApi
    );
    await getTotalIssuance(picassoAssets, 'dotsama', picassoApi);
    const afterUserBalance = ksm?.balance.get(testWallet.address) as BigNumber;
    const afterEscrowBalance = ksm?.balance.get(substrateEscrowAddress) as BigNumber;
    const afterTotalIssuance = ksm?.totalIssuance as BigNumber;
    const expectedBalanceIncrease = ibcSentAmount.minus(feeCharged);
    expect(toNumber(afterUserBalance.minus(preUserBalance), 12)).to.be
      .within(toNumber(expectedBalanceIncrease.minus('10000000000'), 12),
        toNumber(expectedBalanceIncrease.plus('10000000000'), 12));
    expect(afterTotalIssuance).to.be.bignumber.eq(preTotalIssuance);
    expect(afterEscrowBalance).to.be.bignumber.eq(preEscrowBalance.minus(ibcSentAmount));
  });

  after('Disconnects from apis', async () => {
    await picassoApi.disconnect();
    await composableApi.disconnect();
    await kusamaApi.disconnect();
  });
})