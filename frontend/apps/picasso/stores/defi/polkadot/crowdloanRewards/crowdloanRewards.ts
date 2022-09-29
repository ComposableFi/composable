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

export function updateContributions(
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

export async function fetchClaimableRewards(
  api: ApiPromise,
  picassoAccount: string
): Promise<BigNumber> {
  try {
    const claimableRewards =
      await api.rpc.crowdloanRewards.amountAvailableToClaimFor(picassoAccount);
    return fromChainIdUnit(
        new BigNumber(claimableRewards.toString()),
        12
      )
  } catch (error) {
    console.error(error);
    return new BigNumber(0)
  }
}

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
