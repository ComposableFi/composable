import { NamedSet } from "zustand/middleware";
import { AllSlices, StoreSlice } from "../../../types";

interface CrowdloanRewards {
  netVestedPICA: string;
  claimablePICA: string;
  claimedPICA: string;
  contribution: string;
}

interface CrowdloanRewardsState {
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

export interface CrowdloanRewardsSlice {
  crowdloanRewards: CrowdloanRewardsState & {
    setUseAssociationMode: (useAssociationMode: AssociationMode) => void;
    setUserClaimEigibility: (isEligible: boolean) => void;
    setUserCrowdloanData: (
      netVestedPICA: string,
      claimablePICA: string,
      claimedPICA: string
    ) => void;
    setUserClaimablePICA: (claimablePICA: string) => void;
    setUserClaimedPICA: (claimedPICA: string) => void;
    setUserNetVestedPICA: (etVestedPICA: string) => void;
    setUserAssociatedWith: (
      associatedWith: "relayChain" | "ethereum" | null
    ) => void;
    setInitialPayment: (initialPayment: string) => void;
    setUserContribution: (contribution: string) => void;
    setEvmAlreadyAssociated: (evmAlreadyAssociated: boolean) => void;
  };
}

export const createCrowdloanRewardsSlice: StoreSlice<CrowdloanRewardsSlice> = (
  set: NamedSet<CrowdloanRewardsSlice>
) => ({
  crowdloanRewards: {
    ...initialState,
    setUseAssociationMode: (useAssociationMode: AssociationMode) => {
      set((state: AllSlices) => {
        state.crowdloanRewards.ui.useAssociationMode = useAssociationMode;
      });
    },
    setUserClaimEigibility: (isEligible: boolean) => {
      set((state: AllSlices) => {
        state.crowdloanRewards.ui.isEligible = isEligible;
      });
    },
    setUserCrowdloanData: (
      netVestedPICA: string,
      claimablePICA: string,
      claimedPICA: string
    ) => {
      set((state: AllSlices) => {
        state.crowdloanRewards.user.claimablePICA = claimablePICA;
        state.crowdloanRewards.user.netVestedPICA = netVestedPICA;
        state.crowdloanRewards.user.claimedPICA = claimedPICA;
      });
    },
    setUserClaimablePICA: (claimablePICA: string) => {
      set((state: AllSlices) => {
        state.crowdloanRewards.user.claimablePICA = claimablePICA;
      });
    },
    setUserClaimedPICA: (claimedPICA: string) => {
      set((state: AllSlices) => {
        state.crowdloanRewards.user.claimedPICA = claimedPICA;
      });
    },
    setUserNetVestedPICA: (netVestedPICA: string) => {
      set((state: AllSlices) => {
        state.crowdloanRewards.user.netVestedPICA = netVestedPICA;
      });
    },
    setUserAssociatedWith: (
      associatedWith: "relayChain" | "ethereum" | null
    ) => {
      set((state: AllSlices) => {
        state.crowdloanRewards.associatedWith = associatedWith;
      });
    },
    setInitialPayment: (initialPayment: string) => {
      set((state: AllSlices) => {
        state.crowdloanRewards.constants.initialPayment = initialPayment;
      });
    },
    setUserContribution: (contribution: string) => {
      set((state: AllSlices) => {
        state.crowdloanRewards.user.contribution = contribution;
      });
    },
    setEvmAlreadyAssociated: (evmAlreadyAssociated: boolean) => {
      set((state: AllSlices) => {
        state.crowdloanRewards.evmAlreadyAssociated = evmAlreadyAssociated;
      });
    },
  },
});
