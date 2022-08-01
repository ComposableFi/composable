"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
require("./interfaces/augment-api");
require("./interfaces/augment-types");
const api_1 = require("@polkadot/api");
const keyring_1 = require("@polkadot/keyring");
const types_1 = require("@polkadot/types");
const bn_js_1 = __importDefault(require("bn.js"));
const definitions = __importStar(require("./interfaces/definitions"));
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        const { assets, basilisk_collator_url, composable_collator_url, basilisk_para_id, composable_para_id, } = require("../config/config.json");
        const composableTypes = {
            AssetNativeLocation: "MultiLocation",
            MultiLocation: "MultiLocationV1",
        };
        const basiliskTypes = Object.values(definitions).reduce((res, { types }) => (Object.assign(Object.assign({}, res), types)), {});
        const composableApi = yield createApi(composable_collator_url, composableTypes);
        const basiliskApi = yield createApi(basilisk_collator_url, basiliskTypes);
        yield chainInfo(composableApi);
        yield chainInfo(basiliskApi);
        const keyring = new keyring_1.Keyring({ type: "sr25519" });
        const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
        const composableLocalAdmin = keyring.addFromUri("//Eve", { name: "Eve default" });
        const composableForeignAdmin = keyring.addFromUri("//Ferdie", { name: "Ferdie default" });
        yield doBasiliskAssetsMapping(basiliskApi, assets, alice);
        yield doComposableAssetsMapping(basiliskApi, composableApi, assets, basilisk_para_id, alice, composableLocalAdmin, composableForeignAdmin);
    });
}
function createApi(url, types) {
    return __awaiter(this, void 0, void 0, function* () {
        const provider = new api_1.WsProvider(url);
        return yield api_1.ApiPromise.create({ provider, types });
    });
}
function chainInfo(api) {
    return __awaiter(this, void 0, void 0, function* () {
        const [chain, nodeName, nodeVersion] = yield Promise.all([
            api.rpc.system.chain(),
            api.rpc.system.name(),
            api.rpc.system.version(),
        ]);
        console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
    });
}
function doComposableAssetsMapping(basiliskApi, composableApi, assets, basiliskParaId, root, localAdmin, foreignAdmin) {
    return __awaiter(this, void 0, void 0, function* () {
        let adminsUpdated = false;
        const txs = [
            composableApi.tx.sudo.sudo(composableApi.tx.assetsRegistry.setLocalAdmin(localAdmin.address)),
            composableApi.tx.sudo.sudo(composableApi.tx.assetsRegistry.setForeignAdmin(foreignAdmin.address)),
        ];
        yield composableApi.tx.utility
            .batch(txs)
            .signAndSend(root, ({ status }) => {
            if (status.isInBlock) {
                console.log(`LocalAdmin and ForeignAdmin updated`);
                adminsUpdated = true;
            }
        });
        while (!adminsUpdated) {
            if (Math.round(Date.now() / 1000) % 5 === 0) {
                console.log(`Waiting admins update...`);
            }
            yield sleep(1000);
        }
        for (const { name, composable_id, basilisk_id, decimals } of assets) {
            const basiliskAssetIdOpt = yield basiliskApi.query.assetRegistry
                .assetIds(name);
            const basiliskAssetId = basiliskAssetIdOpt.unwrapOr(null);
            if (basiliskAssetId === null) {
                console.log(`AssetId with name=${name} not found. Stopping work.`);
                return;
            }
            const junctionsV2 = (0, types_1.createType)(composableApi.registry, "JunctionsV2", { x2: [{ parachain: basiliskParaId }, { generalKey: toBE(basiliskAssetId) }] });
            const location = (0, types_1.createType)(composableApi.registry, "AssetNativeLocation", { parents: 0, interior: junctionsV2 });
            yield composableApi.tx.assetsRegistry
                .approveAssetsMappingCandidate(composable_id, basilisk_id, location, decimals)
                .signAndSend(localAdmin, { nonce: -1 }, ({ status }) => {
                if (status.isInBlock) {
                    let function_call = `approveAssetsMappingCandidate(${composable_id}, ${basilisk_id}, ${location}, ${decimals})`;
                    console.log(`Current status of ${function_call} is ${status}`);
                }
            });
            yield composableApi.tx.assetsRegistry
                .approveAssetsMappingCandidate(composable_id, basilisk_id, location, decimals)
                .signAndSend(foreignAdmin, { nonce: -1 }, ({ status }) => {
                if (status.isInBlock) {
                    let function_call = `approveAssetsMappingCandidate(${composable_id}, ${basilisk_id}, ${location}, ${decimals})`;
                    console.log(`Current status of ${function_call} is ${status}`);
                }
            });
        }
    });
}
function doBasiliskAssetsMapping(api, assets, root) {
    return __awaiter(this, void 0, void 0, function* () {
        for (const { name, composable_id } of assets) {
            const existentialDeposit = 1000;
            const assetType = (0, types_1.createType)(api.registry, "AssetType", { Token: true });
            let registrationDone = false;
            yield api.tx.sudo
                .sudo(api.tx.assetRegistry.register(name, assetType, existentialDeposit))
                .signAndSend(root, { nonce: -1 }, ({ status }) => {
                if (status.isInBlock) {
                    console.log(`Current status of register(...) is ${status}`);
                    registrationDone = true;
                }
            });
            while (!registrationDone) {
                if (Math.round(Date.now() / 1000) % 5 === 0) {
                    console.log(`Waiting registration...`);
                }
                yield sleep(1000);
            }
            const assetIdOpt = yield api.query.assetRegistry
                .assetIds(name);
            const assetId = assetIdOpt.unwrapOr(null);
            if (assetId === null) {
                console.log(`AssetId with name=${name} not found. Stopping work.`);
                return;
            }
            const junctionsV1 = (0, types_1.createType)(api.registry, "JunctionsV1", { here: true });
            const location = (0, types_1.createType)(api.registry, "AssetNativeLocation", { parents: 0, interior: junctionsV1 });
            yield api.tx.sudo
                .sudo(api.tx.assetRegistry.setLocation(assetId, location))
                .signAndSend(root, { nonce: -1 }, ({ status }) => {
                console.log(`Current status of setLocation(...) is ${status}`);
            });
        }
    });
}
function toBE(a) {
    return (new bn_js_1.default(a)).toArray();
}
function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}
main().catch(console.error).finally(() => process.exit());
//# sourceMappingURL=index.js.map