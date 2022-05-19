import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise } from "@polkadot/api";


export async function createLiquidationStrategyHandler(
  api: ApiPromise,
  sudoKey: KeyringPair,
  configuration
) {
  return await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.liquidations.addLiqudationStrategy(configuration)
    )
  );
}