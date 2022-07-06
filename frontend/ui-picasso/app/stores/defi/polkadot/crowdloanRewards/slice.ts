import { NamedSet } from "zustand/middleware";
import { StoreSlice } from "../../../types";

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
    setUserClaimEligibility: (isEligible: boolean) => void;
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
      set((state) => {
        state.crowdloanRewards.ui.useAssociationMode = useAssociationMode;

        return state;
      });
    },
    setUserClaimEligibility: (isEligible: boolean) => {
      set((state) => {
        state.crowdloanRewards.ui.isEligible = isEligible;

        return state;
      });
    },
    setUserCrowdloanData: (
      netVestedPICA: string,
      claimablePICA: string,
      claimedPICA: string
    ) => {
      set((state) => {
        state.crowdloanRewards.user.claimablePICA = claimablePICA;
        state.crowdloanRewards.user.netVestedPICA = netVestedPICA;
        state.crowdloanRewards.user.claimedPICA = claimedPICA;

        return state;
      });
    },
    setUserClaimablePICA: (claimablePICA: string) => {
      set((state) => {
        state.crowdloanRewards.user.claimablePICA = claimablePICA;

        return state;
      });
    },
    setUserClaimedPICA: (claimedPICA: string) => {
      set((state) => {
        state.crowdloanRewards.user.claimedPICA = claimedPICA;

        return state;
      });
    },
    setUserNetVestedPICA: (netVestedPICA: string) => {
      set((state) => {
        state.crowdloanRewards.user.netVestedPICA = netVestedPICA;

        return state;
      });
    },
    setUserAssociatedWith: (
      associatedWith: "relayChain" | "ethereum" | null
    ) => {
      set((state) => {
        state.crowdloanRewards.associatedWith = associatedWith;

        return state;
      });
    },
    setInitialPayment: (initialPayment: string) => {
      set((state) => {
        state.crowdloanRewards.constants.initialPayment = initialPayment;

        return state;
      });
    },
    setUserContribution: (contribution: string) => {
      set((state) => {
        state.crowdloanRewards.user.contribution = contribution;

        return state;
      });
    },
    setEvmAlreadyAssociated: (evmAlreadyAssociated: boolean) => {
      set((state) => {
        state.crowdloanRewards.evmAlreadyAssociated = evmAlreadyAssociated;

        return state;
      });
    },
  },
});
