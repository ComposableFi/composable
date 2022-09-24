
import { useKusamaAccounts, usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { useEffect } from "react";
import { useBlockchainProvider } from "bi-lib";
import { fromPerbill } from "shared";
import { fetchAssociations, fetchClaimableAndClaimedRewards, getConnectedAccountState } from "./crowdloanRewards";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { encodeAddress } from "@polkadot/util-crypto";
import {
  setCrowdloanRewardsState,
  useCrowdloanRewardsSlice,
} from "./crowdloanRewards.slice";
// Import static JSON files
import rewards from "@/defi/polkadot/constants/pica-rewards.json";
import contributions from "@/defi/polkadot/constants/contributions.json";
import contributionsDev from "@/defi/polkadot/constants/contributions-dev.json";
import rewardsDev from "@/defi/polkadot/constants/pica-rewards-dev.json";
/**
 * Check for contributions in JSON
 * @param account ethereum or kusama format address
 * @returns string | undefined
 */
export const presentInContributors = (account: string): string | undefined =>
  (contributions.contributedAmounts as Record<string, string>)[account];
/**
 * Check for rewards in JSON
 * @param account ethereum or kusama format address
 * @returns string | undefined
 */
export const presentInRewards = (account: string): string | undefined =>
  (rewards as Record<string, string>)[account];
/**
 * Check for rewards in JSON (dev env)
 * @param account ethereum or kusama format address
 * @returns string | undefined
 */
export const presentInRewardsDev = (account: string): string | undefined =>
  (rewardsDev as Record<string, string>)[account];
/**
 * Check for constributions in JSON (dev env)
 * @param account ethereum or kusama format address
 * @returns string | undefined
 */
export const presentInContributorsDev = (account: string): string | undefined =>
  (contributionsDev.contributedAmounts as Record<string, string>)[account];

const DEFAULT_EVM_ID = 1;

const CrowdloanRewardsUpdater = () => {
  const { account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const kusamaAccounts = useKusamaAccounts();
  const selectedAccount = useSelectedAccount();
  const { parachainApi, accounts } = usePicassoProvider();

  useEffect(() => {
    if (parachainApi) {
      const initialPayment = fromPerbill(
        parachainApi.consts.crowdloanRewards.initialPayment.toString()
      ).div(100);
      console.log('initialPayment.toString()', initialPayment.toString());
      setCrowdloanRewardsState({ initialPayment });
    }
  }, [parachainApi]);

  useEffect(() => {
    if (parachainApi && accounts.length > 0) {
      const addresses = accounts.map(
        (_account) => _account.address
      );
      fetchAssociations(
        parachainApi,
        addresses.filter((address) => !address.startsWith("0x"))
      )
        .then((onChainAssociations) => {
          setCrowdloanRewardsState({ onChainAssociations });
        })
        .catch(console.error);
    }
  }, [selectedAccount, parachainApi, accounts]);

  const { onChainAssociations } = useCrowdloanRewardsSlice();

  useEffect(() => {
    const ethereumAccount = account ? account.toLowerCase() : undefined;
    if (!ethereumAccount) return;
    if (!parachainApi) return;
    if (accounts.length > 0 && !(onChainAssociations.length > 0)) return;

    const accountsState = accounts.filter(acc => !acc.address.startsWith("0x")).map((picaAccount) => {
      return getConnectedAccountState(
        encodeAddress(picaAccount.address, SUBSTRATE_NETWORKS.kusama.ss58Format),
        "kusama",
        process.env.NODE_ENV as any,
        onChainAssociations
      );
    });

    accountsState.push(
      getConnectedAccountState(
        ethereumAccount,
        "ethereum",
        process.env.NODE_ENV as any,
        onChainAssociations
      )
    );

    if (
      accountsState.some(
        (account) => account.crowdloanSelectedAccountStatus === "canClaim"
      )
    ) {
      fetchClaimableAndClaimedRewards(parachainApi, accountsState).then(
        (accountsState) => {
          setCrowdloanRewardsState({ accountsState });
        }
      );
    } else {
      setCrowdloanRewardsState({
        accountsState,
      });
    }
  }, [
    selectedAccount,
    kusamaAccounts,
    account,
    parachainApi,
    onChainAssociations,
    accounts
  ]);

  return null;
};

export default CrowdloanRewardsUpdater;
