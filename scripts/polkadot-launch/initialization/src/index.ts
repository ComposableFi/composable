import "./interfaces/augment-api";
import "./interfaces/augment-types";

import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import type { KeyringPair } from "@polkadot/keyring/types";
import { createType } from "@polkadot/types";
import type { AssetId, JunctionsV1 } from "@polkadot/types/interfaces";
import type { ISubmittableResult, RegistryTypes } from "@polkadot/types/types";

import * as definitions from "./interfaces/definitions";
import type { AssetNativeLocation, AssetType } from "./interfaces/types";

interface IAsset {
    name: string;
    composable_id: number;
    basilisk_id: number;
}

async function main() {
    const { assets, basilisk_collator_url, composable_collator_url } = require("../config/config.json");
    const basiliskTypes = Object.values(definitions).reduce((res, { types }): object => ({ ...res, ...types }), {});

    const composableApi = await createApi(composable_collator_url, undefined);
    const basiliskApi = await createApi(basilisk_collator_url, basiliskTypes);

    await chainInfo(composableApi);
    await chainInfo(basiliskApi);

    const keyring = new Keyring({ type: "sr25519" });
    const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
    const composableLocalAdmin = keyring.addFromUri("//Eve", { name: "Eve default" });
    const composableForeignAdmin = keyring.addFromUri("//Ferdie", { name: "Ferdie default" });

    await doComposableAssetsMapping(
        composableApi,
        assets,
        alice,
        composableLocalAdmin,
        composableForeignAdmin,
    );

    await doBasiliskAssetsMapping(
        basiliskApi,
        assets,
        alice,
    );
}

async function createApi(url: string, types: RegistryTypes | undefined): Promise<ApiPromise> {
    const provider = new WsProvider(url);
    return await ApiPromise.create({ provider, types });
}

async function chainInfo(api: ApiPromise) {
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version(),
    ]);

    console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
}

async function doComposableAssetsMapping(
    api: ApiPromise,
    assets: IAsset[],
    root: KeyringPair,
    localAdmin: KeyringPair,
    foreignAdmin: KeyringPair,
) {
    let adminsUpdated = false;
    const txs = [
        api.tx.sudo.sudo(api.tx.assetsRegistry.setLocalAdmin(localAdmin.address)),
        api.tx.sudo.sudo(api.tx.assetsRegistry.setForeignAdmin(foreignAdmin.address)),
    ];
    await api.tx.utility
        .batch(txs)
        .signAndSend(root, ({ status }: ISubmittableResult) => {
            if (status.isInBlock) {
                console.log(`LocalAdmin and ForeignAdmin updated`);
                adminsUpdated = true;
            }
        });

    while (!adminsUpdated) {
        if (Math.round(Date.now() / 1000) % 5 === 0) {
            console.log(`Waiting admins update...`);
        }
        await sleep(1000);
    }

    for (const { composable_id, basilisk_id } of assets) {
        await api.tx.assetsRegistry
            .approveAssetsMappingCandidate(composable_id, basilisk_id)
            .signAndSend(localAdmin, { nonce: -1 }, ({ status }: ISubmittableResult) => {
                if (status.isInBlock) {
                    console.log(`Current status of approveAssetsMappingCandidate(${composable_id}, ${basilisk_id}) is ${status}`);
                }
            });
        await api.tx.assetsRegistry
            .approveAssetsMappingCandidate(composable_id, basilisk_id)
            .signAndSend(foreignAdmin, { nonce: -1 }, ({ status }: ISubmittableResult) => {
                if (status.isInBlock) {
                    console.log(`Current status of approveAssetsMappingCandidate(${composable_id}, ${basilisk_id}) is ${status}`);
                }
            });
    }
}

async function doBasiliskAssetsMapping(api: ApiPromise, assets: IAsset[], root: KeyringPair) {
    for (const { name } of assets) {
        const existentialDeposit = 1000;
        const composableParachainId = 2000;
        const assetType: AssetType = createType(api.registry, "AssetType", { Token: true });
        let registrationDone = false;
        await api.tx.sudo
            .sudo(api.tx.assetRegistry.register(name, assetType, existentialDeposit))
            .signAndSend(root, { nonce: -1 }, ({ status }: ISubmittableResult) => {
                if (status.isInBlock) {
                    console.log(`Current status of register(...) is ${status}`);
                    registrationDone = true;
                }
            });
        while (!registrationDone) {
            if (Math.round(Date.now() / 1000) % 5 === 0) {
                console.log(`Waiting registration...`);
            }
            await sleep(1000);
        }
        const assetIdOpt = await api.query.assetRegistry
            .assetIds(name);
        const assetId: AssetId | null = assetIdOpt.unwrapOr(null);
        if (assetId === null) {
            console.log(`AssetId with name=${name} not found. Stopping work.`);
            return;
        }
        const junctionsV1: JunctionsV1 = createType(api.registry, "JunctionsV1", { here: true });
        const location: AssetNativeLocation = createType(
            api.registry,
            "AssetNativeLocation",
            { parents: 0, interior: junctionsV1 },
        );
        await api.tx.sudo
            .sudo(api.tx.assetRegistry.setLocation(assetId, location))
            .signAndSend(root, { nonce: -1 }, ({ status }: ISubmittableResult) => {
                console.log(`Current status of setLocation(...) is ${status}`);
            });
    }
}

function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

main().catch(console.error).finally(() => process.exit());
