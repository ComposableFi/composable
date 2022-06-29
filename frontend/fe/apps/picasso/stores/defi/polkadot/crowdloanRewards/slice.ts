import { RootState } from "@/stores/root";
import { createSlice, PayloadAction } from "@reduxjs/toolkit";

interface CrowdloanRewards {
  netVestedPICA: string;
  claimablePICA: string;
  claimedPICA: string;
  contribution: string;
}

export type AssociationMode = "relayChain" | "ethereum";

const initialState: {
  user: CrowdloanRewards;
  ui: {
    useAssociationMode: AssociationMode;
    isEligible: boolean;
  };
  constants: {
    initialPayment: string;
  };
  associatedWith: AssociationMode | null;
  evmAlreadyAssociated: boolean;
} = {
  ui: {
    useAssociationMode: "relayChain",
    isEligible: true,
  },
  user: {
    netVestedPICA: "0",
    claimablePICA: "0",
    claimedPICA: "0",
    contribution: "0",
  },
  constants: {
    initialPayment: "0",
  },
  associatedWith: null,
  evmAlreadyAssociated: false,
};

export const crowdloanRewardsSlice = createSlice({
  name: "crowdloanRewards",
  initialState,
  reducers: {
    setUseAssociationMode: (
      state,
      action: PayloadAction<{ useAssociationMode: AssociationMode }>
    ) => {
      const { useAssociationMode } = action.payload;
      state.ui.useAssociationMode = useAssociationMode;
    },
    setUserClaimEigibility: (
      state,
      action: PayloadAction<{ isEligible: boolean }>
    ) => {
      const { isEligible } = action.payload;
      state.ui.isEligible = isEligible;
    },
    setUserCrowdloanData: (
      state,
      action: PayloadAction<{
        netVestedPICA: string;
        claimablePICA: string;
        claimedPICA: string;
      }>
    ) => {
      const { netVestedPICA, claimablePICA, claimedPICA } = action.payload;
      state.user.claimablePICA = claimablePICA;
      state.user.netVestedPICA = netVestedPICA;
      state.user.claimedPICA = claimedPICA;
    },
    setUserClaimablePICA: (
      state,
      action: PayloadAction<{
        claimablePICA: string;
      }>
    ) => {
      const { claimablePICA } = action.payload;
      state.user.claimablePICA = claimablePICA;
    },
    setUserClaimedPICA: (
      state,
      action: PayloadAction<{
        claimedPICA: string;
      }>
    ) => {
      const { claimedPICA } = action.payload;
      state.user.claimedPICA = claimedPICA;
    },
    setUserNetVestedPICA: (
      state,
      action: PayloadAction<{
        netVestedPICA: string;
      }>
    ) => {
      const { netVestedPICA } = action.payload;
      state.user.netVestedPICA = netVestedPICA;
    },
    setUserAssociatedWith: (
      state,
      action: PayloadAction<{
        associatedWith: "relayChain" | "ethereum" | null;
      }>
    ) => {
      const { associatedWith } = action.payload;
      state.associatedWith = associatedWith;
    },
    setInitialPayment: (
      state,
      action: PayloadAction<{
        initialPayment: string;
      }>
    ) => {
      const { initialPayment } = action.payload;
      state.constants.initialPayment = initialPayment;
    },
    setUserContribution: (
      state,
      action: PayloadAction<{
        contribution: string;
      }>
    ) => {
      const { contribution } = action.payload;
      state.user.contribution = contribution;
    },
    setEvmAlreadyAssociated: (
      state,
      action: PayloadAction<{
        evmAlreadyAssociated: boolean;
      }>
    ) => {
      const { evmAlreadyAssociated } = action.payload;
      state.evmAlreadyAssociated = evmAlreadyAssociated;
    },
  },
});

export const {
  setUseAssociationMode,
  setUserCrowdloanData,
  setUserAssociatedWith,
  setUserClaimEigibility,
  setUserClaimablePICA,
  setUserClaimedPICA,
  setUserNetVestedPICA,
  setUserContribution,
  setInitialPayment,
  setEvmAlreadyAssociated,
} = crowdloanRewardsSlice.actions;

export const selectCrowdloadRewardsUserInfo = (state: RootState) =>
  state.crowdloanRewards.user;
export const selectCrowdloanRewardsUIHelper = (state: RootState) =>
  state.crowdloanRewards.ui;
export const selectCrowdloanRewardsUserAssociation = (state: RootState) =>
  state.crowdloanRewards.associatedWith;
export const selectCrowdloanRewardsinitialPayment = (state: RootState) =>
  state.crowdloanRewards.constants.initialPayment;

export const selectIsEvmAlreadyAssociated = (state: RootState) =>
  state.crowdloanRewards.evmAlreadyAssociated;

export default crowdloanRewardsSlice.reducer;
