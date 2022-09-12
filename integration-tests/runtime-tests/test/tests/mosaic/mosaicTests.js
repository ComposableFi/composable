"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const chai_1 = require("chai");
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
const mintingHelper_1 = require("@composable/utils/mintingHelper");
const bn_js_1 = __importDefault(require("bn.js"));
const mosaicTestHelper_1 = require("@composabletests/tests/mosaic/testHandlers/mosaicTestHelper");
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const walletHelper_1 = require("@composable/utils/walletHelper");
const polkadotjs_1 = require("@composable/utils/polkadotjs");
/**
 * Mosaic Pallet Tests
 *  Checked functionalities are as follows;
 *  1. setRelayer
 *  2. rotateRelayer
 *  3. setNetwork
 *  4. setBudget
 *  5. transferTo
 *  6. acceptTransfer
 *  7. claimStaleTo
 *  8. timelockedMint
 *  9. setTimelockDuration
 * 10. rescindTimelockedMint
 * 11. claimTo
 * 12. updateAssetMapping
 *
 * This suite consists of happy path tests. Additionally, we started implementing suites for later references such as regression, smoke etc.
 *
 */
describe("tx.mosaic Tests", function () {
    // Check if group of tests are enabled.
    if (!test_configuration_json_1.default.enabledTests.query.enabled)
        return;
    let api;
    let sudoKey, startRelayerWallet, newRelayerWallet, userWallet, remoteAssetId;
    let transferAmount, assetId, networkId;
    let pNetworkId;
    let ethAddress;
    describe("tx.mosaic Tests", function () {
        this.timeout(4 * 60 * 1000);
        if (!test_configuration_json_1.default.enabledTests.query.account__success.enabled)
            return;
        before("Setting up the tests", async function () {
            this.timeout(4 * 60 * 1000);
            const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
            api = newClient;
            const { devWalletAlice, devWalletEve, devWalletFerdie } = (0, walletHelper_1.getDevWallets)(newKeyring);
            sudoKey = devWalletAlice;
            startRelayerWallet = devWalletEve.derive("/tests/mosaicPallets/wallet1");
            newRelayerWallet = devWalletAlice.derive("/tests/mosaicPallets/wallet2");
            userWallet = devWalletFerdie.derive("/tests/mosaicPallets/wallet3");
            assetId = 4;
            transferAmount = 100000000000;
            networkId = 1;
            pNetworkId = api.createType("u128", 1);
            ethAddress = "0x";
            remoteAssetId = api.createType("CommonMosaicRemoteAssetId", {
                EthereumTokenAddress: api.createType("[u8;20]", "0x")
            });
        });
        before("Mint available assets into wallets", async function () {
            this.timeout(5 * 60 * 1000);
            await (0, mintingHelper_1.mintAssetsToWallet)(api, startRelayerWallet, sudoKey, [1, 4]);
            await (0, mintingHelper_1.mintAssetsToWallet)(api, newRelayerWallet, sudoKey, [1, 4]);
            await (0, mintingHelper_1.mintAssetsToWallet)(api, userWallet, sudoKey, [1, 4]);
        });
        after("Closing the connection", async function () {
            await api.disconnect();
        });
        /**
         * Setting the first relayer.
         * Sudo call therefore result is checked by `.isOk`.
         */
        it("Should be able to set relayer @integration", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.query.account__success.balanceGTZero1)
                this.skip();
            const { data: [result] } = await mosaicTestHelper_1.TxMosaicTests.testSetRelayer(api, sudoKey, startRelayerWallet.address);
            (0, chai_1.expect)(result.isOk).to.be.true;
        });
        /**
         * Setting the network.
         */
        it("Should be able to set the network @integration", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.query.account__success.balanceGTZero1)
                this.skip();
            const networkInfo = api.createType("PalletMosaicNetworkInfo", {
                enabled: api.createType("bool", true),
                minTransferSize: api.createType("u128", 0),
                maxTransferSize: api.createType("u128", 800000000000)
            });
            const { data: [retNetworkId, retNetworkInfo] } = await mosaicTestHelper_1.TxMosaicTests.testSetNetwork(api, startRelayerWallet, pNetworkId, networkInfo);
            (0, chai_1.expect)(retNetworkId).to.not.be.an("Error");
            (0, chai_1.expect)(retNetworkInfo).to.not.be.an("Error");
            //Verifies the newly created networkId
            (0, chai_1.expect)(retNetworkId.toNumber()).to.be.equal(networkId);
        });
        /**
         * Setting the budget.
         * A sudo call therefore result is verified by isOk.
         */
        it("Should be able set the budget", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.query.account__success.balanceGTZero1)
                this.skip();
            const transAmount = 800000000000000;
            const pDecay = api.createType("PalletMosaicDecayBudgetPenaltyDecayer", {
                Linear: api.createType("PalletMosaicDecayLinearDecay", { factor: api.createType("u128", 5) })
            });
            const { data: [result] } = await mosaicTestHelper_1.TxMosaicTests.testSetBudget(api, sudoKey, assetId, transAmount, pDecay);
            (0, chai_1.expect)(result.isOk).to.be.true;
        });
        it("Should be able to update asset mapping", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.updateAssetMapping)
                this.skip();
            const { data: [result] } = await mosaicTestHelper_1.TxMosaicTests.testUpdateAssetMaping(api, sudoKey, assetId, pNetworkId, remoteAssetId);
            (0, chai_1.expect)(result.isOk).to.be.true;
        });
        it("Should be able to send transfers to another network, creating an outgoing transaction", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.sendTransferTo)
                this.skip();
            const { data: [result] } = await mosaicTestHelper_1.TxMosaicTests.testTransferTo(api, startRelayerWallet, pNetworkId, assetId, ethAddress, transferAmount);
            (0, chai_1.expect)(result).to.not.be.an("Error");
            const lockedAmount = await api.query.mosaic.outgoingTransactions(startRelayerWallet.address, assetId);
            //verify that the amount sent is locked in the outgoing pool.
            (0, chai_1.expect)(lockedAmount.unwrap()[0].toNumber()).to.be.equal(transferAmount);
        });
        it("Relayer should be able to mint assets into pallet wallet with timelock//simulates incoming transfer from pair network", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.relayerCanMintAssets)
                this.skip();
            const toTransfer = newRelayerWallet.address;
            await mosaicTestHelper_1.TxMosaicTests.lockFunds(api, startRelayerWallet, pNetworkId, remoteAssetId, toTransfer, transferAmount);
            const lockedAmount = await api.query.mosaic.incomingTransactions(toTransfer, assetId);
            //verify that the incoming transaction is locked in the incoming transaction pool.
            (0, chai_1.expect)(lockedAmount.unwrap()[0].toNumber()).to.be.equal(transferAmount);
        });
        it("Other users should be able to mint assets into pallet wallet with timelock//simulates incoming transfer from pair network", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.userCanMintAssets)
                this.skip();
            const toTransfer = userWallet.address;
            await mosaicTestHelper_1.TxMosaicTests.lockFunds(api, startRelayerWallet, pNetworkId, remoteAssetId, toTransfer, transferAmount);
            const lockedAmount = await api.query.mosaic.incomingTransactions(toTransfer, assetId);
            //verify that the incoming transaction is locked in the incoming transaction pool.
            (0, chai_1.expect)(lockedAmount.unwrap()[0].toNumber()).to.be.equal(transferAmount);
        });
        it("Only relayer should mint assets into pallet wallet with timelock/incoming transactions (Failure Test)", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.OnlyRelayerCanMintAssets)
                this.skip();
            const toTransfer = newRelayerWallet.address;
            //verify that the transaction fails with BadOrigin message
            await mosaicTestHelper_1.TxMosaicTests.lockFunds(api, userWallet, pNetworkId, remoteAssetId, toTransfer, transferAmount).catch(error => (0, chai_1.expect)(error.message).to.contain("BadOrigin"));
        });
        /**
         * Rotating the relayer.
         * Sudo call therefore result is checked by `.isOk`.
         */
        it("Should be able to rotate relayer", async function () {
            if (!test_configuration_json_1.default.enabledTests.query.account__success.balanceGTZero1)
                this.skip();
            const { data: [result] } = await mosaicTestHelper_1.TxMosaicTests.testRotateRelayer(api, startRelayerWallet, newRelayerWallet.address);
            (0, chai_1.expect)(result).to.not.be.an("Error");
            const relayerInfo = await api.query.mosaic.relayer();
            //verify that the relayer records information about the next relayer wallet
            (0, chai_1.expect)(relayerInfo.unwrap().relayer.next.toJSON().account).to.be.equal(api.createType("AccountId32", newRelayerWallet.address).toString());
        });
        it("Should the finality issues occur, relayer can burn untrusted amounts from tx", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.relayerCanRescindTimeLockFunds)
                this.skip();
            const wallet = startRelayerWallet;
            const returnWallet = newRelayerWallet;
            const { data: [result] } = await mosaicTestHelper_1.TxMosaicTests.testRescindTimeLockedFunds(api, wallet, returnWallet, remoteAssetId, transferAmount);
            //We can change the assertion, get the info from chain from incoming pool and verify that the amount locked is reduced from the amount total
            (0, chai_1.expect)(result.toString()).to.be.equal(api.createType("AccountId32", newRelayerWallet.address).toString());
        });
        it("Only relayer should be able to burn untrusted amounts from incoming tx (Failure Test)", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.OnlyRelayerCanRescindTimeLockFunds)
                this.skip();
            const wallet = userWallet;
            const returnWallet = newRelayerWallet;
            await mosaicTestHelper_1.TxMosaicTests.testRescindTimeLockedFunds(api, wallet, returnWallet, remoteAssetId, transferAmount).catch(error => (0, chai_1.expect)(error.message).to.contain("BadOrigin"));
        });
        it("Other users should be able to send transfers to another network, creating an outgoing transaction", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.userCanCreateOutgoingTransaction)
                this.skip();
            const paramRemoteTokenContAdd = "0x0423276a1da214B094D54386a1Fb8489A9d32730";
            const { data: [result] } = await mosaicTestHelper_1.TxMosaicTests.testTransferTo(api, userWallet, pNetworkId, assetId, paramRemoteTokenContAdd, transferAmount);
            (0, chai_1.expect)(result).to.not.be.an("Error");
            const lockedAmount = await api.query.mosaic.outgoingTransactions(userWallet.address, assetId);
            //Verify that the transferred amount is locked in the outgoing transaction pool.
            (0, chai_1.expect)(lockedAmount.unwrap()[0].toNumber()).to.be.equal(transferAmount);
        });
        it("Relayer should be able to accept outgoing transfer", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.relayerAcceptTransfer)
                this.skip();
            this.timeout(2 * 60 * 1000);
            const senderWallet = userWallet;
            const { data: [result] } = await mosaicTestHelper_1.TxMosaicTests.testAcceptTransfer(api, startRelayerWallet, senderWallet, pNetworkId, remoteAssetId, transferAmount);
            //verify that the relayer address is returned.
            (0, chai_1.expect)(result.toString()).to.be.equal(api.createType("AccountId32", senderWallet.address).toString());
        });
        it("Only receiver should be able to claim incoming transfers (Failure Test)", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.OnlyReceiverCanClaimTransaction)
                this.skip();
            this.timeout(2 * 60 * 1000);
            const receiverWallet = startRelayerWallet;
            await mosaicTestHelper_1.TxMosaicTests.testClaimTransactions(api, receiverWallet, receiverWallet, assetId).catch(error => {
                (0, chai_1.expect)(error.message).to.contain("NoClaimable");
            });
        });
        it("Receiver should be able to claim incoming transfers", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.receiverCanClaimTransaction)
                this.skip();
            this.timeout(2 * 60 * 1000);
            const receiverWallet = userWallet;
            const initialTokens = await api.query.tokens.accounts(userWallet.address, assetId);
            const { data: [result] } = await mosaicTestHelper_1.TxMosaicTests.testClaimTransactions(api, userWallet, receiverWallet, assetId);
            (0, chai_1.expect)(result).to.not.be.an("Error");
            const afterTokens = await api.query.tokens.accounts(userWallet.address, assetId);
            (0, chai_1.expect)(new bn_js_1.default(initialTokens.free).eq(new bn_js_1.default(afterTokens.free).sub(new bn_js_1.default(transferAmount)))).to.be.true;
        });
        it("User should be able to reclaim the stale funds not accepted by the relayer and locked in outgoing transactions pool", async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.userCanClaimStaleFunds)
                this.skip();
            this.timeout(5 * 60 * 1000);
            const wallet = startRelayerWallet;
            const initialTokens = await api.query.tokens.accounts(wallet.address, assetId);
            let retry = true;
            let finalResult;
            while (retry) {
                await (0, polkadotjs_1.waitForBlocks)(api, 2);
                const { data: [result] } = await mosaicTestHelper_1.TxMosaicTests.testClaimStaleFunds(api, startRelayerWallet, assetId).catch(error => {
                    if (error.message.includes("TxStillLocked"))
                        return { data: ["Retrying..."] };
                });
                if (result !== "Retrying...") {
                    retry = false;
                    finalResult = result;
                }
            }
            (0, chai_1.expect)(finalResult).to.not.be.an("Error");
            const afterTokens = await api.query.tokens.accounts(wallet.address, assetId);
            //verify that the reclaimed tokens are transferred into user balance.
            (0, chai_1.expect)(new bn_js_1.default(initialTokens.free).eq(new bn_js_1.default(afterTokens.free).sub(new bn_js_1.default(transferAmount)))).to.be.true;
        });
    });
});
//# sourceMappingURL=mosaicTests.js.map