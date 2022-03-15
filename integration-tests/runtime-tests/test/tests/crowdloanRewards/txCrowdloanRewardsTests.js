"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.TxCrowdloanRewardsTests = void 0;
const ramda_1 = __importDefault(require("ramda"));
const chai_1 = require("chai");
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
const polkadotjs_1 = require("@composable/utils/polkadotjs");
const toHexString = bytes => Array.prototype.map.call(bytes, x => ('0' + (x & 0xFF).toString(16)).slice(-2)).join('');
// The prefix is defined as pallet config
const proofMessage = (account, isEth = false) => (isEth ? "picasso-" : "<Bytes>picasso-") + toHexString(account.publicKey) + (isEth ? "" : "</Bytes>");
const ethAccount = (seed) => web3.eth.accounts.privateKeyToAccount("0x" + seed.toString(16).padStart(64, '0'));
describe('CrowdloanRewards Tests', function () {
    if (!test_configuration_json_1.default.enabledTests.tx.enabled)
        return;
    let wallet, sudoKey, contributor, contributorRewardAccount, contributorEth, contributorEthRewardAccount;
    let onExistingChain = false;
    /**
     * We identify if this chain had already tests run on it.
     * And if so, we skip the populate() and initialize() tests.
     */
    before(async function () {
        sudoKey = walletAlice;
        wallet = walletAlice;
        let associationExisting = true;
        let i = 1;
        while (associationExisting) {
            contributor = wallet.derive("/contributor-" + i);
            contributorEth = ethAccount(i);
            // arbitrary, user defined reward account
            contributorRewardAccount = contributor.derive("/reward");
            contributorEthRewardAccount = wallet.derive("/reward-eth-" + i);
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
            console.info("tx.crowdloanRewards Tests: Detected already configured chain! " +
                "Skipping populate() & initialize().");
    });
    // 2 minutes timeout
    this.timeout(60 * 2 * 1000);
    it('Can populate the list of contributors', async function () {
        if (!test_configuration_json_1.default.enabledTests.tx.populate_success.populate1 || onExistingChain)
            this.skip();
        const { data: [result], } = await TxCrowdloanRewardsTests.txCrowdloanRewardsPopulateTest(sudoKey);
        (0, chai_1.expect)(result.isOk).to.be.true;
    });
    it('Can initialize the crowdloan', async function () {
        if (!test_configuration_json_1.default.enabledTests.tx.initialize_success.initialize1 || onExistingChain)
            this.skip();
        const { data: [result], } = await TxCrowdloanRewardsTests.txCrowdloanRewardsInitializeTest(sudoKey);
        (0, chai_1.expect)(result.isOk).to.be.true;
    });
    it('Can associate a picasso account', async function () {
        if (!test_configuration_json_1.default.enabledTests.tx.associate_success.associate1)
            this.skip();
        await Promise.all([
            TxCrowdloanRewardsTests.txCrowdloanRewardsEthAssociateTests(wallet, contributorEth, contributorEthRewardAccount),
            TxCrowdloanRewardsTests.txCrowdloanRewardsRelayAssociateTests(wallet, contributor, contributorRewardAccount),
        ]).then(function (result) {
            (0, chai_1.expect)(result[0].data[1].toString()).to.be
                .equal(api.createType('AccountId32', contributorEthRewardAccount.publicKey).toString());
            (0, chai_1.expect)(result[1].data[1].toString()).to.be
                .equal(api.createType('AccountId32', contributorRewardAccount.publicKey).toString());
        });
    });
});
class TxCrowdloanRewardsTests {
    /**
     * Task order list:
     *  * Populate the list of contributors
     *  * Initialize the crowdloan
     *  * Associate a picassso account
     */
    /**
     * tx.crowdloanRewards.initialize
     */
    static txCrowdloanRewardsInitializeTest(sudoKey) {
        return (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.crowdloanRewards.initialize()));
    }
    /**
     * tx.crowdloanRewards.populate
     */
    static async txCrowdloanRewardsPopulateTest(sudoKey) {
        const vesting48weeks = api.createType('u32', 100800);
        const reward = api.createType('u128', 1000000000000);
        const relay_accounts = ramda_1.default.unfold(n => n > 50 ? false : [[
                api.createType('PalletCrowdloanRewardsModelsRemoteAccount', { RelayChain: walletAlice.derive("/contributor-" + n.toString()).publicKey }),
                reward,
                vesting48weeks,
            ], n + 1], 1);
        const eth_accounts = ramda_1.default.unfold(n => n > 50 ? false : [[
                api.createType('PalletCrowdloanRewardsModelsRemoteAccount', { Ethereum: ethAccount(n).address }),
                reward,
                vesting48weeks,
            ], n + 1], 1);
        const accounts = relay_accounts.concat(eth_accounts);
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.crowdloanRewards.populate(accounts)));
    }
    /**
     * tx.crowdloanRewards.associate RelayChain
     * @param { KeyringPair } wallet
     * @param contributor
     * @param contributorRewardAccount
     */
    static async txCrowdloanRewardsRelayAssociateTests(wallet, contributor, contributorRewardAccount) {
        // arbitrary, user defined reward account
        const proof = contributor.sign(proofMessage(contributorRewardAccount));
        return await (0, polkadotjs_1.sendUnsignedAndWaitForSuccess)(api, api.events.crowdloanRewards.Associated.is, api.tx.crowdloanRewards.associate(contributorRewardAccount.publicKey, { RelayChain: [contributor.publicKey, { Sr25519: proof }] }));
    }
    /**
     * tx.crowdloanRewards.associate ETH Chain
     * @param { KeyringPair } wallet
     * @param contributor
     * @param contributorRewardAccount
     */
    static async txCrowdloanRewardsEthAssociateTests(wallet, contributor, contributorRewardAccount) {
        const proof = contributor.sign(proofMessage(contributorRewardAccount, true));
        return await (0, polkadotjs_1.sendUnsignedAndWaitForSuccess)(api, api.events.crowdloanRewards.Associated.is, api.tx.crowdloanRewards.associate(contributorRewardAccount.publicKey, { Ethereum: proof.signature }));
    }
}
exports.TxCrowdloanRewardsTests = TxCrowdloanRewardsTests;
