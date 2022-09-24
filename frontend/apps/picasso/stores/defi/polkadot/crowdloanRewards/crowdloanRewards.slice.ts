import create from "zustand";
import BigNumber from "bignumber.js";

export type AccountAssociation = {
  account: string;
  association: string | null;
};
export type OnChainAccountAssociation = {
  account: string;
  association: string | null;
};
export type CrowdloanSelectedAccountStatus =
  | "canClaim"
  | "canAssociate"
  | "ineligible";
export type CrowdloanAccountAccountState = {
  address: {
    ksmOrEthAddress: string;
    source: "ethereum" | "kusama";
    picassoFormat?: string;
  };
  crowdloanSelectedAccountStatus: CrowdloanSelectedAccountStatus;
  amountContributed: BigNumber;
  totalRewards: BigNumber;
  availableToClaim: BigNumber;
  claimedRewards: BigNumber;
};
export interface CrowdloanRewardsSlice {
  // Association strategy selected
  associationStrategySelected: "ethereum" | "relayChain";
  // on chain associations
  onChainAssociations: Array<OnChainAccountAssociation>;
  // account states
  accountsState: Array<CrowdloanAccountAccountState>;
  // initialPayment
  initialPayment: BigNumber;
}

export const useCrowdloanRewardsSlice = create<CrowdloanRewardsSlice>(() => ({
  associationStrategySelected: "relayChain",
  onChainAssociations: [],
  accountsState: [],
  initialPayment: new BigNumber(0),
}));

export const setCrowdloanRewardsState = (
  updates: Partial<CrowdloanRewardsSlice>
) =>
  useCrowdloanRewardsSlice.setState((state) => ({
    ...state,
    ...updates,
  }));

export function useAccountState(
  account: string,
  source: "kusama" | "ethereum"
): CrowdloanAccountAccountState | undefined {
  const { accountsState } = useCrowdloanRewardsSlice();

  return accountsState.find((accountState) => {
    return source === "ethereum"
      ? accountState.address.ksmOrEthAddress === account
      : accountState.address.picassoFormat
      ? accountState.address.picassoFormat === account
      : false;
  });
}

export const setAssociatedEthereum = (
  ethereumAddress: string,
  picassoAssociatedAddress: string
) =>
  useCrowdloanRewardsSlice.setState((state) => ({
    ...state,
    accountsState: state.accountsState.map((accountState) => {
      accountState.address.picassoFormat = picassoAssociatedAddress;
      accountState.crowdloanSelectedAccountStatus =
        accountState.address.ksmOrEthAddress === ethereumAddress
          ? "canClaim"
          : accountState.crowdloanSelectedAccountStatus;
      const oldAvailableToClaim = accountState.availableToClaim.plus(0);
      accountState.availableToClaim = new BigNumber(0);
      accountState.claimedRewards = oldAvailableToClaim;

      return accountState;
    }),
  }));

export const setAssociatedKsm = (picassoAssociatedAddress: string) =>
  useCrowdloanRewardsSlice.setState((state) => ({
    ...state,
    accountsState: state.accountsState.map((accountState) => {
      accountState.crowdloanSelectedAccountStatus =
        accountState.address.picassoFormat &&
        accountState.address.picassoFormat === picassoAssociatedAddress
          ? "canClaim"
          : accountState.crowdloanSelectedAccountStatus;
      const oldAvailableToClaim = accountState.availableToClaim.plus(0);
      accountState.availableToClaim = new BigNumber(0);
      accountState.claimedRewards = oldAvailableToClaim;

      return accountState;
    }),
  }));
