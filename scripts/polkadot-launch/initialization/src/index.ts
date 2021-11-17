const { ApiPromise, WsProvider } = require("@polkadot/api");
const { ISubmittableResult, XcmV1MultiLocation, XcmV1MultilocationJunctions } = require("@polkadot/types/types");
const { Keyring, KeyringPair } = require("@polkadot/keyring");

interface IAsset {
    name: string;
    composable_id: number;
    basilisk_id: number;
}

enum AssetType {
    PoolShare = 1,
    Token = 0,
}

async function main() {
    const { assets, basilisk_collator_url, composable_collator_url } = require("../config/config.json");

    const composableApi = await createApi(composable_collator_url, {});
    const basiliskApi = await createApi(basilisk_collator_url, {
        AssetType: {
            _enum: {
                PoolShare: "(AssetId,AssetId)",
                Token: "Null",
            },
        },
    });

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

    /*
    >>>
    await doBasiliskAssetsMapping(
        basiliskApi,
        assets,
        alice,
    );
    */
}

async function createApi(url: string, types: object): Promise<typeof ApiPromise> {
    const provider = new WsProvider(url);
    return await ApiPromise.create({ provider }, types);
}

async function chainInfo(api: typeof ApiPromise) {
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version(),
    ]);

    console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
}

async function doComposableAssetsMapping(
    api: typeof ApiPromise,
    assets: IAsset[],
    root: typeof KeyringPair,
    localAdmin: typeof KeyringPair,
    foreignAdmin: typeof KeyringPair,
) {
    let adminsUpdated = false;
    const txs = [
        api.tx.assetsRegistry.setLocalAdmin(localAdmin.address),
        api.tx.assetsRegistry.setForeignAdmin(foreignAdmin.address),
    ];
    await api.tx.utility
        .batch(txs)
        .signAndSend(root, ({ status, events }: typeof ISubmittableResult) => {
            if (status.isInBlock) {
                console.log(`LocalAdmin and ForeignAdmin updated`);
                adminsUpdated = true;
            }
        });

    while (!adminsUpdated) {
        console.log(`Waiting admins update...`);
        await sleep(1000);
    }

    for (const { composable_id, basilisk_id } of assets) {
        await api.tx.assetsRegistry
            .approveAssetsMappingCandidate(composable_id, basilisk_id)
            .signAndSend(localAdmin, { nonce: -1 }, ({ status }: typeof ISubmittableResult) => {
                if (status.isInBlock) {
                    console.log(`Current status of approveAssetsMappingCandidate(${composable_id}, ${basilisk_id}) is ${status}`);
                }
            });
        await api.tx.assetsRegistry
            .approveAssetsMappingCandidate(composable_id, basilisk_id)
            .signAndSend(foreignAdmin, { nonce: -1 }, ({ status }: typeof ISubmittableResult) => {
                if (status.isInBlock) {
                    console.log(`Current status of approveAssetsMappingCandidate(${composable_id}, ${basilisk_id}) is ${status}`);
                }
            });
    }
}

async function doBasiliskAssetsMapping(api: typeof ApiPromise, assets: IAsset[], root: typeof KeyringPair) {
    let { rootNonce } = await api.query.system.account(root.address);
    assets.forEach(async ({ name }) => {
        const existentialDeposit = 1000;
        const composableParachainId = 2000;
        await api.tx.assetRegistry
            .register(name, AssetType.Token, existentialDeposit)
            .signAndSend(root, { rootNonce }, ({ status }: typeof ISubmittableResult) => {
                console.log(`Current status of register(...) is ${status}`);
            });
        rootNonce++;
        const assetId = api.query.assetRegistry
            .assetIds(name);
        const location: typeof XcmV1MultiLocation = {
            interior: XcmV1MultilocationJunctions.XcmV1Junction.asParachain(composableParachainId),
            parents: 0,
        };
        await api.tx.assetRegistry
            .setLocation(assetId, location)
            .signAndSend(root, { rootNonce }, ({ status }: typeof ISubmittableResult) => {
                console.log(`Current status of setLocation(...) is ${status}`);
            });
        rootNonce++;
    });
}

function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

main().catch(console.error).finally(() => process.exit());
