import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise } from "@polkadot/api";

// ToDo: Add types!
export async function createLendingMarketHandler(
  api: ApiPromise,
  wallet:KeyringPair,
  input,
  keepAlive
) {

  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.lending.MarketCreated.is,
    api.tx.lending.createMarket(input, keepAlive),
    false
  );
}