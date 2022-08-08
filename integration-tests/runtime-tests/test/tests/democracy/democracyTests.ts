import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import BN from "bn.js";
import exp from "constants";
import { before } from "mocha";
import { PalletDemocracyPreimageStatus } from "@composable/types/interfaces";
import { Option } from "@polkadot/types-codec";

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
 * - emergencyCancel(refIndex) // Can not be tested here!
 * - enactProposal(proposalHash, index) // Not tested! System internal extrinsic, to be called by Scheduler.
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
 */
describe.only("[SHORT] Democracy Tests", function () {
  if (!testConfiguration.enabledTests.query.enabled) return;

  let api: ApiPromise;
  let walletAlice: KeyringPair,
    walletBob: KeyringPair,
    walletCharlie: KeyringPair,
    walletDave: KeyringPair,
    walletEve: KeyringPair,
    walletFerdie: KeyringPair; // ToDo: Change wallets!
  let proposalId1: number, // Created during propose test. Will stay a proposal.
    proposalId2: number; // Created during `fastTrack` test. Will get into referendum.

  // Image proposes to mint 999_999_999_999 PICA into Alice. Used for all testing proposals.
  // ToDo: Change to hashing a function directly for better readability.
  const proposalHashOne = "0xcbf6e77875268d88955338afb3538de93bb8cdb0caebbec60e9a0936cb1076b4";

  // Proposal to mint 999_999_999 KSM into Alice. Used for external proposal test.
  const proposalHashTwo = "0x0a4bbd9f97fc9303c331e548b6f3e17d759cb9c52386f07384ff5e284b8b79a8";

  // Proposal to mint 999_999_999 KSM into Dave. Used for external proposal default test.
  const proposalHashThree = "0x4696077c56d369271d482e769036e543187551b5044dee8cca8cee344680ac60";

  // Proposal to set the chains timestamp to 0. Will be blacklisted by sudo.
  const proposalToBlackList = "0x5a121beb1148b31fc56f3d26f80800fd9eb4a90435a72d3cc74c42bc72bca9b8";

  // Proposal to move all sender funds to Eve. Will be externally proposed, vetoed and blacklisted by sudo.
  const externalProposalToVetoAndBlacklist = "0xdc3ea549546a1fff28397c102f9471bbf6140cb82ebd8e027e55c3275663ece9";

  // Proposal to mint 999_999_999_999 PICA into Bob. The preimage will be submitted and then reaped from chain.
  const proposalToReapPreImage = "0x8ebd35936272482c15bf7f151c994fa83bc70212036d061333eef67bae317ee3";

  // Proposal to burn 999_999_999_999 PICA from Dave. To be emergency cancelled.
  const proposalToEmergencyCancel = "0x8184fbd37fd31f72c8938e3779773ab6261de4ab51478c717ea758efec0dcf2a";

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice, devWalletBob, devWalletCharlie, devWalletDave, devWalletEve, devWalletFerdie } =
      getDevWallets(newKeyring);
    walletAlice = devWalletAlice;
    walletBob = devWalletBob;
    walletCharlie = devWalletCharlie;
    walletDave = devWalletDave;
    walletEve = devWalletEve;
    walletFerdie = devWalletFerdie;
  });

  before("Providing funds", async function () {
    this.timeout(5 * 60 * 1000);
    await mintAssetsToWallet(api, walletAlice, walletAlice, [1]);
    await mintAssetsToWallet(api, walletBob, walletAlice, [1]);
    await mintAssetsToWallet(api, walletCharlie, walletAlice, [1]);
    await mintAssetsToWallet(api, walletDave, walletAlice, [1]);
    await mintAssetsToWallet(api, walletEve, walletAlice, [1]);
    await mintAssetsToWallet(api, walletFerdie, walletAlice, [1]);
  });

  before("Setting up council members", async function () {
    this.timeout(2 * 60 * 1000);
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      api,
      walletAlice,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.councilMembership.addMember(walletBob.address))
    );
    expect(result.isOk).to.be.true;
  });

  before("Setting up council members", async function () {
    this.timeout(2 * 60 * 1000);
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      api,
      walletAlice,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.technicalMembership.addMember(walletBob.address))
    );
    expect(result.isOk).to.be.true;
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
        .then(async function ({ data: [resultProposalHash, resultWho, resultDeposit] }) {
          await waitForBlocks(api);
          expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", walletAlice.publicKey).toString());
          const preImageResult = <Option<PalletDemocracyPreimageStatus>>(
            await api.query.democracy.preimages(resultProposalHash)
          );
          expect(preImageResult.unwrapOr("NO_PREIMAGE")).to.be.not.be.equal("NO_PREIMAGE");
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

      const result = await sendAndWaitForSuccess(
        api,
        walletCharlie,
        api.events.democracy.Delegated.is,
        api.tx.democracy.delegate(walletBob.address, api.createType("PalletDemocracyConviction"), 100_000_000_000_000n)
      );
      console.debug(result.toString());
    });
  });

  describe("democracy.undelegate", function () {
    it("A user can undelegate their voting power", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletCharlie,
        api.events.democracy.Undelegated.is,
        api.tx.democracy.undelegate()
      );
      console.log(result.toString());
    });
  });

  describe("democracy.blacklist", function () {
    it("Sudo can blacklist certain proposals", async function () {
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

      await sendAndWaitForSuccess(
        api,
        walletCharlie,
        api.events.democracy.Proposed.is,
        api.tx.democracy.propose(proposalToBlackList, 999_999_999_999_999_999n)
      ).catch(e => {
        expect(e.toString()).to.contain("democracy.ProposalBlacklisted: Proposal still blacklisted");
      });
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
      // Trying to second it should fail.
      const secondResult = await sendAndWaitForSuccess(
        api,
        walletCharlie,
        api.events.democracy.Seconded.is,
        api.tx.democracy.second(proposalId, 1)
      ).catch(function (e) {
        console.debug(e.toString());
      });
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
        api.tx.sudo.sudo(api.tx.democracy.externalProposeMajority(proposalHashTwo))
      );
      expect(proposalId).to.not.be.an("Error");
      proposalId2 = proposalId.toNumber();
    });

    it("Sudo can schedule the currently externally-proposed majority-carries referendum to be tabled", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.fastTrack(proposalHashTwo, 8, 0))
      );
      expect(result.isOk).to.be.true;
    });
  });

  describe("democracy.vote", function () {
    it("Multiple users can vote on a proposal", async function () {
      this.timeout(2 * 60 * 1000);

      await Promise.all([
        sendAndWaitForSuccess(
          api,
          walletAlice,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: { aye: true, conviction: null, balance: 99_999_999_999_999_999_999n }
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
              Standard: { aye: true, conviction: null, balance: 99_999_999_999_999_999_999n }
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
              Standard: { aye: true, conviction: null, balance: 99_999_999_999_999_999_999n }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          walletDave,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: { aye: true, conviction: null, balance: 99_999_999_999_999_999_999n }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          walletEve,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: { aye: true, conviction: null, balance: 99_999_999_999_999_999_999n }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          walletFerdie,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: { aye: false, conviction: null, balance: 999_999_999_999n }
            })
          )
        )
      ]).then(function ([result1, result2, result3, result4, result5, result6]) {
        console.debug(result1.toString());
        console.debug(result2.toString());
        console.debug(result3.toString());
        console.debug(result4.toString());
        console.debug(result5.toString());
        console.debug(result6.toString());
        return [result1, result2, result3];
      });
      await waitForBlocks(api);
      const voteWalletAlice = await api.query.democracy.votingOf(walletAlice.publicKey);
      const voteWalletBob = await api.query.democracy.votingOf(walletBob.publicKey);
      const voteWalletCharlie = await api.query.democracy.votingOf(walletCharlie.publicKey);
      const voteWalletDave = await api.query.democracy.votingOf(walletDave.publicKey);
      const voteWalletEve = await api.query.democracy.votingOf(walletEve.publicKey);
      const voteWalletFerdie = await api.query.democracy.votingOf(walletFerdie.publicKey);

      console.debug(voteWalletAlice.toString());
      console.debug(voteWalletBob.toString());
      console.debug(voteWalletCharlie.toString());
      console.debug(voteWalletDave.toString());
      console.debug(voteWalletEve.toString());
      console.debug(voteWalletFerdie.toString());
    });
  });

  describe("democracy.removeVote", function () {
    it("Users can remove their vote", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletFerdie,
        api.events.system.ExtrinsicSuccess.is,
        api.tx.democracy.removeVote(proposalId2)
      );
      expect(result).to.be.not.an("Error");
      // ToDo: Get check work!
      //const voteWalletFerdie = await api.query.democracy.votingOf(walletFerdie.publicKey);
      //console.debug(voteWalletFerdie.toString());
      //expect(voteWalletFerdie.direct.votes.length).to.equal(0);
    });
  });

  describe("Referendum should succeed", function () {
    it("Waiting for referendum succession", async function () {
      this.timeout(5 * 60 * 1000);
      let success = false;
      do {
        const currentEvents = await api.query.system.events();
        currentEvents.forEach(event => {
          if (event.event.section.toString() === "democracy") {
            if (event.event.method.toString() === "Passed") {
              success = true;
              return;
            }
            if (event.event.method.toString() === "NotPassed") throw new Error("Referendum failed");
          }
        });
      } while (!success);
      expect(success).to.be.true;
    });
  });

  describe("democracy.reapPreimage", function () {
    before("Submit preimage", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.democracy.PreimageNoted.is,
        api.tx.democracy.notePreimage(proposalToReapPreImage)
      );
      expect(result).to.not.be.an("Error");
      await waitForBlocks(api);
    });
    it("Sudo can reap a preimage", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.democracy.PreimageReaped.is,
        api.tx.democracy.reapPreimage(proposalToReapPreImage, 0)
      );
      expect(result).to.not.be.an("Error");
      // ToDo: Improve verification!
    });
  });
  describe("democracy.clearPublicProposals", function () {
    it("Sudo can clear public proposals", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.clearPublicProposals())
      );
      expect(result.isOk).to.be.true;
    });
  });
});
