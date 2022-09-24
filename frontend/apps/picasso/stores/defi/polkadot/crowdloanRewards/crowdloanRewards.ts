import { fromChainIdUnit } from "@/../../packages/shared";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { ApiPromise } from "@polkadot/api";
import { decodeAddress, encodeAddress } from "@polkadot/util-crypto";
import {
  AccountAssociation,
  CrowdloanAccountAccountState,
  CrowdloanSelectedAccountStatus,
  OnChainAccountAssociation,
} from "./crowdloanRewards.slice";
import {
  presentInRewards,
  presentInRewardsDev,
  presentInContributors,
  presentInContributorsDev,
} from "./CrowdloanRewardsUpdater";
import BigNumber from "bignumber.js";

export async function fetchAssociations(
  api: ApiPromise,
  connectedAccounts: string[]
): Promise<AccountAssociation[]> {
  let associations: Array<AccountAssociation> = [];
  try {
    const associationPromises = connectedAccounts.map<
      Promise<AccountAssociation>
    >((account) => {
      return new Promise((res, rej) => {
        api.query.crowdloanRewards
          .associations(decodeAddress(account))
          .then((_association) => {
            res(
              _association.value.isEthereum
                ? {
                    account,
                    association: _association.value.toString(),
                  }
                : _association.value.isRelayChain
                ? {
                    account,
                    association: _association.value.toString(),
                  }
                : {
                    account,
                    association: null,
                  }
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

export async function fetchClaimableAndClaimedRewards(
  api: ApiPromise,
  accountsState: CrowdloanAccountAccountState[]
): Promise<CrowdloanAccountAccountState[]> {
  try {
    for (const account of accountsState) {
      if (account.address.picassoFormat) {
        const claimableRewards = await api.rpc.crowdloanRewards.amountAvailableToClaimFor(
          account.address.picassoFormat
        );
        account.availableToClaim = fromChainIdUnit(
          new BigNumber(claimableRewards.toString())
        );
      }

      const isRelayChainAccount = account.address.source !== "ethereum";
      const rewards = await api.query.crowdloanRewards.rewards(
        isRelayChainAccount && account.address.picassoFormat
          ? api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
              RelayChain: api.createType(
                "AccountId32",
                account.address.picassoFormat
              ),
            })
          : api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
              Ethereum: account.address,
            })
      );

      console.log(rewards.toJSON());
    }
  } catch (error) {
    console.error(error);
  } finally {
    return accountsState;
  }
}

export function getConnectedAccountState(
  connectedAddress: string,
  connectedAddressSource: "ethereum" | "kusama",
  environment: "development" | "production",
  connectedAccountsAssociations: OnChainAccountAssociation[],
  picassoSS58Format = SUBSTRATE_NETWORKS.picasso.ss58Format
): CrowdloanAccountAccountState {
  console.log('connectedAddress: ', connectedAddress)
  let crowdloan: CrowdloanAccountAccountState = {
    address: {
      ksmOrEthAddress: connectedAddress,
      source: connectedAddressSource,
      picassoFormat:
        connectedAddressSource === "kusama" &&
        !connectedAddress.startsWith("0x")
          ? encodeAddress(connectedAddress, picassoSS58Format)
          : undefined,
    },
    crowdloanSelectedAccountStatus: "ineligible",
    amountContributed: new BigNumber(0),
    totalRewards: new BigNumber(0),
    availableToClaim: new BigNumber(0),
    claimedRewards: new BigNumber(0),
  };

  let crowdloanSelectedAccountStatus: CrowdloanSelectedAccountStatus =
    "ineligible";

  let presentAmountInRewards =
    environment === "production"
      ? presentInRewards(connectedAddress)
      : environment === "development"
      ? presentInRewardsDev(connectedAddress)
      : undefined;
  let presentAmountInContributions =
    environment === "production"
      ? presentInContributors(connectedAddress)
      : environment === "development"
      ? presentInContributorsDev(connectedAddress)
      : undefined;

  if (
    presentAmountInRewards !== undefined &&
    presentAmountInContributions !== undefined
  ) {
    crowdloanSelectedAccountStatus = "canAssociate";

    const association = connectedAccountsAssociations.find((association) => {
      return connectedAddressSource === "ethereum"
        ? association.association === connectedAddress
        : association.account === connectedAddress;
    });

    if (association) {
      crowdloanSelectedAccountStatus = "canClaim";
    }
  }

  crowdloan.crowdloanSelectedAccountStatus = crowdloanSelectedAccountStatus;
  if (presentAmountInContributions)
    crowdloan.amountContributed = new BigNumber(presentAmountInContributions);
  if (presentAmountInRewards)
    crowdloan.totalRewards = new BigNumber(presentAmountInRewards);

  return crowdloan;
}
