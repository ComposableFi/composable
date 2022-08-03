import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import BN from "bn.js";
import exp from "constants";

const getProposalHash = (api: ApiPromise, proposal) => {
  return null;
};

/**
 * Democracy Test Suite
 *
 * Democracy Extrinsics & Queries
 *
 * Queries:
 *
 * - Blacklist(H256) : Option<(u32, Vec<AccountId32>)>
 * - Cancellations(H256) : bool
 * - depositOf(u32) : Option<Vec<AccountId32>, u128)>
 * - lastTableWasExternal : bool
 * - lowestUnbaked : u32
 * - nextExternal : Option<(H256, PalletDemocracyVoteThreshold)>
 * - preimages(H256) : Option<PalletDemocracyPreimageStatus>
 * - publicPropCount : u32
 * - publicProps : Vec<(u32, H256, AccountId32)>
 * - referendumCount : u32
 * - referendumInfoOf(u32) : Option<PalletDemocracyReferendumInfo>
 * - storageVersion : Option<PalletDemocracyReleases>
 * - votingOf(AccountId32) : PalletDemocracyVoteVoting
 *
 * Extrinsics:
 *
 * - blacklist(proposalHash, maybeRefIndex)
 * - cancelProposal(propIndex)
 * - cancelQueued(which)
 * - cancelReferendum(refIndex)
 * - clearPublicProposals
 * - delegate(to, conviction, balance)
 * - emergencyCancel(refIndex)
 * - enactProposal(proposalHash, index)
 * - externalPropose(proposalHash)
 * - externalProposeDefault(proposalHash)
 * - externalProposeMajority(proposalHash)
 * - fastTrack(proposalHash, votingPeriod, delay)
 * - noteImminentPreimage(encodedProposal
 * - noteImminentPreimageOperational(encodedProposal)
 * - notePreimage(encodedProposal)
 * - notePreimageOperational(encodedProposal)
 * - propose(proposalHash, value)
 * - reapPreimage(proposalHash, proposalLenUpperBound)
 * - removeOtherVote(target, index)
 * - removeVote(index)
 * - second(proposal, secondsUpperBound)
 * - Undelegate
 * - unlock(target)
 * - vetoExternal(proposalHash)
 * - vote(refIndex, vote)
 *
 *
 *
 * 1. Submit a preimage of the extrinsic we want to propose.
 * 2. Submit proposal for the submitted preimage.
 * 3. Endorse proposal using another wallet.
 * ...
 *
 * 4. Vote for the proposal.
 */
describe.only("[SHORT] Democracy Tests", function () {
  if (!testConfiguration.enabledTests.query.enabled) return;

  let api: ApiPromise;
  let walletAlice: KeyringPair, walletBob: KeyringPair, walletCharlie: KeyringPair; // ToDo: Change wallets!
  let proposalId1: number, // Created during propose test. Will stay a proposal.
    proposalId2: number; // Created during `fastTrack` test. Will get into voting.

  // Image proposes to mint 999_999_999_999 PICA into Alice. Used for all testing proposals.
  // ToDo: Change to hashing a function directly for better readability.
  const proposalHashOne = "0xcbf6e77875268d88955338afb3538de93bb8cdb0caebbec60e9a0936cb1076b4";

  // Propsal to mint 999_999_999_999 PICA into Bob. Used for blacklisting.
  const proposalToBlackList = "0x8ebd35936272482c15bf7f151c994fa83bc70212036d061333eef67bae317ee3";

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice, devWalletBob, devWalletCharlie } = getDevWallets(newKeyring);
    walletAlice = devWalletAlice;
    walletBob = devWalletBob;
    walletCharlie = devWalletCharlie;
  });

  before("Providing funds", async function () {
    this.timeout(2 * 60 * 1000);
    await mintAssetsToWallet(api, walletAlice, walletAlice, [1]);
    await mintAssetsToWallet(api, walletBob, walletAlice, [1]);
  });

  before("Setting up council members", async function () {
    this.timeout(2 * 60 * 1000);
    await sendAndWaitForSuccess(
      api,
      walletAlice,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.councilMembership.addMember(walletBob.address))
    );
  });

  after("Closing the connection", async function () {
    this.timeout(60 * 1000);
    await api.disconnect();
  });

  describe("democracy.notePreimage", function () {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("A user can submit a preimage for a proposal", async function () {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      this.timeout(2 * 60 * 1000);

      await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.democracy.PreimageNoted.is,
        api.tx.democracy.notePreimage(proposalHashOne)
      )
        .catch(e => {
          if (e.message.includes("Preimage already noted")) {
            console.warn("      Skipping test: Preimage already noted!");
            this.skip();
          }
          return e;
        })
        .then(({ data: [resultProposalHash, resultWho, resultDeposit] }) => {
          expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletAlice.publicKey).toString());
          console.debug(resultProposalHash.toString());
          console.debug(resultWho.toString());
          console.debug(resultDeposit.toString());
        });
    });
  });

  describe("democracy.propose", function () {
    it("A user can propose a previously submitted preimage", async function () {
      this.timeout(2 * 60 * 1000);
      const propCountBefore = await api.query.democracy.publicPropCount();
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletBob,
        api.events.democracy.Proposed.is,
        api.tx.democracy.propose(proposalHashOne, 100_000_000_000_000)
      );
      const propCountAfter = await api.query.democracy.publicPropCount();
      // ToDo: Add verification!
      expect(propCountAfter).to.be.bignumber.equal(propCountBefore.addn(1));
      proposalId1 = result.toNumber();
      console.debug(result.toString());
    });
  });

  describe("democracy.second", function () {
    it("Another user can endorse a proposal", async function () {
      this.timeout(2 * 60 * 1000);

      const secondsUpperBound = 1; // ToDo: Check what this parameter does!
      const {
        data: [resultSeconder, resultPropIndex]
      } = await sendAndWaitForSuccess(
        api,
        walletCharlie,
        api.events.democracy.Seconded.is,
        api.tx.democracy.second(proposalId1, secondsUpperBound)
      );
      // ToDo: Add verification!
      expect(resultSeconder.toString()).to.be.equal(api.createType("AccountId32", walletCharlie.publicKey).toString());
      expect(resultPropIndex).to.be.bignumber.equal(new BN(proposalId1));
    });
  });

  describe("democracy.delegate", function () {
    it("A user can delegate their voting power to another account", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletCharlie,
        api.events.democracy.Delegated.is,
        api.tx.democracy.delegate(walletBob.address, api.createType("PalletDemocracyConviction"), 100_000_000_000_000n)
      );
      console.log(result.toString());
    });
  });

  describe("democracy.blacklist", function () {
    it("Sudo can blacklist certain propsals", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.blacklist(proposalToBlackList, api.createType("Option<u32>")))
      );
      expect(result.isOk).to.be.true;
      // Verification
      const {
        data: [resultProposalHash]
      } = await sendAndWaitForSuccess(
        api,
        walletCharlie,
        api.events.democracy.PreimageNoted.is,
        api.tx.democracy.notePreimage(proposalToBlackList)
      );
      expect(resultProposalHash).to.be.an("Error");
    });
  });

  describe("democracy.cancel", function () {
    it("Sudo can cancel certain proposals", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [proposalId]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.democracy.Proposed.is,
        api.tx.democracy.propose(proposalHashOne, 100_000_000_000_000n)
      );
      expect(proposalId).to.not.be.an("Error");
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.cancelProposal(proposalId))
      );
      expect(result.isOk).to.be.true;
    });
  });

  describe("democracy.fastTrack", function () {
    before("Create a proposal", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [proposalId]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.externalProposeMajority(proposalHashOne))
      );
      expect(proposalId).to.not.be.an("Error");
      proposalId2 = proposalId.toNumber();
    });

    it("Sudo can schedule the currently externally-proposed majority-carries referendum to be tabled", async function () {
      this.timeout(2 * 60 * 1000);

      const fundsBefore = await api.rpc.assets.balanceOf("1", walletAlice.address);
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.fastTrack(proposalHashOne, 4, 0))
      );
      expect(result.isOk).to.be.true;
      const fundsAfter = await api.rpc.assets.balanceOf("1", walletAlice.address);
      expect(new BN(fundsAfter.toString())).to.be.bignumber.equal(
        new BN(fundsBefore.toString()).add(new BN("1000000000000"))
      );
    });
  });

  describe("democracy.vote", function () {
    it("Multiple users can vote on a proposal", async function () {
      this.timeout(2 * 60 * 1000);

      Promise.all([
        sendAndWaitForSuccess(
          api,
          walletAlice,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: { aye: true, conviction: null, balance: 999_999_999_999n }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          walletBob,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: { aye: true, conviction: null, balance: 999_999_999_999n }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          walletCharlie,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: { aye: false, conviction: null, balance: 999_999_999n }
            })
          )
        )
      ]);
    });
  });
});
