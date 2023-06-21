import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import { PABLO_OVERVIEW_STATS, PabloOverviewStats } from "@composable/utils/subsquid/apollo/queries/pabloOverviewStats";
import { PABLO_SPOT_PRICE, PabloSpotPrice } from "@composable/utils/subsquid/apollo/queries/pabloSpotPrice";
import { PABLO_DAILY, PabloDaily } from "@composable/utils/subsquid/apollo/queries/pabloDaily";
import { PABLO_TOTAL_VALUE_LOCKED, PabloTVL } from "@composable/utils/subsquid/apollo/queries/pabloTVL";

import { client } from "@composable/utils/subsquid/apollo/apolloGraphql";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { getDevWallets } from "@composable/utils/walletHelper";
import { KeyringPair } from "@polkadot/keyring/types";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";

function Pica(value: number) {
  return BigInt(value) * BigInt(10 ** 12);
}

describe("Pablo graphql queries", function () {
  let api: ApiPromise;
  let sudoKey: KeyringPair;

  before("Setting up the tests", async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  it("Correctly populates overview data", async function () {
    this.timeout(2 * 60 * 1000);
    this.retries(0);
    const poolId = api.createType("u128", 0);

    await mintAssetsToWallet(api, sudoKey, sudoKey, [4, 130], BigInt(1_000_000_000_000_000));

    const poolAssetsAdded = api.createType("BTreeMap<u128, u128>", {
      "4": Pica(700),
      "130": Pica(700)
    });

    console.log("Adding liquidity to pool 0...");
    await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.pablo.LiquidityAdded.is,
      api.tx.pablo.addLiquidity(poolId, poolAssetsAdded, 0, false)
    );

    const poolAssetsRemoved = api.createType("BTreeMap<u128, u128>", {
      "4": Pica(0),
      "130": Pica(0)
    });

    console.log("Removing liquidity from pool 0...");
    await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.pablo.LiquidityRemoved.is,
      api.tx.pablo.removeLiquidity(poolId, api.createType("u128", Pica(100)), poolAssetsRemoved)
    );

    // This gives enough time for Subsquid to process the events for the next tests
    console.log("Waiting for Subsquid to process the events...");
    await waitForBlocks(api, 2);

    const { data } = await client.query<PabloOverviewStats>({ query: PABLO_OVERVIEW_STATS });

    const { totalValueLocked } = data.pabloOverviewStats;

    expect(totalValueLocked.find(tvl => tvl.assetId === "4")!.amount).not.to.be.equal("0");

    const assetIdSet = new Set(totalValueLocked.map(({ assetId }) => assetId));
    expect(assetIdSet.size).to.equal(totalValueLocked.length);

    for (const tvl of totalValueLocked) {
      expect(tvl.amount).not.to.equal("0");
    }
  });

  it("Correctly gets spot price", async function () {
    this.timeout(2 * 60 * 1000);
    this.retries(0);

    const poolId = api.createType("u128", 0);

    const poolAssetsSwapped = api.createType("ComposableTraitsDexAssetAmount", {
      assetId: api.createType("u128", 4),
      amount: api.createType("u128", Pica(100))
    });

    const minReceive = api.createType("ComposableTraitsDexAssetAmount", {
      assetId: api.createType("u128", 130),
      amount: api.createType("u128", 0)
    });

    console.log("Swapping assets from pool 0...");
    await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.pablo.Swapped.is,
      api.tx.pablo.swap(poolId, poolAssetsSwapped, minReceive, false)
    );

    // This gives enough time for Subsquid to process the events for the next tests
    console.log("Waiting for Subsquid to process the events...");
    await waitForBlocks(api, 2);

    const { data: dataAfter1 } = await client.query<PabloSpotPrice>({
      query: PABLO_SPOT_PRICE,
      variables: { poolId: "0", baseAssetId: "130", quoteAssetId: "4" }
    });

    expect(dataAfter1.pabloSpotPrice.spotPrice).to.equal("1.145876374781079");

    const { data: dataAfter2 } = await client.query<PabloSpotPrice>({
      query: PABLO_SPOT_PRICE,
      variables: { poolId: "0", baseAssetId: "4", quoteAssetId: "130" }
    });

    expect(dataAfter2.pabloSpotPrice.spotPrice).to.equal("0.8726944913154799");
  });

  // TODO: fix
  it.skip("Correctly gets daily stats", async function () {
    const { data } = await client.query({
      query: PABLO_DAILY,
      variables: { asd: "0" }
    });

    const { fees, poolId, volume, transactions, assetId } = data.pabloDaily;

    expect(fees).to.equal("600000000000");
    expect(poolId).to.equal("0");
    expect(volume).to.equal("174538898263096");
    expect(transactions).to.equal("3");
    expect(assetId).to.equal("4");
  });

  it("Correctly gets TVL for last day", async function () {
    const { data } = await client.query<PabloTVL>({
      query: PABLO_TOTAL_VALUE_LOCKED,
      variables: {
        range: "day",
        poolId: "0"
      }
    });

    expect(data.pabloTVL[data.pabloTVL.length - 1].assetId).to.equal("4");
    expect(data.pabloTVL[data.pabloTVL.length - 1].totalValueLocked).not.to.equal("0");
  });

  it("Correctly gets TVL for last week", async function () {
    const { data } = await client.query<PabloTVL>({
      query: PABLO_TOTAL_VALUE_LOCKED,
      variables: {
        range: "week",
        poolId: "0"
      }
    });

    expect(data.pabloTVL[data.pabloTVL.length - 1].assetId).to.equal("4");
    expect(data.pabloTVL[data.pabloTVL.length - 1].totalValueLocked).not.to.equal("0");
  });

  it("Correctly gets TVL for last month", async function () {
    const { data } = await client.query<PabloTVL>({
      query: PABLO_TOTAL_VALUE_LOCKED,
      variables: {
        range: "month",
        poolId: "0"
      }
    });

    expect(data.pabloTVL[data.pabloTVL.length - 1].assetId).to.equal("4");
    expect(data.pabloTVL[data.pabloTVL.length - 1].totalValueLocked).not.to.equal("0");
  });

  it("Correctly gets TVL for last year", async function () {
    const { data } = await client.query<PabloTVL>({
      query: PABLO_TOTAL_VALUE_LOCKED,
      variables: {
        range: "year",
        poolId: "0"
      }
    });

    expect(data.pabloTVL[data.pabloTVL.length - 1].assetId).to.equal("4");
    expect(data.pabloTVL[data.pabloTVL.length - 1].totalValueLocked).not.to.equal("0");
  });
});
