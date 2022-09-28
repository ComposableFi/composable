import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { ApiPromise } from "@polkadot/api";
import { encodeAddress } from "@polkadot/util-crypto";
import BigNumber from "bignumber.js";
import { useMemo, useState, useEffect } from "react";
import { fetchClaimedRewards, fetchClaimableRewards } from "./crowdloanRewards";
import {
  CrowdloanStep,
  useCrowdloanRewardsSlice,
} from "./crowdloanRewards.slice";

export const useCrowdloanNextStep = (
  selectedPicassoAccount: string | undefined,
  selectedEthereumAccount: string | undefined
): CrowdloanStep => {
  const { ethereumContributions, kusamaContributions, onChainAssociations } =
    useCrowdloanRewardsSlice();

  return useMemo(() => {
    if (selectedPicassoAccount) {
      const selectedPICAinKSMFormat = encodeAddress(
        selectedPicassoAccount,
        SUBSTRATE_NETWORKS.kusama.ss58Format
      );

      if (selectedEthereumAccount) {
        const isEthAccountEligible =
          selectedEthereumAccount in ethereumContributions;
        const isKsmAccountEligible =
          selectedPICAinKSMFormat in kusamaContributions;

        if (isEthAccountEligible && !isKsmAccountEligible) {
          const isAssociatedPicassoAccount = onChainAssociations.find(
            ([_account, association]) => {
              return (
                association !== null && association === selectedEthereumAccount
              );
            }
          );

          const isConnectedPICAAccountAssociatedAsWell =
            isAssociatedPicassoAccount &&
            isAssociatedPicassoAccount[0] === selectedPicassoAccount;

          if (isConnectedPICAAccountAssociatedAsWell) {
            return CrowdloanStep.Claim;
          } else {
            return CrowdloanStep.AssociateEth;
          }
        } else if (isEthAccountEligible && isKsmAccountEligible) {
          // cant happen
          // two people having eth private key
          // and ksm private key as well.
          // prioritize ksm association
          const hasBeenAssociated = onChainAssociations.find(
            ([_account, association]) => {
              association === selectedPicassoAccount;
            }
          );

          if (hasBeenAssociated) {
            return CrowdloanStep.Claim;
          } else {
            return CrowdloanStep.AssociateKsm;
          }
        } else if (isKsmAccountEligible && !isEthAccountEligible) {
          const hasBeenAssociated = onChainAssociations.find(
            ([_account, association]) => {
              association === selectedPicassoAccount;
            }
          );

          if (hasBeenAssociated) {
            return CrowdloanStep.Claim;
          } else {
            return CrowdloanStep.AssociateKsm;
          }
        }
      } else {
        let isKsmEligible = selectedPICAinKSMFormat in kusamaContributions;

        if (isKsmEligible) {
          const hasBeenAssociated = onChainAssociations.find(
            ([_account, association]) => {
              association === selectedPicassoAccount;
            }
          );

          let isksmAssociatedAccountSameAsConnected =
            hasBeenAssociated &&
            hasBeenAssociated[0] === selectedPicassoAccount;

          if (isksmAssociatedAccountSameAsConnected) {
            return CrowdloanStep.Claim;
          } else {
            return CrowdloanStep.AssociateKsm;
          }
        }
      }
    }

    return CrowdloanStep.None;
  }, [
    ethereumContributions,
    kusamaContributions,
    onChainAssociations,
    selectedEthereumAccount,
    selectedPicassoAccount,
  ]);
};

export const useCrowdloanContributions = (
  _crowdloanStep: CrowdloanStep,
  ethAccount: string | undefined,
  picassoAccount: string | undefined
): { totalRewards: BigNumber; contributedAmount: BigNumber } => {
  const { ethereumContributions, kusamaContributions } =
    useCrowdloanRewardsSlice();

  return useMemo(() => {
    if (ethAccount && ethAccount in ethereumContributions)
      return ethereumContributions[ethAccount];

    if (picassoAccount) {
      const ksmFormat = encodeAddress(
        picassoAccount,
        SUBSTRATE_NETWORKS.kusama.ss58Format
      );
      if (ksmFormat && ksmFormat in kusamaContributions)
        return kusamaContributions[ksmFormat];
    }

    return {
      totalRewards: new BigNumber(0),
      contributedAmount: new BigNumber(0),
    };
  }, [ethAccount, ethereumContributions, kusamaContributions, picassoAccount]);
};

export const useClaimedAmount = (
  _crowdloanStep: CrowdloanStep,
  ethAccount: string | undefined,
  picassoAccount: string | undefined,
  api?: ApiPromise
): BigNumber => {
  const { ethereumContributions, kusamaContributions } =
    useCrowdloanRewardsSlice();

  const [claimedAmount, setClaimedAmount] = useState(new BigNumber(0));

  useEffect(() => {
    if (api && ethAccount && ethAccount in ethereumContributions) {
      fetchClaimedRewards(api, ethAccount.toLowerCase()).then(setClaimedAmount);
      return;
    }

    if (picassoAccount && api) {
      const ksmAddress = encodeAddress(
        picassoAccount,
        SUBSTRATE_NETWORKS.kusama.ss58Format
      );
      if (ksmAddress in kusamaContributions) {
        fetchClaimedRewards(api, ksmAddress).then(setClaimedAmount);
      }
      return;
    }
  }, [
    ethereumContributions,
    kusamaContributions,
    api,
    ethAccount,
    picassoAccount,
  ]);

  return claimedAmount;
};

export const useClaimableAmount = (
  _crowdloanStep: CrowdloanStep,
  ethAccount: string | undefined,
  picassoAccount: string | undefined,
  api: ApiPromise | undefined,
  initialPayment: BigNumber
): BigNumber => {
  const { onChainAssociations, ethereumContributions, kusamaContributions } =
    useCrowdloanRewardsSlice();

  const [claimableAmount, setClaimableAmount] = useState(new BigNumber(0));

  useEffect(() => {
    if (api) {
      if (picassoAccount) {
        const selectedAccountKsmFormat = encodeAddress(
          picassoAccount,
          SUBSTRATE_NETWORKS.kusama.ss58Format
        );

        const isEthEligible = ethAccount && ethAccount in ethereumContributions;
        const isKsmEligible = selectedAccountKsmFormat in kusamaContributions;

        if (isEthEligible && !isKsmEligible) {
          const ethAccountAssociation = onChainAssociations.find(
            ([_associatedAccount, associatedAccount]) => {
              return (
                associatedAccount?.toLowerCase() === ethAccount.toLowerCase()
              );
            }
          );

          if (ethAccountAssociation) {
            let isSameAsConnected = ethAccountAssociation[0] === picassoAccount;

            if (isSameAsConnected) {
              fetchClaimableRewards(api, picassoAccount).then(
                setClaimableAmount
              );
            } else {
              new BigNumber(0);
            }
          } else if (!ethAccountAssociation) {
            setClaimableAmount(
              ethereumContributions[ethAccount].totalRewards.times(
                initialPayment
              )
            );
          }
        } else if (!isEthEligible && isKsmEligible) {
          const onChainAssociation = onChainAssociations.find(
            ([_associatedAccount, associationAccount]) =>
              associationAccount === picassoAccount
          );

          if (onChainAssociation) {
            if (onChainAssociation[0] === picassoAccount) {
              fetchClaimableRewards(api, picassoAccount).then(
                setClaimableAmount
              );
            } else {
              console.log("Invalid Account Connected");
            }
          } else if (!onChainAssociation) {
            setClaimableAmount(
              kusamaContributions[selectedAccountKsmFormat].totalRewards.times(
                initialPayment
              )
            );
          }
        } else if (!isEthEligible && !isKsmEligible) {
          setClaimableAmount(new BigNumber(0));
        }
      } else {
        setClaimableAmount(new BigNumber(0));
      }
    }
  }, [
    ethAccount,
    picassoAccount,
    kusamaContributions,
    onChainAssociations,
    api,
    initialPayment,
    ethereumContributions,
  ]);

  return claimableAmount;
};
