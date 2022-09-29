import { ConnectedAccount } from "@/../../packages/substrate-react/src";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { ApiPromise } from "@polkadot/api";
import { encodeAddress } from "@polkadot/util-crypto";
import BigNumber from "bignumber.js";
import { useMemo, useState, useEffect } from "react";
import { fetchClaimedRewards, fetchClaimableRewards } from "./crowdloanRewards";
import {
  CrowdloanStep,
  setCrowdloanRewardsState,
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
              return association === selectedPicassoAccount;
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
              return association === selectedPicassoAccount;
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
              return association === selectedPicassoAccount;
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
  const {
    onChainAssociations,
    ethereumContributions,
    kusamaContributions,
    claimableAmount,
  } = useCrowdloanRewardsSlice();

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
                (claimableAmount) => {
                  setCrowdloanRewardsState({ claimableAmount });
                }
              );
            } else {
              setCrowdloanRewardsState({ claimableAmount: new BigNumber(0) });
            }
          } else if (!ethAccountAssociation) {
            const claimableAmount =
              ethereumContributions[ethAccount].totalRewards.times(
                initialPayment
              );
            setCrowdloanRewardsState({ claimableAmount });
          }
        } else if (!isEthEligible && isKsmEligible) {
          const onChainAssociation = onChainAssociations.find(
            ([_associatedAccount, associationAccount]) =>
              associationAccount === picassoAccount
          );

          if (onChainAssociation) {
            if (onChainAssociation[0] === picassoAccount) {
              fetchClaimableRewards(api, picassoAccount).then(
                (claimableAmount) => {
                  setCrowdloanRewardsState({ claimableAmount });
                }
              );
            } else {
              console.log("Invalid Account Connected");
            }
          } else if (!onChainAssociation) {
            const claimableAmount =
              kusamaContributions[selectedAccountKsmFormat].totalRewards.times(
                initialPayment
              );
            setCrowdloanRewardsState({ claimableAmount });
          }
        } else if (!isEthEligible && !isKsmEligible) {
          setCrowdloanRewardsState({ claimableAmount: new BigNumber(0) });
        }
      } else {
        setCrowdloanRewardsState({ claimableAmount: new BigNumber(0) });
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

/**
 * Given ethereum and picasso accounts
 * check whether they are present in
 * rewards and contributors list
 * @param ethAccount
 * @param picassoAccount
 * @returns { isEthAccountEligible: boolean; isPicassoAccountEligible: boolean }
 */
export const useEligibility = (
  ethAccount?: string,
  picassoAccount?: string
): { isEthAccountEligible: boolean; isPicassoAccountEligible: boolean } => {
  const { ethereumContributions, kusamaContributions } =
    useCrowdloanRewardsSlice();

  return useMemo(() => {
    const isPicassoAccountEligible =
      picassoAccount !== undefined &&
      encodeAddress(picassoAccount, SUBSTRATE_NETWORKS.kusama.ss58Format) in
        kusamaContributions;
    const isEthAccountEligible =
      ethAccount !== undefined && ethAccount in ethereumContributions;

    return {
      isEthAccountEligible,
      isPicassoAccountEligible,
    };
  }, [ethereumContributions, kusamaContributions, ethAccount, picassoAccount]);
};

/**
 * Given an ethereum address return the connected and associated Polkadot account
 * otherwise return currently connected one to associate
 * @param ethAccount Connected Ethereum Account
 * @param connectedPicassoAccount Currently Selected Polkadot Account
 * @param connectedPicassoAccounts All Connected Polkadot Accounts
 * @returns
 */
export const useEthereumAssociatedAccount = (
  ethereumAccount?: string,
  connectedPicassoAccount?: ConnectedAccount,
  connectedPicassoAccounts?: ConnectedAccount[]
): ConnectedAccount | undefined => {
  const { onChainAssociations } = useCrowdloanRewardsSlice();

  return useMemo(() => {
    if (ethereumAccount && connectedPicassoAccount) {
      const ethAssociation = onChainAssociations.find(
        ([_account, _association]) => {
          return (
            _association !== null &&
            _association.toLowerCase() === ethereumAccount.toLowerCase()
          );
        }
      );

      if (
        ethAssociation &&
        ethAssociation[0] === connectedPicassoAccount.address
      ) {
        return connectedPicassoAccount;
      } else if (
        ethAssociation &&
        connectedPicassoAccounts &&
        connectedPicassoAccounts.length > 0
      ) {
        return connectedPicassoAccounts.find(({ address, name }) => {
          return address === ethAssociation[0];
        });
      }
    }

    return undefined;
  }, [
    onChainAssociations,
    ethereumAccount,
    connectedPicassoAccount,
    connectedPicassoAccounts,
  ]);
};

/**
 * Returns the vesting start time
 * of the
 * @param api API object
 * @returns unix timestamp in number
 */
export const useVestingTimeStart = (api?: ApiPromise): number => {
  const [vestingTimeStart, setVestingTimeStart] = useState(-1);

  useEffect(() => {
    if (api) {
      api.query.crowdloanRewards.vestingTimeStart((timeStart) => {
        const bn = new BigNumber(timeStart.value.toString());
        setVestingTimeStart(bn.toNumber());
      });
    }
  }, [api]);

  return vestingTimeStart;
};
