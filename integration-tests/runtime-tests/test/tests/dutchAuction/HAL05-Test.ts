import { getNewConnection } from "@composable/utils/connectionHelper";
import { mintAssetsToWallet, Pica } from "@composable/utils/mintingHelper";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { getDevWallets } from "@composable/utils/walletHelper";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { expect } from "chai";

/**
 * Test for validating the Dutch Auction Audit fix, creating a sell offer for the same asset.
 * Description: Inside the dutch-auction pallet ask function accepts sell offers with the
 * same asset provided as both base and quote.
 * This can result in situations when a victim pays, for example, 1000 tokens
 * to get 100 tokens of the same type.
 */
describe.only("Halborn Audit Fix Dutch Auction Fix Test", function () {
  let api: ApiPromise;
  let sudoKey: KeyringPair, walletId1: KeyringPair;
  let eth: number;
  let amount: bigint, limit: bigint;
  this.timeout(3 * 60 * 1000);
  before("Initialize Variables", async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletEve } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    walletId1 = devWalletEve.derive("/test/dutchAuction/walletId1");
    eth = 6;
    amount = Pica(50);
    limit = Pica(1);
  });
  before("Minting assets", async function () {
    await mintAssetsToWallet(api, walletId1, sudoKey, [1, eth]);
  });
  after("Closing the connection", async function () {
    await api.disconnect();
  });
  it("Shouldn't be able to create a sell offer with the same asset Id's", async function () {
    const order = api.createType("ComposableTraitsDefiSellCurrencyId", {
      pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: api.createType("u128", eth),
        quote: api.createType("u128", eth)
      }),
      take: api.createType("ComposableTraitsDefiTake", {
        amount: api.createType("u128", amount),
        limit: api.createType("u128", limit)
      })
    });
    const configuration = api.createType("ComposableTraitsTimeTimeReleaseFunction", {
      linearDecrease: api.createType("ComposableTraitsTimeLinearDecrease", {
        total: api.createType("u64", 100)
      })
    });
    const {data: [result]} = await sendAndWaitForSuccess(
      api,
      walletId1,
      api.events.dutchAuction.OrderAdded.is,
      api.tx.dutchAuction.ask(order, configuration)
    ).catch(error => {
      console.warn(error.message);
      expect(error.message).to.contain("SomeValue");
      return error;
    });
    expect(result).to.be.an("Error");
  });
});
