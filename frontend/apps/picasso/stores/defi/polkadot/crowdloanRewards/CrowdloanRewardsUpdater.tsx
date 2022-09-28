import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { useEffect } from "react";
import { useBlockchainProvider } from "bi-lib";
import { fromPerbill } from "shared";
import {
  fetchAssociations,
  updateContributions,
} from "./crowdloanRewards";
import {
  CrowdloanContributionRecord,
  setCrowdloanRewardsState,
} from "./crowdloanRewards.slice";
// Import static JSON files
import rewards from "@/defi/polkadot/constants/pica-rewards.json";
import contributions from "@/defi/polkadot/constants/contributions.json";
import contributionsDev from "@/defi/polkadot/constants/contributions-dev.json";
import rewardsDev from "@/defi/polkadot/constants/pica-rewards-dev.json";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { encodeAddress } from "@polkadot/util-crypto";

/**
 * Check for contributions in JSON
 * @param account ethereum or kusama format address
 * @returns string | undefined
 */
export const presentInContributors = (
  account: string,
  env: "development" | "production" | "test"
): string | undefined => {
  switch (env) {
    case "production":
      return (contributions.contributedAmounts as Record<string, string>)[
        account
      ];
    case "development":
      return (contributionsDev.contributedAmounts as Record<string, string>)[
        account
      ];
    default:
      return undefined;
  }
};

/**
 * Check for rewards in JSON
 * @param account ethereum or kusama format address
 * @returns string | undefined
 */
export const presentInRewards = (
  account: string,
  env: "development" | "production" | "test"
): string | undefined => {
  switch (env) {
    case "production":
      return (rewards as Record<string, string>)[account];
    case "development":
      return (rewardsDev as Record<string, string>)[account];
    default:
      return undefined;
  }
};

const DEFAULT_EVM_ID = 1;

const CrowdloanRewardsUpdater = () => {
  const { account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const { parachainApi, accounts } = usePicassoProvider();

  useEffect(() => {
    if (parachainApi) {
      const initialPayment = fromPerbill(
        parachainApi.consts.crowdloanRewards.initialPayment.toString()
      ).div(100);
      setCrowdloanRewardsState({ initialPayment });
    }
  }, [parachainApi]);

  useEffect(() => {
    if (parachainApi && accounts.length > 0) {
      const addresses = accounts.map((_account) => _account.address);
      fetchAssociations(
        parachainApi,
        addresses.filter((address) => !address.startsWith("0x"))
      )
        .then((onChainAssociations) => {
          setCrowdloanRewardsState({ onChainAssociations });
        })
        .catch(console.error);
    }
  }, [parachainApi, accounts]);

  useEffect(() => {
    if (accounts.length <= 0) return;

    let contributions = accounts
      .map((ksmAccount) => {
        const ksmAddress = encodeAddress(
          ksmAccount.address,
          SUBSTRATE_NETWORKS.kusama.ss58Format
        );
        return updateContributions(ksmAddress);
      })
      .reduce((agg, curr) => {
        return {
          ...agg,
          ...curr,
        };
      }, {} as CrowdloanContributionRecord);

    setCrowdloanRewardsState({
      kusamaContributions: contributions,
    });
  }, [accounts]);

  useEffect(() => {
    if (!account) return;

    setCrowdloanRewardsState({
      ethereumContributions: updateContributions(account.toLowerCase()),
    });
  }, [account]);

  return null;
};

export default CrowdloanRewardsUpdater;
