import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";


export async function createLiquidationStrategyHandler(
  sudoKey,
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