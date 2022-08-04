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

  // Proposal to mint 999_999_999 KSM into Alice.
  const proposalHashTwo = "0x0a4bbd9f97fc9303c331e548b6f3e17d759cb9c52386f07384ff5e284b8b79a8";

  // Proposal to set the chains timestamp to 0. Will be blacklisted by sudo.
  const proposalToBlackList = "0x5a121beb1148b31fc56f3d26f80800fd9eb4a90435a72d3cc74c42bc72bca9b8";

  // Proposal to move all sender funds to Eve. Will be blacklisted by sudo.
  const externalProposalToVetoAndBlacklist = "0xdc3ea549546a1fff28397c102f9471bbf6140cb82ebd8e027e55c3275663ece9";

  // Proposal to mint 999_999_999_999 PICA into Bob. The preimage will be submitted and then reaped from chain.
  const proposalToReapPreImage = "0x8ebd35936272482c15bf7f151c994fa83bc70212036d061333eef67bae317ee3";

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

  describe("democracy.noteImminentPreimage", function () {});
  describe("democracy.noteImminentPreimageOperational", function () {});
  describe("democracy.notePreimageOperational", function () {});

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

  describe("democracy.externalPropose", function () {
    it("Sudo can propose an external proposal", async function () {
      this.timeout(2 * 60 * 1000);
      const propCountBefore = await api.query.democracy.publicPropCount();
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.externalPropose(proposalHashOne))
      );
      const propCountAfter = await api.query.democracy.publicPropCount();
      expect(result.isOk).to.be.true;
      expect(propCountAfter).to.be.bignumber.equal(propCountBefore.addn(1));
      console.debug(result.toString());
    });
  });
  describe("democracy.externalProposeDefault", function () {
    it("Sudo can propose an external proposal", async function () {
      this.timeout(2 * 60 * 1000);
      const propCountBefore = await api.query.democracy.publicPropCount();
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.externalProposeDefault(proposalHashOne))
      );
      const propCountAfter = await api.query.democracy.publicPropCount();
      expect(result.isOk).to.be.true;
      expect(propCountAfter).to.be.bignumber.equal(propCountBefore.addn(1));
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

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.fastTrack(proposalHashOne, 12, 0))
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
              Standard: { aye: true, conviction: null, balance: 999_999_999_999_999_999n }
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
              Standard: { aye: true, conviction: null, balance: 999_999_999_999_999_999n }
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
              Standard: { aye: true, conviction: null, balance: 999_999_999_999_999_999n }
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
              Standard: { aye: true, conviction: null, balance: 999_999_999_999_999_999n }
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
              Standard: { aye: true, conviction: null, balance: 999_999_999_999_999_999n }
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
              Standard: { aye: false, conviction: null, balance: 999_999_999_999_999n }
            })
          )
        )
      ]).then(function ([result1, result2, result3, result4, result5]) {
        console.debug(result1.toString());
        console.debug(result2.toString());
        console.debug(result3.toString());
        return [result1, result2, result3];
      });
      const x = await api.query.democracy.votingOf(proposalId2);
      console.debug(x.toString());
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
    });
  });
  describe("democracy.removeOtherVote", function () {
    it("Sudo can remove other users' votes", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.removeOtherVote(walletEve.address, proposalId2))
      );
      expect(result.isOk).to.be.true;
    });
  });

  describe("democracy.cancelQueued", function () {
    it("Sudo can cancel a proposal queued for enactment", async function () {});
  });

  describe("democracy.cancelReferendum", function () {
    let cancelReferendumId: number;
    before("Create referendum", async function () {
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
      cancelReferendumId = proposalId.toNumber();
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.fastTrack(proposalHashOne, 4, 0))
      );
      expect(result.isOk).to.be.true;
    });

    it("Sudo can cancel a referendum", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.cancelReferendum(cancelReferendumId))
      );
      expect(result.isOk).to.be.true;
    });
  });
  describe("democracy.emergencyCancel", function () {
    let emergencyCancelId: number;
    before("Create referendum", async function () {
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
      emergencyCancelId = proposalId.toNumber();
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.fastTrack(proposalHashOne, 4, 0))
      );
      expect(result.isOk).to.be.true;
      await waitForBlocks(api);
    });

    it("Sudo can cancel a referendum", async function () {
      this.timeout(2 * 60 * 1000);

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.emergencyCancel(emergencyCancelId))
      );
      expect(result.isOk).to.be.true;
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
    });
    it("Sudo can reap a preimage", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.reapPreimage(proposalToReapPreImage, 0))
      );
      expect(result.isOk).to.be.true;
    });
  });
  describe("democracy.unlock", function () {
    it("Sudo can unlock a proposal", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletFerdie,
        api.events.system.ExtrinsicSuccess.is,
        api.tx.democracy.unlock(walletFerdie.publicKey)
      );
      console.debug(result.toString());
    });
  });
  describe("democracy.vetoExternal", function () {
    before("Create external proposal", async function () {});

    it("Sudo can veto an external proposal", async function () {
      this.timeout(2 * 60 * 1000);
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.democracy.vetoExternal(externalProposalToVetoAndBlacklist))
      );
      expect(result.isOk).to.be.true;
      const {
        data: [proposalResult]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.democracy.Proposed.is,
        api.tx.democracy.externalPropose(externalProposalToVetoAndBlacklist)
      );
      expect(proposalResult).to.be.an("Error");
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
