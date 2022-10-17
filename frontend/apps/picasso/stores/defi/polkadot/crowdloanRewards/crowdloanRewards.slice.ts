import create from "zustand";
import BigNumber from "bignumber.js";

/** Address in KSM format => string address in PICA or ETH format */
export type CrowdloanContributionRecord = Record<
  string,
  {
    totalRewards: BigNumber;
    contributedAmount: BigNumber;
  }
>;
/** Address in PICA format => string address in PICA or ETH format */
export type CrowdloanAssociation = [string, string | null];

export enum CrowdloanStep {
  AssociateEth,
  AssociateKsm,
  Claim,
  None,
}
export interface CrowdloanRewardsSlice {
  // connected ksm account contributions
  // ksm format => values
  kusamaContributions: CrowdloanContributionRecord;
  // eth format => values
  // connected eth  account contributions
  ethereumContributions: CrowdloanContributionRecord;
  // pica format => eth or pica account
  // on chain associations
  onChainAssociations: CrowdloanAssociation[];
  // initialPayment
  initialPayment: BigNumber;
  // claimableAmount
  claimableAmount: BigNumber;
}

export const useCrowdloanRewardsSlice = create<CrowdloanRewardsSlice>(() => ({
  kusamaContributions: {},
  ethereumContributions: {},
  onChainAssociations: [],
  claimableRewards: {},
  claimedRewards: {},
  initialPayment: new BigNumber(0),
  claimableAmount: new BigNumber(0)
}));

export const setCrowdloanRewardsState = (
  updates: Partial<CrowdloanRewardsSlice>
) =>
  useCrowdloanRewardsSlice.setState((state) => ({
    ...state,
    ...updates,
  }));