#!/usr/bin/env ts-node
/**
 * This is the entry point for the devnet setup script.
 */
import "@composable/types";
import "@composable/types/lookup";
import "@composable/types/types-lookup";
import "@composable/types/augment-api";
import "@composable/types/augment-types";

import { ApiPromise, Keyring } from "@polkadot/api";
import { getNewConnection, sendAndWaitForSuccess } from "@composable/utils";
import { u128 } from "@polkadot/types-codec";
import { JunctionsV2 } from "@polkadot/types/interfaces";
import { getDevWallets } from "@composable/utils";
import { IEvent } from "@polkadot/types/types";
import { ComposableTraitsXcmAssetsXcmAssetLocation } from "@composable/types";

const parachainStatemineId = 1000;
const parachainKaruraId = 2000;
const parachainDaliId = 2087;

const karuraAssetIdKAR = 128;
const karuraAssetIdKUSD = 129;

const connect = async (endpoint: string): Promise<{ api: ApiPromise; keyring: Keyring }> => {
  const { newClient, newKeyring } = await getNewConnection(endpoint);
  const api = newClient;
  const keyring = newKeyring;
  return { api, keyring };
};

const connectPara = async (): Promise<{ api: ApiPromise; keyring: Keyring }> =>
  connect(process.env.PARACHAIN_ENDPOINT ?? "ws://127.0.0.1:9988")

const registerAssets = async () => {
  const { api, keyring } = await connectPara();

  const { devWalletAlice: wallet } = getDevWallets(keyring);

  const karuraAssetLocation = (assetId: number): JunctionsV2 =>
    api.createType("JunctionsV2", {
      x2: [{
        parachain: parachainKaruraId
      }, {
        generalKey: api.createType("Bytes", api.createType("u16", assetId).toHex())
      }]
    });

  const registerAsset = (location: JunctionsV2): Promise<IEvent<[assetId: u128, location: ComposableTraitsXcmAssetsXcmAssetLocation]>> =>
    sendAndWaitForSuccess(
      api,
      wallet,
      api.events.assetsRegistry.AssetRegistered.is,
      api.tx.sudo.sudo(
        api.tx.assetsRegistry.registerAsset({
          parents: 1,
          interior: location
        },
          0, // Existential deposit
          "1000000000000000000", // FixedU128::One
          12 // Decimals
        )
      )
    );

  console.log("Setting up assets registry.");

  console.log("Registering Karura KAR token...");
  await registerAsset(karuraAssetLocation(karuraAssetIdKAR));
  console.log("Done.");

  console.log("Registering Karura KUSD token...");
  await registerAsset(karuraAssetLocation(karuraAssetIdKUSD));
  console.log("Done.");

  await api.disconnect();
};

const main = async () => {
  await registerAssets();
};

main()
  .then(() => {
    process.exit(0);
  })
  .catch(err => {
    console.error(err.toString());
    process.exit(1);
  });
