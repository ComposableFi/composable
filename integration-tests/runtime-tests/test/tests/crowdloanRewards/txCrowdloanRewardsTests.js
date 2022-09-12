"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const chai_1 = require("chai");
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
const crowdloanHandler_1 = require("@composabletests/tests/crowdloanRewards/testHandlers/crowdloanHandler");
const mintingHelper_1 = require("@composable/utils/mintingHelper");
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const walletHelper_1 = require("@composable/utils/walletHelper");
/**
 * Task order list:
 *  1. Provide funds to crowdloan pallet
 *  2. Populate the list of contributors
 *  3. Initialize the crowdloan
 *  4. Associate a picassso account (which also claims)
 *  5. Claiming more rewards.
 */
describe("CrowdloanRewards Tests", function () {
    if (!test_configuration_json_1.default.enabledTests.tx.enabled)
        return;
    let api;
    let walletCharlie, sudoKey, contributor, contributorRewardAccount, contributorEth, contributorEthRewardAccount;
    let onExistingChain = false;
    /**
     * We mainly set up some variables here.
     *
     * We also identify if this chain had already tests run on it.
     * And if so, we skip the populate() and initialize() tests.
     */
    before("Setting up tests", async function () {
        this.timeout(60 * 1000);
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletAlice, devWalletCharlie } = (0, walletHelper_1.getDevWallets)(newKeyring);
        walletCharlie = devWalletCharlie;
        sudoKey = devWalletAlice;
        let associationExisting = true;
        let i = 1;
        while (associationExisting) {
            contributor = devWalletCharlie.derive("/contributor-" + i);
            contributorEth = (0, crowdloanHandler_1.ethAccount)(i);
            // arbitrary, user defined reward account
            contributorRewardAccount = contributor.derive("/reward");
            contributorEthRewardAccount = devWalletCharlie.derive("/reward-eth-" + i);
            const existingAssociations = await api.query.crowdloanRewards.associations(contributorRewardAccount.publicKey);
            if (existingAssociations.toString() == "") {
                associationExisting = false;
            }
            else {
                onExistingChain = true;
            }
            i++;
        }
        if (onExistingChain)
            console.info("tx.crowdloanRewards Tests: Detected already configured chain! " + "Skipping populate() & initialize().");
    });
    before("Providing funds to Alice & our imaginary contributor wallet", async function () {
        if (!test_configuration_json_1.default.enabledTests.tx.setup.provideAssets)
            this.skip();
        // 2 minutes timeout
        this.timeout(60 * 2 * 1000);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, sudoKey, sudoKey, [1]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, contributorRewardAccount, sudoKey, [1]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, contributorEthRewardAccount, sudoKey, [1]);
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    /**
     * Here we populate the crowdloan pallet with a random generated list of contributors.
     *
     * This is a SUDO call! Though the checks are within the handler function, due to multiple transaction being sent
     * within a for loop.
     */
    it("Can populate the list of contributors", async function () {
        if (!test_configuration_json_1.default.enabledTests.tx.populate_success.populate1 || onExistingChain)
            this.skip();
        // 5 minutes timeout
        this.timeout(5 * 60 * 1000);
        const testContributorRelayChainObject = await crowdloanHandler_1.TxCrowdloanRewardsTests.txCrowdloanRewardsPopulateTest(api, sudoKey, contributor);
        (0, chai_1.expect)(testContributorRelayChainObject).to.not.be.an("Error");
    });
    /**
     * Here we initialize the crowdloan with our populated list of contributors.
     *
     * This is a SUDO call and is checked with `.isOk`.
     */
    it("Can initialize the crowdloan", async function () {
        if (!test_configuration_json_1.default.enabledTests.tx.initialize_success.initialize1 || onExistingChain)
            this.skip();
        // 2 minutes timeout
        this.timeout(60 * 2 * 1000);
        const { data: [result] } = await crowdloanHandler_1.TxCrowdloanRewardsTests.txCrowdloanRewardsInitializeTest(api, sudoKey);
        (0, chai_1.expect)(result.isOk).to.be.true;
    });
    /***
     * Here we associate our picasso account with our ETH & RelayChain wallets.
     *
     * Here we send 2 transactions at the same time, therefore we have 2 results,
     * though with the exact same result structure.
     *
     * Results:
     * 1. The public key of the remote wallet.
     * 2. The public key of the transacting wallet.
     */
    it("Can associate a picasso account", async function () {
        if (!test_configuration_json_1.default.enabledTests.tx.associate_success.associate1)
            this.skip();
        // 2 minutes timeout
        this.timeout(60 * 20 * 1000);
        await Promise.all([
            crowdloanHandler_1.TxCrowdloanRewardsTests.txCrowdloanRewardsEthAssociateTest(api, contributorEth, contributorEthRewardAccount),
            crowdloanHandler_1.TxCrowdloanRewardsTests.txCrowdloanRewardsRelayAssociateTests(api, contributor, contributorRewardAccount)
        ]).then(function ([{ data: [result1Request1, result2Request1] }, { data: [result1Request2, result2Request2] }]) {
            (0, chai_1.expect)(result1Request1).to.not.be.an("Error");
            (0, chai_1.expect)(result1Request2).to.not.be.an("Error");
            (0, chai_1.expect)(result2Request1.toString()).to.be.equal(api.createType("AccountId32", contributorEthRewardAccount.publicKey).toString());
            (0, chai_1.expect)(result2Request2.toString()).to.be.equal(api.createType("AccountId32", contributorRewardAccount.publicKey).toString());
        });
    });
    /**
     * Can we finally claim the crowdloan reward?
     * We're gonna find out!
     *
     * Results are:
     * 1. The public key of the remote account.
     * 2. The public key of the transacting wallet.
     * 3. The claimed amount.
     */
    it("KSM contributor can claim the crowdloan reward", async function () {
        if (!test_configuration_json_1.default.enabledTests.tx.claim_success.claim1 || onExistingChain)
            this.skip();
        // 2 minutes timeout
        this.timeout(60 * 2 * 1000);
        const { data: [resultRemoteAccountId, resultAccountId, resultClaimedAmount] } = await crowdloanHandler_1.TxCrowdloanRewardsTests.txCrowdloanRewardsClaimTest(api, contributorRewardAccount);
        (0, chai_1.expect)(resultRemoteAccountId).to.not.be.an("Error");
        (0, chai_1.expect)(resultClaimedAmount).to.be.a.bignumber;
        (0, chai_1.expect)(resultClaimedAmount.toNumber()).to.be.greaterThan(0);
        (0, chai_1.expect)(resultAccountId.toString()).to.be.equal(api.createType("AccountId32", contributorRewardAccount.publicKey).toString());
    });
    it("ETH contributor can claim the crowdloan reward", async function () {
        if (!test_configuration_json_1.default.enabledTests.tx.claim_success.claim1 || onExistingChain)
            this.skip();
        // 2 minutes timeout
        this.timeout(60 * 2 * 1000);
        const { data: [resultRemoteAccountId, resultAccountId, resultClaimedAmount] } = await crowdloanHandler_1.TxCrowdloanRewardsTests.txCrowdloanRewardsClaimTest(api, contributorEthRewardAccount);
        (0, chai_1.expect)(resultRemoteAccountId).to.not.be.an("Error");
        (0, chai_1.expect)(resultClaimedAmount).to.be.a.bignumber;
        (0, chai_1.expect)(resultAccountId.toString()).to.be.equal(api.createType("AccountId32", contributorEthRewardAccount.publicKey).toString());
    });
    describe("Crowdloan Failure Tests", function () {
        if (!test_configuration_json_1.default.enabledTests.tx.failure_tests.enabled)
            return;
        /***
         * Here we try to re- associate the same contributor wallets,
         * even though they're already associated.
         */
        it("Can not re- associate the same picasso account", async function () {
            if (!test_configuration_json_1.default.enabledTests.tx.failure_tests.associate_failure.associate1)
                this.skip();
            // 2 minutes timeout
            this.timeout(60 * 2 * 1000);
            await Promise.all([
                crowdloanHandler_1.TxCrowdloanRewardsTests.txCrowdloanRewardsEthAssociateTest(api, contributorEth, walletCharlie),
                crowdloanHandler_1.TxCrowdloanRewardsTests.txCrowdloanRewardsRelayAssociateTests(api, contributor, walletCharlie)
            ]).then(function ([{ data: [result] }, { data: [result2] }]) {
                (0, chai_1.expect)(result).to.be.an("Error");
                (0, chai_1.expect)(result2).to.be.an("Error");
            });
        });
    });
});
//# sourceMappingURL=txCrowdloanRewardsTests.js.map