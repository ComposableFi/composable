"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
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
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.getNewConnection = void 0;
const chai_1 = __importDefault(require("chai"));
const chai_bn_1 = __importDefault(require("chai-bn"));
const bn_js_1 = __importDefault(require("bn.js"));
const definitions = __importStar(require("@composable/types/interfaces/definitions"));
const api_1 = require("@polkadot/api");
/**
 * Async function to set up a picasso blockchain connection.
 * It reads our chain types & sets them up. Then defines the connection endpoint,
 * as well as creating a WebSocket Provider, and finally establishes the connection.
 *
 * ToDo: Add functionality to read private keys from `external file` or `env variables`.
 *    This will make it possible to use the integration tests against a live network.
 *
 * @return Promise<{ApiPromise, Keyring}> The connected API client object & a ready to use Keyring
 */
async function getNewConnection() {
    // Enable and inject BN dependency
    chai_1.default.use((0, chai_bn_1.default)(bn_js_1.default));
    const rpc = Object.keys(definitions)
        .filter(k => Object.keys(definitions[k].rpc).length > 0)
        .reduce((accumulator, key) => ({ ...accumulator, [key]: definitions[key].rpc }), {});
    const types = Object.values(definitions).reduce((accumulator, { types }) => ({ ...accumulator, ...types }), {});
    const endpoint = "ws://" + (process.env.ENDPOINT ?? "127.0.0.1:9988");
    const provider = new api_1.WsProvider(endpoint);
    const apiOptions = {
        provider,
        types,
        rpc
    };
    const newClient = await api_1.ApiPromise.create(apiOptions);
    // do something before every test,
    // then run the next hook in this array
    const newKeyring = new api_1.Keyring({ type: "sr25519" });
    return { newClient, newKeyring };
}
exports.getNewConnection = getNewConnection;
//# sourceMappingURL=connectionHelper.js.map