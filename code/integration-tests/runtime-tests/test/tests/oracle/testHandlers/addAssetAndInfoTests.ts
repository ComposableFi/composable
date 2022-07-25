import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { Bool, u128, u32 } from "@polkadot/types-codec";
import { AnyNumber } from "@polkadot/types-codec/types";
import { Balance, BlockNumber, Percent } from "@polkadot/types/interfaces/runtime";
import { IEvent } from "@polkadot/types/types";
import { expect } from "chai";

/**
 * Tests tx.oracle.addAssetAndInfo with provided parameters that should succeed.
 * @param api Connect ApiPromise
 * @param {KeyringPair} sudoKey Connected API Promise w/ sudo rights.
 * @param assetId Id for the asset
 * @param threshold Percent close to mean to be rewarded
 * @param minAnswers Min answers before aggregation
 * @param maxAnswers Max answers to aggregate
 * @param blockInterval blocks until oracle triggered
 * @param reward reward amount for correct answer
 * @param slash slash amount for bad answer
 * @param emitPriceChanges Emit price changes event
 */
export async function txOracleAddAssetAndInfoSuccessTest(
  api: ApiPromise,
  sudoKey: KeyringPair,
  assetId: number | u128 | AnyNumber | Uint8Array,
  threshold: Percent | AnyNumber | Uint8Array,
  minAnswers: number | u32 | AnyNumber | Uint8Array,
  maxAnswers: number | u32 | AnyNumber | Uint8Array,
  blockInterval: number | u32 | AnyNumber | Uint8Array,
  reward: number | u128 | AnyNumber | Uint8Array,
  slash: number | u128 | AnyNumber | Uint8Array,
  emitPriceChanges: boolean | Bool
): Promise<IEvent<[u128, Percent, u32, u32, BlockNumber, Balance, Balance]>> {
  return await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.oracle.AssetInfoChange.is,
    api.tx.sudo.sudo(
      api.tx.oracle.addAssetAndInfo(
        assetId,
        threshold,
        minAnswers,
        maxAnswers,
        blockInterval,
        reward,
        slash,
        emitPriceChanges
      )
    )
  );
}

/**
 *
 */
export async function verifyOracleCreation(
  api: ApiPromise,
  resultData: IEvent<[u128, Percent, u32, u32, BlockNumber, Balance, Balance]>,
  expectedData: {
    threshold: Percent;
    minAnswers: u32;
    maxAnswers: u32;
    blockInterval: BlockNumber;
    rewardWeight: Balance;
    slash: Balance;
  }
) {
  // 1. Comparing result from creation with oracle stats on chain.
  const oracleStatsWrapped = await api.query.oracle.assetsInfo(resultData.data[0]);
  const oracleStats = oracleStatsWrapped.unwrap();
  expect(oracleStats.threshold.toString()).to.be.equal(resultData.data[1].toString());
  expect(oracleStats.minAnswers.toString()).to.be.equal(resultData.data[2].toString());
  expect(oracleStats.maxAnswers.toString()).to.be.equal(resultData.data[3].toString());
  expect(oracleStats.blockInterval.toString()).to.be.equal(resultData.data[4].toString());
  expect(oracleStats.rewardWeight.toString()).to.be.equal(resultData.data[5].toString());
  expect(oracleStats.slash.toString()).to.be.equal(resultData.data[6].toString());

  // 2. Comparing oracle stats on chain, with intended parameters.
  expect(oracleStats.threshold.toString()).to.be.equal(expectedData.threshold.toString());
  expect(oracleStats.minAnswers.toString()).to.be.equal(expectedData.minAnswers.toString());
  expect(oracleStats.maxAnswers.toString()).to.be.equal(expectedData.maxAnswers.toString());
  expect(oracleStats.blockInterval.toString()).to.be.equal(expectedData.blockInterval.toString());
  expect(oracleStats.rewardWeight.toString()).to.be.equal(expectedData.rewardWeight.toString());
  expect(oracleStats.slash.toString()).to.be.equal(expectedData.slash.toString());
}
