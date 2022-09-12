"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.TxCrowdloanRewardsTests = exports.ethAccount = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
const contributions_json_1 = require("@composabletests/tests/crowdloanRewards/contributions.json");
const chai_1 = require("chai");
const web3_1 = __importDefault(require("web3"));
const toHexString = bytes => Array.prototype.map.call(bytes, x => ("0" + (x & 0xff).toString(16)).slice(-2)).join("");
// The prefix is defined as pallet config
const proofMessage = (account, isEth = false) => (isEth ? "picasso-" : "<Bytes>picasso-") + toHexString(account.publicKey) + (isEth ? "" : "</Bytes>");
const ethAccount = (seed) => new web3_1.default().eth.accounts.privateKeyToAccount("0x" + seed.toString(16).padStart(64, "0"));
exports.ethAccount = ethAccount;
class TxCrowdloanRewardsTests {
    /**
     * Providing the crowdloan pallet with funds
     *
     * Unfortunately we can't directly mint into the pallet therefore,
     * we mint into the Alice wallet and transfer funds from there.
     *
     * @param {ApiPromise} api Connected API Client.
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param amount
     */
    static async beforeCrowdloanTestsProvideFunds(api, sudoKey, amount) {
        const palletPublicKey = api.consts.crowdloanRewards.accountId;
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.balances.Transfer.is, api.tx.balances.transfer(palletPublicKey, amount));
    }
    /**
     * tx.crowdloanRewards.initialize
     *
     * @param {ApiPromise} api Connected API Client.
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     */
    static txCrowdloanRewardsInitializeTest(api, sudoKey) {
        return (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.crowdloanRewards.initialize()));
    }
    /**
     * tx.crowdloanRewards.populate
     *
     * @param {ApiPromise} api Connected API Client.
     * @param {Web3} web3 Web3 Object, to be received using `connectionHelper.getNewConnection()`
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param testContributorWallet KSM Wallet of contributor to populate with.
     */
    static async txCrowdloanRewardsPopulateTest(api, sudoKey, testContributorWallet) {
        const vesting48weeks = api.createType("u32", 100800);
        let contributors = [];
        // Before we go through all the contributors, we inject our test wallet at the very beginning.
        const testContributorReward = api.createType("u128", 1000000000000);
        const testContriborRelayChainObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
            RelayChain: testContributorWallet.publicKey
        });
        const testContributorEthChainObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
            Ethereum: (0, exports.ethAccount)(1).address
        });
        contributors.push([testContriborRelayChainObject, testContributorReward, vesting48weeks]);
        contributors.push([testContributorEthChainObject, testContributorReward, vesting48weeks]);
        // Iterating through our list of contributors
        let i = 0;
        let amount = testContributorReward.toNumber() * 2;
        for (const [key, value] of Object.entries(contributions_json_1.shares)) {
            let remoteAccountObject;
            // Creating either an ethereum or ksm contributor object.
            if (key.startsWith("0x"))
                remoteAccountObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", { Ethereum: key });
            else
                remoteAccountObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
                    RelayChain: api.createType("AccountId32", key)
                });
            // Preparing our contributor object and adding it to the list of contributors to be populated.
            // This should be (value * 10^8) if I'm correct. But this lead to integer overflows.
            const currentContributorAmount = parseInt((parseFloat(value) * Math.pow(10, 6)).toFixed(0));
            amount += currentContributorAmount;
            contributors.push([remoteAccountObject, api.createType("u128", currentContributorAmount), vesting48weeks]);
            // Every 2500th iteration we send our list of contributors, else we'd break the block data size limit.
            if (i % 2500 == 0 && i != 0) {
                // Providing funds since calling `populate` verifies that the pallet funds are equal to contributor amount.
                const { data: [provideFundsResult] } = await TxCrowdloanRewardsTests.beforeCrowdloanTestsProvideFunds(api, sudoKey, api.createType("u128", amount));
                (0, chai_1.expect)(provideFundsResult).to.not.be.undefined;
                // Actual population step.
                const { data: [result] } = await TxCrowdloanRewardsTests.txCrowdloanRewardsPopulateTestHandler(api, sudoKey, contributors);
                (0, chai_1.expect)(result.isOk).to.be.true;
                amount = 0;
                contributors = [];
            }
            i++;
        }
        return testContriborRelayChainObject;
    }
    /**
     * tx.crowdloanRewards.populate
     *
     * @param {KeyringPair} sudoKey Wallet with sudo rights.
     * @param {KeyringPair} contributors List of contributors to be transacted.
     */
    static async txCrowdloanRewardsPopulateTestHandler(api, sudoKey, contributors) {
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.crowdloanRewards.populate(contributors)));
    }
    /**
     * tx.crowdloanRewards.associate RelayChain
     *
     * @param {KeyringPair} contributor The contributor relay chain wallet public key.
     * @param {KeyringPair} contributorRewardAccount The wallet the contributor wants to receive their PICA to.
     */
    static async txCrowdloanRewardsRelayAssociateTests(api, contributor, contributorRewardAccount) {
        // arbitrary, user defined reward account
        const proof = contributor.sign(proofMessage(contributorRewardAccount));
        return await (0, polkadotjs_1.sendUnsignedAndWaitForSuccess)(api, api.events.crowdloanRewards.Associated.is, api.tx.crowdloanRewards.associate(contributorRewardAccount.publicKey, api.createType("PalletCrowdloanRewardsModelsProof", { RelayChain: [contributor.publicKey, { Sr25519: proof }] })));
    }
    /**
     * tx.crowdloanRewards.associate ETH Chain
     *
     * @param {KeyringPair} contributor The contributor ETH chain wallet public key.
     * @param {KeyringPair} contributorRewardAccount The wallet the contributor wants to receive their PICA to.
     */
    static async txCrowdloanRewardsEthAssociateTest(api, contributor, contributorRewardAccount) {
        const proof = contributor.sign(proofMessage(contributorRewardAccount, true));
        return await (0, polkadotjs_1.sendUnsignedAndWaitForSuccess)(api, api.events.crowdloanRewards.Associated.is, api.tx.crowdloanRewards.associate(contributorRewardAccount.publicKey, api.createType("PalletCrowdloanRewardsModelsProof", { Ethereum: proof.signature })));
    }
    /**
     * tx.crowdloanRewards.claim
     *
     * @param { KeyringPair } wallet The reward account which tries to claim.
     */
    static async txCrowdloanRewardsClaimTest(api, wallet) {
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.crowdloanRewards.Claimed.is, api.tx.crowdloanRewards.claim());
    }
}
exports.TxCrowdloanRewardsTests = TxCrowdloanRewardsTests;
//# sourceMappingURL=crowdloanHandler.js.map