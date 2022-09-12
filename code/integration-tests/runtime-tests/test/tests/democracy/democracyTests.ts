import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import BN from "bn.js";
import { before } from "mocha";
import { PalletDemocracyPreimageStatus } from "@composable/types/interfaces";
import { Option } from "@polkadot/types-codec";

/**
 * Democracy Test Suite
 *
 * Democracy Extrinsics & Queries
 *
 * Queries:
 *
 * - blacklist(H256) : Option<(u32, Vec<AccountId32>)>
 * - cancellations(H256) : bool
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
 * - undelegate
 * - unlock(target)
 * - vetoExternal(proposalHash)
 * - vote(refIndex, vote)
 */
describe("Democracy Tests", function () {
  if (!testConfiguration.enabledTests.query.enabled) return;

  let api: ApiPromise;
  let testWallet1: KeyringPair,
    testWallet2: KeyringPair,
    testWallet3: KeyringPair,
    testWallet4: KeyringPair,
    testWallet5: KeyringPair,
    testWallet6: KeyringPair,
    sudoKey: KeyringPair; // ToDo: Change wallets!
  let proposalId1: number, // Created during propose test. Will get cancelled.
    proposalId2: number; // Created during `fastTrack` test. Will get into referendum.

  // Image proposes to mint 999_999_999_999 PICA into Alice. Used for all testing proposals.
  // ToDo: Change to hashing a function directly for better readability.
  const proposalHashOne = "0xcbf6e77875268d88955338afb3538de93bb8cdb0caebbec60e9a0936cb1076b4";

  // Proposal to mint 999_999_999 KSM into Alice. Used for external proposal test.
  const proposalHashTwo = "0x0a4bbd9f97fc9303c331e548b6f3e17d759cb9c52386f07384ff5e284b8b79a8";

  // Proposal to set the chains timestamp to 0. Will be blacklisted by sudo.
  const proposalToBlackList = "0x5a121beb1148b31fc56f3d26f80800fd9eb4a90435a72d3cc74c42bc72bca9b8";

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice, devWalletBob, devWalletCharlie, devWalletDave, devWalletEve, devWalletFerdie } =
      getDevWallets(newKeyring);
    testWallet1 = devWalletAlice.derive("/test/democracy/1");
    testWallet2 = devWalletBob.derive("/test/democracy/2");
    testWallet3 = devWalletCharlie.derive("/test/democracy/3");
    testWallet4 = devWalletDave.derive("/test/democracy/4");
    testWallet5 = devWalletEve.derive("/test/democracy/5");
    testWallet6 = devWalletFerdie.derive("/test/democracy/6");
    sudoKey = devWalletAlice;
  });

  before("Providing funds", async function () {
    this.timeout(5 * 60 * 1000);
    await mintAssetsToWallet(api, testWallet1, sudoKey, [1]);
    await mintAssetsToWallet(api, testWallet2, sudoKey, [1]);
    await mintAssetsToWallet(api, testWallet3, sudoKey, [1]);
    await mintAssetsToWallet(api, testWallet4, sudoKey, [1]);
    await mintAssetsToWallet(api, testWallet5, sudoKey, [1]);
    await mintAssetsToWallet(api, testWallet6, sudoKey, [1]);
  });

  before("Setting up council members", async function () {
    this.timeout(2 * 60 * 1000);
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.councilMembership.addMember(testWallet2.address))
    );
    expect(result.isOk).to.be.true;
  });

  before("Setting up council members", async function () {
    this.timeout(2 * 60 * 1000);
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.technicalMembership.addMember(testWallet2.address))
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
        testWallet1,
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
          expect(resultWho.toString()).to.be.equal(api.createType("AccountId32", testWallet1.publicKey).toString());
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
        testWallet2,
        api.events.democracy.Proposed.is,
        api.tx.democracy.propose(proposalHashOne, 100_000_000_000_000)
      );
      const propCountAfter = await api.query.democracy.publicPropCount();
      expect(propCountAfter).to.be.bignumber.equal(propCountBefore.addn(1));
      proposalId1 = result.toNumber();
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
        testWallet3,
        api.events.democracy.Seconded.is,
        api.tx.democracy.second(proposalId1, secondsUpperBound)
      );
      expect(resultSeconder.toString()).to.be.equal(api.createType("AccountId32", testWallet3.publicKey).toString());
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
        testWallet3,
        api.events.democracy.Delegated.is,
        api.tx.democracy.delegate(
          testWallet2.address,
          api.createType("PalletDemocracyConviction"),
          100_000_000_000_000n
        )
      );
      expect(result).to.not.be.an("Error");
    });
  });

  describe("democracy.blacklist", function () {
    it("Sudo can blacklist certain proposals", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.blacklist(proposalToBlackList, api.createType("Option<u32>")))
      );
      expect(result.isOk).to.be.true;

      await sendAndWaitForSuccess(
        api,
        testWallet3,
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
        testWallet1,
        api.events.democracy.Proposed.is,
        api.tx.democracy.propose(proposalHashOne, 100_000_000_000_000n)
      );
      expect(proposalId).to.not.be.an("Error");
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.cancelProposal(proposalId))
      );
      expect(result.isOk).to.be.true;
      // Trying to second it should fail.
      await sendAndWaitForSuccess(
        api,
        testWallet3,
        api.events.democracy.Seconded.is,
        api.tx.democracy.second(proposalId, 1)
      ).catch(function (e) {
        expect(e.toString()).to.contain("democracy.ProposalMissing");
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
        sudoKey,
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
        sudoKey,
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
          testWallet1,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: 99_999_999_999_900_000_000n
              }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          testWallet2,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: 99_999_999_999_900_000_000n
              }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          testWallet4,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: 99_999_999_999_900_000_000n
              }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          testWallet5,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: 99_999_999_999_900_000_000n
              }
            })
          )
        ),
        sendAndWaitForSuccess(
          api,
          testWallet6,
          api.events.democracy.Voted.is,
          api.tx.democracy.vote(
            proposalId2,
            api.createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: "Nay"
                },
                balance: 99_999_999_999_900_000_000n
              }
            })
          )
        )
      ]);
      await waitForBlocks(api);
      const voteWalletAlice = await api.query.democracy.votingOf(testWallet1.publicKey);
      const voteWalletBob = await api.query.democracy.votingOf(testWallet2.publicKey);
      const voteWalletCharlie = await api.query.democracy.votingOf(testWallet3.publicKey);
      const voteWalletDave = await api.query.democracy.votingOf(testWallet4.publicKey);
      const voteWalletEve = await api.query.democracy.votingOf(testWallet5.publicKey);
      const voteWalletFerdie = await api.query.democracy.votingOf(testWallet6.publicKey);

      expect(voteWalletAlice.asDirect.votes.length).to.equal(1);
      expect(voteWalletBob.asDirect.votes.length).to.equal(1);
      expect(voteWalletCharlie.asDelegating).to.not.be.an("Undefined");
      expect(voteWalletDave.asDirect.votes.length).to.equal(1);
      expect(voteWalletEve.asDirect.votes.length).to.equal(1);
      expect(voteWalletFerdie.asDirect.votes.length).to.equal(1);
    });
  });

  describe("democracy.removeVote", function () {
    it("Users can remove their vote", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        testWallet6,
        api.events.system.ExtrinsicSuccess.is,
        api.tx.democracy.removeVote(proposalId2)
      );
      expect(result).to.be.not.an("Error");
      const voteWalletFerdie = await api.query.democracy.votingOf(testWallet6.publicKey);
      expect(voteWalletFerdie.asDirect.votes.length).to.equal(0);
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

  describe("democracy.undelegate", function () {
    it("A user can undelegate their voting power", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        testWallet3,
        api.events.democracy.Undelegated.is,
        api.tx.democracy.undelegate()
      );
      expect(result).to.be.not.an("Error");
    });
  });

  describe("democracy.clearPublicProposals", function () {
    it("Sudo can clear public proposals", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.clearPublicProposals())
      );
      expect(result.isOk).to.be.true;
    });
  });
});
