import { expect } from "chai";
import { ApiPromise, Keyring } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { sendAndWaitForSuccess, sendWithBatchAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { Pica } from "@composable/utils/mintingHelper";
import BN from "bn.js";
import { before } from "mocha";
import { PalletDemocracyPreimageStatus, PalletDemocracyVoteVoting } from "@composable/types/interfaces";
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
describe("[SHORT] Democracy Tests", function () {
  if (!testConfiguration.enabledTests.query.enabled) return;
  this.timeout(2 * 60 * 1000);
  this.retries(0);

  const apis: ApiPromise[] = [];

  let testWallets: KeyringPair[];
  let sudoKey: KeyringPair;
  let councilMembers: KeyringPair[];
  let technicalCouncilMembers: KeyringPair[];
  let proposalId1: number, // Created during "propose" test. Will get cancelled.
    proposalId2: number; // Created during `fastTrack` test. Will get into a referendum.

  // Image proposes to mint 999_999_999_999 PICA into Alice. Used for all testing proposals.
  const proposalHashOne = "0xcbf6e77875268d88955338afb3538de93bb8cdb0caebbec60e9a0936cb1076b4";

  // Proposal to mint 999_999_999 KSM into Alice. Used for external proposal test.
  const proposalHashTwo = "0x0a4bbd9f97fc9303c331e548b6f3e17d759cb9c52386f07384ff5e284b8b79a8";

  // Proposal to set the chain's timestamp to 0. Will be ban-listed by sudo.
  const proposalToBlackList = "0x5a121beb1148b31fc56f3d26f80800fd9eb4a90435a72d3cc74c42bc72bca9b8";

  before("Setting up the tests", async function () {
    const connections = await Promise.all(Array(7).fill(getNewConnection()));
    let keyring;
    connections.forEach(function (connection: { newClient: ApiPromise; newKeyring: Keyring }, i) {
      apis.push(connection.newClient);
      if (i == 0) keyring = connection.newKeyring;
    });
    if (!keyring) throw new Error("Could not get keyring.");
    const { devWalletAlice, devWalletEve } = getDevWallets(keyring);
    sudoKey = devWalletAlice;
    testWallets = Array(...Array(7)).map(function (_, i) {
      return devWalletEve.derive("/test/democracy/" + i.toString());
    });
    councilMembers = Array(...Array(7)).map(function (_, i) {
      return devWalletEve.derive("/test/democracy/council/" + i.toString());
    });
    technicalCouncilMembers = Array(...Array(7)).map(function (_, i) {
      return devWalletEve.derive("/test/democracy/technicalCouncil/" + i.toString());
    });
  });

  before("Providing funds", async function () {
    const txs = [];
    for (const testWallet of testWallets) {
      txs.push(apis[0].tx.sudo.sudo(apis[0].tx.assets.mintInto(1, testWallet.publicKey, Pica(1_000))));
    }
    for (const councilMember of councilMembers) {
      txs.push(apis[0].tx.sudo.sudo(apis[0].tx.assets.mintInto(1, councilMember.publicKey, Pica(1_000))));
    }
    for (const technicalCouncilMember of technicalCouncilMembers) {
      txs.push(apis[0].tx.sudo.sudo(apis[0].tx.assets.mintInto(1, technicalCouncilMember.publicKey, Pica(1_000))));
    }
    const {
      data: [result]
    } = await sendWithBatchAndWaitForSuccess(apis[0], sudoKey, apis[0].events.sudo.Sudid.is, txs, false);
    expect(result.isOk).to.be.true;
  });

  before("Setting up council members", async function () {
    const councilMemberPublicKeys = [];
    for (const councilMember of councilMembers) {
      councilMemberPublicKeys.push(councilMember.address);
    }
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      apis[0],
      sudoKey,
      apis[0].events.sudo.Sudid.is,
      apis[0].tx.sudo.sudo(apis[0].tx.council.setMembers(councilMemberPublicKeys, councilMemberPublicKeys[0], 0))
    );
    expect(result.isOk).to.be.true;
  });

  before("Setting up technical council members", async function () {
    const technicalCouncilMemberPublicKeys = [];
    for (const technicalCouncilMember of technicalCouncilMembers) {
      technicalCouncilMemberPublicKeys.push(technicalCouncilMember.address);
    }
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      apis[0],
      sudoKey,
      apis[0].events.sudo.Sudid.is,
      apis[0].tx.sudo.sudo(
        apis[0].tx.technicalCommittee.setMembers(
          technicalCouncilMemberPublicKeys,
          technicalCouncilMemberPublicKeys[0],
          0
        )
      )
    );
    expect(result.isOk).to.be.true;
  });

  after("Closing the connection", async function () {
    const txs = [];
    for (const api of apis) {
      txs.push(api.disconnect());
    }
    await Promise.all(txs);
  });

  describe("democracy.notePreimage", function () {
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    it("A user can submit a preimage for a proposal", async function () {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      await sendAndWaitForSuccess(
        apis[0],
        testWallets[0],
        apis[0].events.democracy.PreimageNoted.is,
        apis[0].tx.democracy.notePreimage(proposalHashOne)
      )
        .catch(e => {
          if (e.message.includes("Preimage already noted")) {
            console.warn("      Skipping test: Preimage already noted!");
            this.skip();
          }
          return e;
        })
        .then(async function ({ data: [resultProposalHash, resultWho] }) {
          await waitForBlocks(apis[0]);
          expect(resultWho.toString()).to.be.equal(
            apis[0].createType("AccountId32", testWallets[0].publicKey).toString()
          );
          const preImageResult = <Option<PalletDemocracyPreimageStatus>>(
            await apis[0].query.democracy.preimages(resultProposalHash)
          );
          expect(preImageResult.unwrapOr("NO_PREIMAGE")).to.be.not.be.equal("NO_PREIMAGE");
        });
    });
  });

  describe("democracy.propose", function () {
    it("A user can propose a previously submitted preimage", async function () {
      const propCountBefore = await apis[0].query.democracy.publicPropCount();
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        apis[0],
        testWallets[1],
        apis[0].events.democracy.Proposed.is,
        apis[0].tx.democracy.propose(proposalHashOne, Pica(100))
      );
      const propCountAfter = await apis[0].query.democracy.publicPropCount();
      expect(propCountAfter).to.be.bignumber.equal(propCountBefore.addn(1));
      proposalId1 = result.toNumber();
    });
  });

  describe("democracy.second", function () {
    it("Another user can endorse a proposal", async function () {
      const secondsUpperBound = 1;
      const {
        data: [resultSeconder, resultPropIndex]
      } = await sendAndWaitForSuccess(
        apis[0],
        testWallets[2],
        apis[0].events.democracy.Seconded.is,
        apis[0].tx.democracy.second(proposalId1, secondsUpperBound)
      );
      expect(resultSeconder.toString()).to.be.equal(
        apis[0].createType("AccountId32", testWallets[2].publicKey).toString()
      );
      expect(resultPropIndex).to.be.bignumber.equal(new BN(proposalId1));
    });
  });

  describe("democracy.delegate", function () {
    it("A user can delegate their voting power to another account", async function () {
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        apis[0],
        testWallets[2],
        apis[0].events.democracy.Delegated.is,
        apis[0].tx.democracy.delegate(
          testWallets[1].address,
          apis[0].createType("PalletDemocracyConviction"),
          Pica(100)
        )
      );
      expect(result).to.not.be.an("Error");
    });
  });

  describe("democracy.blacklist", function () {
    it("Sudo can blacklist certain proposals", async function () {
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        apis[0],
        sudoKey,
        apis[0].events.sudo.Sudid.is,
        apis[0].tx.sudo.sudo(apis[0].tx.democracy.blacklist(proposalToBlackList, apis[0].createType("Option<u32>")))
      );
      expect(result.isOk).to.be.true;

      const resultTest = await sendAndWaitForSuccess(
        apis[0],
        testWallets[2],
        apis[0].events.democracy.Proposed.is,
        apis[0].tx.democracy.propose(proposalToBlackList, 999_999_999_999_999_999n)
      ).catch(e => {
        return e;
      });
      expect(resultTest.toString()).to.contain("democracy.ProposalBlacklisted: Proposal still blacklisted");
    });
  });

  describe("democracy.cancel", function () {
    it("Sudo can cancel certain proposals", async function () {
      const {
        data: [proposalId]
      } = await sendAndWaitForSuccess(
        apis[0],
        testWallets[0],
        apis[0].events.democracy.Proposed.is,
        apis[0].tx.democracy.propose(proposalHashOne, Pica(100))
      );
      expect(proposalId).to.not.be.an("Error");
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        apis[0],
        sudoKey,
        apis[0].events.sudo.Sudid.is,
        apis[0].tx.sudo.sudo(apis[0].tx.democracy.cancelProposal(proposalId))
      );
      expect(result.isOk).to.be.true;
      // Trying to second, it should fail.
      await sendAndWaitForSuccess(
        apis[0],
        testWallets[2],
        apis[0].events.democracy.Seconded.is,
        apis[0].tx.democracy.second(proposalId, 1)
      ).catch(function (e) {
        expect(e.toString()).to.contain("democracy.ProposalMissing");
      });
    });
  });

  describe("democracy.fastTrack", function () {
    let councilProposalIndex: number;
    let councilProposalHash: string;

    describe("Council setup", function () {
      it("The Council members can propose a external majority proposal", async function () {
        const threshold = councilMembers.length - 2;
        const lengthBound = 36;
        const {
          data: [resultAccount, resultProposalIndex, resultProposalHash, resultThreshold]
        } = await sendAndWaitForSuccess(
          apis[0],
          councilMembers[0],
          apis[0].events.council.Proposed.is,
          apis[0].tx.council.propose(
            threshold,
            apis[0].tx.democracy.externalProposeMajority(proposalHashTwo),
            lengthBound
          )
        );
        councilProposalIndex = resultProposalIndex.toNumber();
        councilProposalHash = resultProposalHash.toString();
        expect(resultAccount.toString()).to.be.equal(
          apis[0].createType("AccountId32", councilMembers[0].publicKey).toString()
        );
        expect(resultThreshold.toNumber()).to.be.equal(threshold);
      });

      it("Council members can vote on Council proposal", async function () {
        const txs = [];
        for (let i = 0; i < councilMembers.length; i++) {
          txs.push(
            sendAndWaitForSuccess(
              apis[i],
              councilMembers[i],
              apis[i].events.council.Voted.is,
              // The last council member says declines the proposal
              apis[i].tx.council.vote(councilProposalHash, councilProposalIndex, i != councilMembers.length - 1)
            )
          );
        }
        const results = await Promise.all(txs);
        for (let i = 0; i < results.length; i++) {
          const resultData = results[i].data;
          expect(resultData[0].toString()).to.be.equal(
            apis[0].createType("AccountId32", councilMembers[i].publicKey).toString()
          );
          expect(resultData[1].toString()).to.be.equal(councilProposalHash);
          expect(Boolean(resultData[2])).to.be.equal(i != councilMembers.length);
        }
      });

      it("Council members can close the Council proposal after reaching threshold", async function () {
        const weightBound = 103_534_000;
        const lengthBound = 34;
        const {
          data: [resultProposalHash, resultAmountYes, resultAmountNo]
        } = await sendAndWaitForSuccess(
          apis[0],
          councilMembers[0],
          apis[0].events.council.Closed.is,
          apis[0].tx.council.close(councilProposalHash, councilProposalIndex, weightBound, lengthBound)
        );
        expect(resultProposalHash.toString()).to.be.equal(councilProposalHash);
        expect(resultAmountYes.toNumber()).to.be.equal(councilMembers.length - 1);
        expect(resultAmountNo.toNumber()).to.be.equal(1);
      });
    });

    describe("Fast tracking using the Technical Council", function () {
      let technicalProposalIndex: number;
      let technicalProposalHash: string;
      it(
        "The technical council can propose to fast track the currently " +
          "externally-proposed majority-carries referendum to be tabled",
        async function () {
          const threshold = technicalCouncilMembers.length - 2;
          const lengthBound = 43;
          const {
            data: [resultAccount, resultProposalIndex, resultProposalHash, resultThreshold]
          } = await sendAndWaitForSuccess(
            apis[0],
            technicalCouncilMembers[0],
            apis[0].events.technicalCommittee.Proposed.is,
            apis[0].tx.technicalCommittee.propose(
              threshold,
              apis[0].tx.democracy.fastTrack(proposalHashTwo, 20, 4),
              lengthBound
            )
          );
          technicalProposalIndex = resultProposalIndex.toNumber();
          technicalProposalHash = resultProposalHash.toString();
          expect(resultAccount.toString()).to.be.equal(
            apis[0].createType("AccountId32", technicalCouncilMembers[0].publicKey).toString()
          );
          expect(resultThreshold.toNumber()).to.be.equal(threshold);
        }
      );

      it("Technical Council members can vote on Council proposal", async function () {
        const txs = [];
        for (let i = 0; i < technicalCouncilMembers.length; i++) {
          txs.push(
            sendAndWaitForSuccess(
              apis[i],
              technicalCouncilMembers[i],
              apis[i].events.technicalCommittee.Voted.is,
              // The last council member says declines the proposal
              apis[i].tx.technicalCommittee.vote(
                technicalProposalHash,
                technicalProposalIndex,
                i != technicalCouncilMembers.length - 1
              )
            )
          );
        }
        const results = await Promise.all(txs);
        for (let i = 0; i < results.length; i++) {
          const resultData = results[i].data;
          expect(resultData[0].toString()).to.be.equal(
            apis[0].createType("AccountId32", technicalCouncilMembers[i].publicKey).toString()
          );
          expect(resultData[1].toString()).to.be.equal(technicalProposalHash);
        }
      });

      it("Technical Council members can close the Council proposal after reaching threshold", async function () {
        const weightBound = 386_442_000;
        const lengthBound = 42;
        const {
          data: [resultProposalHash, resultAmountYes, resultAmountNo]
        } = await sendAndWaitForSuccess(
          apis[0],
          technicalCouncilMembers[0],
          apis[0].events.technicalCommittee.Closed.is,
          apis[0].tx.technicalCommittee.close(technicalProposalHash, technicalProposalIndex, weightBound, lengthBound)
        );
        expect(resultProposalHash.toString()).to.be.equal(technicalProposalHash);
        expect(resultAmountYes.toNumber()).to.be.equal(technicalCouncilMembers.length - 1);
        expect(resultAmountNo.toNumber()).to.be.equal(1);
      });
    });
  });

  describe("democracy.vote", function () {
    it("Multiple users can vote on a proposal", async function () {
      await Promise.all([
        sendAndWaitForSuccess(
          apis[0],
          testWallets[0],
          apis[0].events.democracy.Voted.is,
          apis[0].tx.democracy.vote(
            proposalId2,
            apis[0].createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: Pica(50)
              }
            })
          )
        ),
        sendAndWaitForSuccess(
          apis[1],
          testWallets[1],
          apis[1].events.democracy.Voted.is,
          apis[1].tx.democracy.vote(
            proposalId2,
            apis[1].createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: Pica(10)
              }
            })
          )
        ),
        sendAndWaitForSuccess(
          apis[2],
          testWallets[3],
          apis[2].events.democracy.Voted.is,
          apis[2].tx.democracy.vote(
            proposalId2,
            apis[2].createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: Pica(100)
              }
            })
          )
        ),
        sendAndWaitForSuccess(
          apis[3],
          testWallets[4],
          apis[3].events.democracy.Voted.is,
          apis[3].tx.democracy.vote(
            proposalId2,
            apis[3].createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: true
                },
                balance: Pica(42)
              }
            })
          )
        ),
        sendAndWaitForSuccess(
          apis[4],
          testWallets[5],
          apis[4].events.democracy.Voted.is,
          apis[4].tx.democracy.vote(
            proposalId2,
            apis[4].createType("PalletDemocracyVoteAccountVote", {
              Standard: {
                vote: {
                  conviction: null,
                  aye: "Nay"
                },
                balance: Pica(1)
              }
            })
          )
        )
      ]);
      await waitForBlocks(apis[0]);
      const queriesToCheck: Promise<PalletDemocracyVoteVoting>[] = [];
      testWallets.forEach(function (query, i) {
        queriesToCheck.push(apis[i].query.democracy.votingOf(testWallets[i].publicKey));
      });
      const [voteWallet1, voteWallet2, voteWallet3, voteWallet4, voteWallet5, voteWallet6, voteWallet7] =
        await Promise.all(queriesToCheck);

      expect(voteWallet1.asDirect.votes.length).to.equal(1);
      expect(voteWallet2.asDirect.votes.length).to.equal(1);
      expect(voteWallet3.asDelegating).to.not.be.an("Undefined");
      expect(voteWallet4.asDirect.votes.length).to.equal(1);
      expect(voteWallet5.asDirect.votes.length).to.equal(1);
      expect(voteWallet6.asDirect.votes.length).to.equal(1);
    });
  });

  describe("democracy.removeVote", function () {
    it("Users can remove their vote", async function () {
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        apis[0],
        testWallets[5],
        apis[0].events.system.ExtrinsicSuccess.is,
        apis[0].tx.democracy.removeVote(proposalId2)
      );
      expect(result).to.be.not.an("Error");
      const voteWalletFerdie = await apis[0].query.democracy.votingOf(testWallets[5].publicKey);
      expect(voteWalletFerdie.asDirect.votes.length).to.equal(0);
    });
  });

  describe("Referendum should succeed", function () {
    it("Waiting for referendum succession", async function () {
      this.timeout(5 * 60 * 1000);
      let success = false;
      do {
        const currentEvents = await apis[0].query.system.events();
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
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        apis[0],
        testWallets[2],
        apis[0].events.democracy.Undelegated.is,
        apis[0].tx.democracy.undelegate()
      );
      expect(result).to.be.not.an("Error");
    });
  });

  describe("democracy.clearPublicProposals", function () {
    it("Sudo can clear public proposals", async function () {
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        apis[0],
        sudoKey,
        apis[0].events.sudo.Sudid.is,
        apis[0].tx.sudo.sudo(apis[0].tx.democracy.clearPublicProposals())
      );
      expect(result.isOk).to.be.true;
    });
  });
});
