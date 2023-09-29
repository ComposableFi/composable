import {KeyringPair} from "@polkadot/keyring/types";
import {initializeApi} from "./utils/apiClient";
import {ApiPromise, Keyring} from "@polkadot/api";
import {
  centauriAddress,
  composableEndpoint,
  dotOncomposable,
  ksmOnComposable,
  layrOnComposable,
  picaIdOnComposable,
  picassoEndpoint,
  picassoFeeAddress,
  usdtOnComposable
} from "./utils/constants";
import {sendAndWaitForWithBatch} from "./utils/txClient";
import {
  addFeelessChannels,
  calculateExpectedDifference,
  createIbcTxsForBatch,
  getActualandExpectedDiff,
  getStateOfFundsForPicassoToCentauri,
  getStateOfFundsForPicassoToComposable,
  initiateIbcFromCentauri,
  queryAllBalanceOnCentauri,
  queryAllBalanceOnPicasso,
  queryTokenBalance,
  queryTotalIssuanceOfTokensOnCentauri,
  queryTotalIssuanceOncomposable,
  waitForSeconds,
} from "./utils/ibcUtils";
import {mintAssetsToWallet} from "./utils/mintingHelper";
import BigNumber from "bignumber.js";
import {IPreBalances, IPreBalancesOnPicasso} from "./utils/types";
import {after} from "mocha";

const expect = require('chai').expect;

describe('IBC Tests', function () {
  this.timeout(6 * 60 * 1000);
  let testWallet1: KeyringPair;
  let sudoKey: KeyringPair;
  let picassoApi: ApiPromise;
  let composableApi: ApiPromise;
  const ksmTransfer = '100000000000000';
  const usdtTransfer = '100000000';
  const dotTransfer = '1000000000000';
  const picaTransfer = '10000000000000000';
  const ksmReturnTransfer = '10000000000000';
  const usdtReturnTransfer = '10000000';
  const dotReturnTransfer = '100000000000';
  const picaReturnTransfer = '1000000000000000';
  const fee: number = 0.004;
  let expectedDifference: { [x: string]: any; };
  const assets = [1, 4, 6, 130];

  before('Initialize Apis and mint tokens', async () => {
    const keyring = new Keyring({type: 'sr25519'});
    testWallet1 = keyring.createFromUri('//Alice/Ibc');
    sudoKey = keyring.createFromUri('//Alice');
    picassoApi = await initializeApi(picassoEndpoint);
    composableApi = await initializeApi(composableEndpoint);
    await addFeelessChannels(composableApi, sudoKey);
    await Promise.all([
      mintAssetsToWallet(composableApi, testWallet1, sudoKey, [dotOncomposable, layrOnComposable], '10000000000000', 'composable'),
      mintAssetsToWallet(picassoApi, sudoKey, sudoKey, [1, 4, 6, 130], '10000000000000000', 'picasso')
    ]);
    expectedDifference = calculateExpectedDifference(fee, ksmTransfer, usdtTransfer, picaTransfer, dotTransfer);
  });

  describe('Picasso to Centauri Tests', () => {
    let preBalanceOnCentauri: { dot: string; usdt: string; ksm: string; pica: string; };
    let preBalanceOnPicasso: any;
    let afterTotalIssuanceOnCentauri: IPreBalances;
    let preTotalIssuanceOnCentauri: IPreBalances;
    let feeAccountPreBalance: IPreBalancesOnPicasso;
    let feeAccountAfterBalance: IPreBalancesOnPicasso;

    before('Stores initial state before transfers', async () => {
      ({preTotalIssuanceOnCentauri, preBalanceOnCentauri, preBalanceOnPicasso, feeAccountPreBalance} =
        await getStateOfFundsForPicassoToCentauri(picassoApi, assets, testWallet1));
    });

    it('Given that users have sufficient funds, users can initiate a transfer of pica, usdt, ksm and dot to centauri', async () => {
      const transferAmounts = [picaTransfer, dotTransfer, usdtTransfer, ksmTransfer];
      const assetsToTransfer = [1, 6, 130, 4];
      const txArr = createIbcTxsForBatch(picassoApi, transferAmounts, assetsToTransfer, centauriAddress, 'centauri');
      await sendAndWaitForWithBatch(
        picassoApi,
        sudoKey,
        picassoApi.events.utility.BatchCompleted.is,
        txArr,
        false
      )
    });

    it('Pica funds arrives at centauri as initial amount minus fee', async () => {
      let afterBalanceOnCentauri = await queryAllBalanceOnCentauri(centauriAddress);
      while (afterBalanceOnCentauri.pica === preBalanceOnCentauri.pica) {
        await waitForSeconds(8);
        afterBalanceOnCentauri = await queryAllBalanceOnCentauri(centauriAddress);
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnCentauri.pica, preBalanceOnCentauri.pica, picaTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('Dot funds arrives at centauri as initial amount minus fee', async () => {
      let afterBalanceOnCentauri = await queryAllBalanceOnCentauri(centauriAddress);
      while (afterBalanceOnCentauri.dot === preBalanceOnCentauri.dot) {
        await waitForSeconds(8);
        afterBalanceOnCentauri = await queryAllBalanceOnCentauri(centauriAddress);
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnCentauri.dot, preBalanceOnCentauri.dot, dotTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('Usdt funds arrives at centauri as initial amount minus fee', async () => {
      let afterBalanceOnCentauri = await queryAllBalanceOnCentauri(centauriAddress);
      while (afterBalanceOnCentauri.usdt === preBalanceOnCentauri.usdt) {
        await waitForSeconds(8);
        afterBalanceOnCentauri = await queryAllBalanceOnCentauri(centauriAddress);
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnCentauri.usdt, preBalanceOnCentauri.usdt, usdtTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('Ksm funds arrives at centauri as initial amount minus fee', async () => {
      let afterBalanceOnCentauri = await queryAllBalanceOnCentauri(centauriAddress);
      while (afterBalanceOnCentauri.ksm === preBalanceOnCentauri.ksm) {
        await waitForSeconds(8);
        afterBalanceOnCentauri = await queryAllBalanceOnCentauri(centauriAddress);
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnCentauri.ksm, preBalanceOnCentauri.ksm, ksmTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('Total issuance of dot,ksm, pica and usdt changes on centauri with the transfers to centauri', async () => {
      afterTotalIssuanceOnCentauri = await queryTotalIssuanceOfTokensOnCentauri();
      const assets = ['dot', 'ksm', 'usdt', 'pica'];
      assets.map(asset => {
        expect((new BigNumber(afterTotalIssuanceOnCentauri[asset as keyof IPreBalances])
          .minus(preTotalIssuanceOnCentauri[asset as keyof IPreBalances]).toString())).to.be.eq(expectedDifference[asset]);
      })
    });

    it('Pica, Ksm, Dot and USDT transfers are charged with %0.4 percent', async () => {
      feeAccountAfterBalance = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], picassoFeeAddress);
      const picaFee = new BigNumber(picaTransfer).multipliedBy(fee);
      const ksmFee = new BigNumber(ksmTransfer).multipliedBy(fee);
      const usdtFee = new BigNumber(usdtTransfer).multipliedBy(fee);
      const dotFee = new BigNumber(dotTransfer).multipliedBy(fee);
      expect((new BigNumber(feeAccountAfterBalance['1']).minus(new BigNumber(feeAccountPreBalance['1']))).toString())
        .to.be.eq(picaFee.toString());
      expect((new BigNumber(feeAccountAfterBalance['4']).minus(new BigNumber(feeAccountPreBalance['4']))).toString())
        .to.be.eq(ksmFee.toString());
      expect((new BigNumber(feeAccountAfterBalance['6']).minus(new BigNumber(feeAccountPreBalance['6']))).toString())
        .to.be.eq(dotFee.toString());
      expect((new BigNumber(feeAccountAfterBalance['130']).minus(new BigNumber(feeAccountPreBalance['130']))).toString())
        .to.be.eq(usdtFee.toString());
    });

    it('Given that users have sufficient funds, users can initiate transfers from centauri to picasso', async () => {
      ({
        preTotalIssuanceOnCentauri,
        preBalanceOnCentauri,
        preBalanceOnPicasso,
        feeAccountPreBalance,
      } = await getStateOfFundsForPicassoToCentauri(picassoApi, assets, testWallet1))
      await initiateIbcFromCentauri(centauriAddress, testWallet1.address.toString());
    });

    it('Dot funds arrives at picasso as initial amount minus fee', async () => {
      let afterBalanceOnPicasso = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], testWallet1.address.toString());
      while (afterBalanceOnPicasso['6'] === preBalanceOnPicasso['6']) {
        await waitForSeconds(8);
        afterBalanceOnPicasso = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], testWallet1.address.toString());
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnPicasso['6'], preBalanceOnPicasso['6'], dotReturnTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('Ksm funds arrives at picasso as initial amount minus fee', async () => {
      let afterBalanceOnPicasso = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], testWallet1.address.toString());
      while (afterBalanceOnPicasso['4'] === preBalanceOnPicasso['4']) {
        await waitForSeconds(8);
        afterBalanceOnPicasso = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], testWallet1.address.toString());
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnPicasso['4'], preBalanceOnPicasso['4'], ksmReturnTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('Usdt funds arrives at picasso as initial amount minus fee', async () => {
      let afterBalanceOnPicasso = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], testWallet1.address.toString());
      while (afterBalanceOnPicasso['130'] === preBalanceOnPicasso['130']) {
        await waitForSeconds(8);
        afterBalanceOnPicasso = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], testWallet1.address.toString());
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnPicasso['130'], preBalanceOnPicasso['130'], usdtReturnTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('Total issuance on centauri is changed as a result of outgoing txs', async () => {
      afterTotalIssuanceOnCentauri = await queryTotalIssuanceOfTokensOnCentauri();
      expect((new BigNumber(preTotalIssuanceOnCentauri['ksm'])
        .minus(afterTotalIssuanceOnCentauri['ksm']).toString())).to.be.eq(ksmReturnTransfer);
      expect((new BigNumber(preTotalIssuanceOnCentauri['dot'])
        .minus(afterTotalIssuanceOnCentauri['dot']).toString())).to.be.eq(dotReturnTransfer);
      expect((new BigNumber(preTotalIssuanceOnCentauri['usdt'])
        .minus(afterTotalIssuanceOnCentauri['usdt']).toString())).to.be.eq(usdtReturnTransfer);
    })

    it('Fee account charges for return transfers to picasso', async () => {
      feeAccountAfterBalance = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], picassoFeeAddress);
      const ksmFee = new BigNumber(ksmReturnTransfer).multipliedBy(fee);
      const usdtFee = new BigNumber(usdtReturnTransfer).multipliedBy(fee);
      const dotFee = new BigNumber(dotReturnTransfer).multipliedBy(fee);
      expect((new BigNumber(feeAccountAfterBalance['4']).minus(new BigNumber(feeAccountPreBalance['4']))).toString())
        .to.be.eq(ksmFee.toString());
      expect((new BigNumber(feeAccountAfterBalance['6']).minus(new BigNumber(feeAccountPreBalance['6']))).toString())
        .to.be.eq(dotFee.toString());
      expect((new BigNumber(feeAccountAfterBalance['130']).minus(new BigNumber(feeAccountPreBalance['130']))).toString())
        .to.be.eq(usdtFee.toString());
    })
  });

  describe.only('Picasso to Composable Tests', () => {
    let preBalancesOnComposable: IPreBalancesOnPicasso;
    let preBalanceOnPicasso: IPreBalancesOnPicasso;
    let preTotalIssuanceOnComposable: IPreBalances;
    let feeAccountPreBalance: IPreBalancesOnPicasso;
    let feeAccountAfterBalance;
    let afterTotalIssuanceOnComposable;

    before('Stores initial state of funds', async () => {
      ({preBalancesOnComposable, preBalanceOnPicasso, preTotalIssuanceOnComposable, feeAccountPreBalance} =
        await getStateOfFundsForPicassoToComposable(composableApi, picassoApi, testWallet1));
    });

    it('Given that user has sufficient funds, user initiates pica, usdt and ksm transfers to Composable', async () => {
      const amounts = [picaTransfer, usdtTransfer, ksmTransfer];
      const assets = [1, 130, 4];
      const ibcTxs =
        createIbcTxsForBatch(picassoApi, amounts, assets, testWallet1.address.toString(), 'composable');
      await sendAndWaitForWithBatch(
        picassoApi,
        sudoKey,
        picassoApi.events.utility.BatchCompleted.is,
        ibcTxs,
        false
      )
    });

    it('User will receive pica on composable as amount minus fee', async () => {
      let afterBalanceOnComposable = await queryTokenBalance(composableApi, testWallet1.address.toString(), picaIdOnComposable);
      while (afterBalanceOnComposable === preBalancesOnComposable['1']) {
        await waitForSeconds(12);
        afterBalanceOnComposable = await queryTokenBalance(composableApi, testWallet1.address.toString(), picaIdOnComposable);
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnComposable, preBalancesOnComposable['1'], picaTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('User will receive ksm on composable as amount minus fee', async () => {
      let afterBalanceOnComposable = await queryTokenBalance(composableApi, testWallet1.address.toString(), ksmOnComposable);
      while (afterBalanceOnComposable === preBalancesOnComposable['4']) {
        await waitForSeconds(12);
        afterBalanceOnComposable = await queryTokenBalance(composableApi, testWallet1.address.toString(), ksmOnComposable);
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnComposable, preBalancesOnComposable['4'], ksmTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('User will receive usdt on composable as amount minus fee', async () => {
      let afterBalanceOnComposable = await queryTokenBalance(composableApi, testWallet1.address.toString(), usdtOnComposable);
      while (afterBalanceOnComposable === preBalancesOnComposable['130']) {
        await waitForSeconds(12);
        afterBalanceOnComposable = await queryTokenBalance(composableApi, testWallet1.address.toString(), usdtOnComposable);
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnComposable, preBalancesOnComposable['130'], usdtTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('The outgoing transfers from picasso, will have fee charged', async () => {
      feeAccountAfterBalance = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], picassoFeeAddress);
      const ksmFee = new BigNumber(ksmTransfer).multipliedBy(fee);
      const usdtFee = new BigNumber(usdtTransfer).multipliedBy(fee);
      const picaFee = new BigNumber(picaTransfer).multipliedBy(fee);
      expect((new BigNumber(feeAccountAfterBalance['1']).minus(new BigNumber(feeAccountPreBalance['1']))).toString())
        .to.be.eq(picaFee.toString());
      expect((new BigNumber(feeAccountAfterBalance['4']).minus(new BigNumber(feeAccountPreBalance['4']))).toString())
        .to.be.eq(ksmFee.toString());
      expect((new BigNumber(feeAccountAfterBalance['130']).minus(new BigNumber(feeAccountPreBalance['130']))).toString())
        .to.be.eq(usdtFee.toString());
    });

    it('The total issuance of tokens on composable changes with the received token', async () => {
      afterTotalIssuanceOnComposable = await queryTotalIssuanceOncomposable(composableApi);
      expect((new BigNumber(afterTotalIssuanceOnComposable['ksm'])
        .minus(preTotalIssuanceOnComposable['ksm']).toString())).to.be.eq(expectedDifference['ksm'].toString());
      expect((new BigNumber(afterTotalIssuanceOnComposable['pica'])
        .minus(preTotalIssuanceOnComposable['pica']).toString())).to.be.eq(expectedDifference['pica'].toString());
      expect((new BigNumber(afterTotalIssuanceOnComposable['usdt'])
        .minus(preTotalIssuanceOnComposable['usdt']).toString())).to.be.eq(expectedDifference['usdt'].toString());
    });

    it('Stores initial state before transfers', async () => {
      ({preBalancesOnComposable, preBalanceOnPicasso, preTotalIssuanceOnComposable, feeAccountPreBalance} =
        await getStateOfFundsForPicassoToComposable(composableApi, picassoApi, testWallet1));
    });

    it('Given that users have sufficient funds, users can initiate a transfer back from composable', async () => {
      const amounts = [picaReturnTransfer, dotReturnTransfer, usdtReturnTransfer, ksmReturnTransfer];
      const assets = [picaIdOnComposable, dotOncomposable, usdtOnComposable, ksmOnComposable];
      const ibcTxs =
        createIbcTxsForBatch(composableApi, amounts, assets, testWallet1.address.toString(), 'picasso');
      await sendAndWaitForWithBatch(
        composableApi,
        testWallet1,
        composableApi.events.utility.BatchCompleted.is,
        ibcTxs,
        false
      )
    });

    it('User will receive pica on picasso', async () => {
      let afterBalanceOnPicasso = await queryTokenBalance(picassoApi, testWallet1.address.toString(), '1');
      while (afterBalanceOnPicasso === preBalanceOnPicasso['1']) {
        await waitForSeconds(12);
        afterBalanceOnPicasso = await queryTokenBalance(picassoApi, testWallet1.address.toString(), '1');
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnPicasso, preBalanceOnPicasso['1'], picaReturnTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('User will receive ksm on picasso', async () => {
      let afterBalanceOnPicasso = await queryTokenBalance(picassoApi, testWallet1.address.toString(), '4');
      while (afterBalanceOnPicasso === preBalanceOnPicasso['4']) {
        await waitForSeconds(12);
        afterBalanceOnPicasso = await queryTokenBalance(picassoApi, testWallet1.address.toString(), '4');
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnPicasso, preBalanceOnPicasso['4'], ksmReturnTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('User will receive dot on picasso', async () => {
      let afterBalanceOnPicasso = await queryTokenBalance(picassoApi, testWallet1.address.toString(), '6');
      while (afterBalanceOnPicasso === preBalanceOnPicasso['6']) {
        await waitForSeconds(12);
        afterBalanceOnPicasso = await queryTokenBalance(picassoApi, testWallet1.address.toString(), '6');
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnPicasso, preBalanceOnPicasso['6'], dotReturnTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('User will receive usdt on picasso', async () => {
      let afterBalanceOnComposable = await queryTokenBalance(picassoApi, testWallet1.address.toString(), usdtOnComposable);
      while (afterBalanceOnComposable === preBalancesOnComposable['130']) {
        await waitForSeconds(12);
        afterBalanceOnComposable = await queryTokenBalance(picassoApi, testWallet1.address.toString(), usdtOnComposable);
      }
      const {expectedDiff, actualDiff} =
        getActualandExpectedDiff(afterBalanceOnComposable, preBalancesOnComposable['130'], usdtTransfer);
      expect(actualDiff.toString()).to.be.eq(expectedDiff.toString());
    });

    it('The incoming transfers to picasso, will have fee charged', async () => {
      feeAccountAfterBalance = await queryAllBalanceOnPicasso(picassoApi, [1, 4, 6, 130], picassoFeeAddress);
      const ksmFee = new BigNumber(ksmReturnTransfer).multipliedBy(fee);
      const usdtFee = new BigNumber(usdtReturnTransfer).multipliedBy(fee);
      const dotFee = new BigNumber(dotReturnTransfer).multipliedBy(fee);
      const picaFee = new BigNumber(picaReturnTransfer).multipliedBy(fee);
      expect((new BigNumber(feeAccountAfterBalance['1']).minus(new BigNumber(feeAccountPreBalance['1']))).toString())
        .to.be.eq(picaFee.toString());
      expect((new BigNumber(feeAccountAfterBalance['4']).minus(new BigNumber(feeAccountPreBalance['4']))).toString())
        .to.be.eq(ksmFee.toString());
      expect((new BigNumber(feeAccountAfterBalance['6']).minus(new BigNumber(feeAccountPreBalance['6']))).toString())
        .to.be.eq(dotFee.toString());
      expect((new BigNumber(feeAccountAfterBalance['130']).minus(new BigNumber(feeAccountPreBalance['130']))).toString())
        .to.be.eq(usdtFee.toString());
    });

    it('The total issuance of tokens on composable changes with the received token', async () => {
      afterTotalIssuanceOnComposable = await queryTotalIssuanceOncomposable(composableApi);
      expect((new BigNumber(preTotalIssuanceOnComposable['ksm'])
        .minus(afterTotalIssuanceOnComposable['ksm']).toString())).to.be.eq(ksmReturnTransfer);
      expect((new BigNumber(preTotalIssuanceOnComposable['dot'])
        .minus(afterTotalIssuanceOnComposable['dot']).toString())).to.be.eq(dotReturnTransfer);
      expect((new BigNumber(preTotalIssuanceOnComposable['pica'])
        .minus(afterTotalIssuanceOnComposable['pica']).toString())).to.be.eq(picaReturnTransfer);
      expect((new BigNumber(preTotalIssuanceOnComposable['usdt'])
        .minus(afterTotalIssuanceOnComposable['usdt']).toString())).to.be.eq(usdtReturnTransfer);
    });
  });

  after('closes connection', async () => {
    await picassoApi.disconnect();
    await composableApi.disconnect();
  });
});
