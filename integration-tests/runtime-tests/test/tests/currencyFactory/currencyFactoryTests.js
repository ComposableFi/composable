"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.CurrencyFactoryTests = void 0;
const chai_1 = require("chai");
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const walletHelper_1 = require("@composable/utils/walletHelper");
const polkadotjs_1 = require("@composable/utils/polkadotjs");
/**
 *
 * Currency Factory Integration Tests
 *
 * The currency factory pallet persists of 2 extrinsics.
 * - currencyFactory.setMetadata
 * - currencyFactory.addRange
 */
describe("[SHORT] Currency Factory Tests", function () {
    if (!test_configuration_json_1.default.enabledTests.enabled)
        return;
    let api;
    let sudoKey;
    let assetIdToSetMetadata;
    before("Setting up the tests", async function () {
        this.timeout(60 * 1000);
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletAlice } = (0, walletHelper_1.getDevWallets)(newKeyring);
        sudoKey = devWalletAlice;
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    describe("tx.currencyFactory.setMetadata", function () {
        it("Sudo can initialize new asset", async function () {
            this.timeout(2 * 60 * 1000);
            const amount = 999999999999;
            const beneficiary = sudoKey.publicKey;
            const assetListBefore = await api.query.currencyFactory.assetEd.entries();
            const { data: [resultCurrencyId, resultBeneficiary, resultAmount] } = await CurrencyFactoryTests.initializeNewAsset(api, sudoKey, amount, beneficiary);
            assetIdToSetMetadata = resultCurrencyId;
            (0, chai_1.expect)(resultBeneficiary.toString()).to.equal(api.createType("AccountId32", beneficiary).toString());
            (0, chai_1.expect)(resultAmount).to.be.bignumber.equal(amount.toString());
            const assetListAfter = await api.query.currencyFactory.assetEd.entries();
            (0, chai_1.expect)(assetListAfter.length).to.be.greaterThan(assetListBefore.length);
        });
        it("Sudo can set metadata for newly created asset", async function () {
            this.timeout(2 * 60 * 1000);
            const assetId = assetIdToSetMetadata;
            const metadata = {
                symbol: {
                    inner: "BAN"
                },
                name: {
                    inner: "Banana Coin"
                }
            };
            const { data: [result] } = await CurrencyFactoryTests.setMetadata(api, sudoKey, assetId, metadata);
            (0, chai_1.expect)(result.isOk).to.be.true;
            const assetMetadataAfter = (await api.query.currencyFactory.assetMetadata(assetId));
            (0, chai_1.expect)(assetMetadataAfter.unwrap().isEmpty).to.be.false;
            (0, chai_1.expect)(CurrencyFactoryTests.hex2a(assetMetadataAfter.unwrap().name.inner)).to.be.equal(metadata.name.inner.toString());
            (0, chai_1.expect)(CurrencyFactoryTests.hex2a(assetMetadataAfter.unwrap().symbol.inner)).to.be.equal(metadata.symbol.inner.toString());
        });
    });
    describe("tx.currencyFactory.addRange", function () {
        it("Sudo can add new currency ID range: ", async function () {
            this.timeout(2 * 60 * 1000);
            const range = 100;
            (0, chai_1.expect)(range).to.be.greaterThan(0);
            const assetIdRangesBefore = await api.query.currencyFactory.assetIdRanges();
            const rangeLengthBefore = assetIdRangesBefore.ranges.length;
            const lastAssetIdRangeBefore = assetIdRangesBefore.ranges[rangeLengthBefore - 1];
            const { data: [result] } = await CurrencyFactoryTests.addRange(api, sudoKey, range);
            (0, chai_1.expect)(result.isOk).to.be.true;
            const assetIdRangesAfter = await api.query.currencyFactory.assetIdRanges();
            const rangeLengthAfter = assetIdRangesAfter.ranges.length;
            const lastAssetIdRangeAfter = assetIdRangesAfter.ranges[rangeLengthAfter - 1];
            (0, chai_1.expect)(rangeLengthAfter).to.be.greaterThan(rangeLengthBefore);
            (0, chai_1.expect)(lastAssetIdRangeAfter.current).to.be.bignumber.equal(lastAssetIdRangeBefore.end.addn(1));
        });
    });
});
class CurrencyFactoryTests {
    static async setMetadata(api, sudoKey, assetId, metadata) {
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.currencyFactory.setMetadata(assetId, metadata)));
    }
    static async addRange(api, sudoKey, range) {
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.currencyFactory.addRange(range)));
    }
    static async initializeNewAsset(api, sudoKey, amount, beneficiary) {
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.tokens.Endowed.is, api.tx.sudo.sudo(api.tx.assets.mintInitialize(amount, beneficiary)));
    }
    /**
     * Converts hex to ascii.
     * Source: https://stackoverflow.com/questions/3745666/how-to-convert-from-hex-to-ascii-in-javascript/3745677#3745677
     * @param hexx
     */
    static hex2a(hexx) {
        const hex = hexx.toString().replace("0x", ""); //force conversion
        let str = "";
        for (let i = 0; i < hex.length; i += 2)
            str += String.fromCharCode(parseInt(hex.substr(i, 2), 16));
        return str;
    }
}
exports.CurrencyFactoryTests = CurrencyFactoryTests;
//# sourceMappingURL=currencyFactoryTests.js.map