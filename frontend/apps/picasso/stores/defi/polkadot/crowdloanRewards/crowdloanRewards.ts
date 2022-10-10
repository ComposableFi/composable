import { ApiPromise } from "@polkadot/api";
import { decodeAddress } from "@polkadot/util-crypto";
import {
  CrowdloanAssociation,
  CrowdloanContributionRecord,
} from "./crowdloanRewards.slice";
import {
  presentInRewards,
  presentInContributors,
} from "./CrowdloanRewardsUpdater";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "shared";
/**
 * Fetch on chain associations
 * @param {ApiPromise} api Parachain Api object
 * @param {string[]} connectedAccounts list of accounts in Picasso SS58 format
 * @returns {Promise<CrowdloanAssociation[]>} [address(ss58 format), address(eth or ss58) | null]
 */
export async function fetchAssociations(
  api: ApiPromise,
  connectedAccounts: string[]
): Promise<CrowdloanAssociation[]> {
  let associations: Array<CrowdloanAssociation> = [];
  try {
    const associationPromises = connectedAccounts.map<
      Promise<CrowdloanAssociation>
    >((account) => {
      return new Promise((res, rej) => {
        api.query.crowdloanRewards
          .associations(decodeAddress(account))
          .then((_association) => {
            const associationJSON = _association.toJSON();
            res(
              _association.value.isEthereum
                ? [
                    account,
                    (
                      associationJSON as Record<"ethereum", string>
                    ).ethereum.toLowerCase(),
                  ]
                : _association.value.isRelayChain
                ? [
                    account,
                    (associationJSON as Record<"relayChain", string>)
                      .relayChain,
                  ]
                : [account, null]
            );
          })
          .catch(rej);
      });
    });

    associations = await Promise.all(associationPromises);
  } catch (err: any) {
    console.error("fetchAssociations ", err.message);
  } finally {
    return associations;
  }
}
/**
 * updateContributions from the JSON given ethereum or
 * connected picasso address in kusama format
 * @param {string} connectedAddress address in ss58 or ethereum format
 * @returns {CrowdloanContributionRecord} { [<address>]: { totalRewards: BigNumber, contributedAmount: BigNumber } }
 */
export function fetchContributionAndRewardsFromJSON(
  connectedAddress: string
): CrowdloanContributionRecord {
  let crowdloanContributionRecord: CrowdloanContributionRecord = {};

  let totalRewards = presentInRewards(connectedAddress, process.env.NODE_ENV);
  let totalContributed = presentInContributors(
    connectedAddress,
    process.env.NODE_ENV
  );

  if (!!totalRewards && !!totalContributed) {
    crowdloanContributionRecord[connectedAddress] = {
      totalRewards: new BigNumber(totalRewards),
      contributedAmount: new BigNumber(totalContributed),
    };
  }

  return crowdloanContributionRecord;
}
/**
 * Given picasso account query its
 * claimable rewards
 * @param {ApiPromise} api Parachain api object
 * @param {string} picassoAccount picasso address
 * @returns {Promise<BigNumber>} => claimableReward Amount
 */
export async function fetchClaimableRewards(
  api: ApiPromise,
  picassoAccount: string
): Promise<BigNumber> {
  try {
    const claimableRewards =
      await api.rpc.crowdloanRewards.amountAvailableToClaimFor(picassoAccount);
    return fromChainIdUnit(new BigNumber(claimableRewards.toString()), 12);
  } catch (error) {
    console.error(error);
    return new BigNumber(0);
  }
}
/**
 * Given picasso account or ethereum account
 * query its claimed rewards from the chain
 * @param {ApiPromise} api Parachain api object
 * @param {string} selectedPicassoOrEthAccount account in string
 * @returns {Promise<BigNumber>} => claimed rewards amount
 */
export async function fetchClaimedRewards(
  api: ApiPromise,
  selectedPicassoOrEthAccount: string
): Promise<BigNumber> {
  try {
    const rewards = await api.query.crowdloanRewards.rewards(
      !selectedPicassoOrEthAccount.startsWith("0x")
        ? api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
            RelayChain: api.createType(
              "AccountId32",
              selectedPicassoOrEthAccount
            ),
          })
        : api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
            Ethereum: selectedPicassoOrEthAccount,
          })
    );

    const rewardsJSON = rewards.toJSON();
    let claimed = new BigNumber(0);

    if (rewardsJSON) {
      // @ts-ignore
      claimed = fromChainIdUnit(rewardsJSON.claimed, 12);
    }

    return claimed;
  } catch (error) {
    console.error(error);
    return new BigNumber(0);
  }
}

export function findAssociation(
  account: string | undefined,
  accountType: "ethereum" | "picasso",
  associations: CrowdloanAssociation[]
): CrowdloanAssociation | undefined {
  if (!account) return undefined;

  return associations.find(([_connectedAccount, associatedAccount]) => {
    return associatedAccount !== null
      ? accountType === "ethereum"
        ? associatedAccount.toLowerCase() === account.toLowerCase()
        : accountType === "picasso"
        ? associatedAccount === account
        : false
      : false;
  });
}

export function findAssociatedByAccount(
  account: string | undefined,
  associations: CrowdloanAssociation[]
): CrowdloanAssociation | undefined {
  if (!account) return undefined;

  return associations.find(([_connectedAccount, _associatedAccount]) => {
    return account !== undefined
      ? account === _connectedAccount : false ;
  });
}

export function isAssociatedAccountSameAsConnectedAccount(
  connectedAccount?: string,
  associatedAccount?: CrowdloanAssociation
): boolean {
  if (!connectedAccount && !associatedAccount) return false;
  return connectedAccount === associatedAccount?.[0]
}