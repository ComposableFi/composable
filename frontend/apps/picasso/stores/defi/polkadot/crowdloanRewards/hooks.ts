import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { ApiPromise } from "@polkadot/api";
import { decodeAddress, encodeAddress } from "@polkadot/util-crypto";
import { useMemo, useState, useEffect, useCallback } from "react";
import {
  fetchClaimedRewards,
  fetchClaimableRewards,
  findAssociation,
  isAssociatedAccountSameAsConnectedAccount,
  findAssociatedByAccount,
} from "./crowdloanRewards";
import {
  CrowdloanAssociation,
  CrowdloanStep,
  useCrowdloanRewardsSlice,
} from "./crowdloanRewards.slice";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "shared";

export const toKusamaAddressFormat = (address: string) =>
  encodeAddress(address, SUBSTRATE_NETWORKS.kusama.ss58Format);

export const useCrowdloanRewardsContributionAndRewards = (
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
      const ksmFormat = toKusamaAddressFormat(picassoAccount);
      if (ksmFormat && ksmFormat in kusamaContributions)
        return kusamaContributions[ksmFormat];
    }

    return {
      totalRewards: new BigNumber(0),
      contributedAmount: new BigNumber(0),
    };
  }, [ethAccount, ethereumContributions, kusamaContributions, picassoAccount]);
};

/**
 * Given a picasso or ethereum account
 * update its claimed shares in zustand store
 * @param {string | undefined} ethAccount connected ethereum account
 * @param {string | undefined} picassoAccount connected picasso account
 * @param {ApiPromise} api parachain api object
 * @returns {BigNumber} amount of rewards claimed
 */
export const useCrowdloanRewardsClaimedRewards = (
  ethAccount: string | undefined,
  picassoAccount: string | undefined,
  api?: ApiPromise
): BigNumber => {
  const [claimedAmount, setClaimedAmount] = useState(new BigNumber(0));
  const { isEthAccountEligible, isPicassoAccountEligible } =
    useCrowdloanRewardsEligibility(ethAccount, picassoAccount);

  const fetchClaimed = useCallback(() => {
    if (api && isEthAccountEligible) {
      fetchClaimedRewards(api, (ethAccount as string).toLowerCase()).then(
        setClaimedAmount
      );
    }

    if (api && isPicassoAccountEligible) {
      fetchClaimedRewards(api, picassoAccount as string).then(setClaimedAmount);
    }
  }, [
    api,
    ethAccount,
    isEthAccountEligible,
    isPicassoAccountEligible,
    picassoAccount,
  ]);

  useEffect(() => {
    fetchClaimed();

    const fetchClaimedInterval = setInterval(() => {
      fetchClaimed();
    }, 30_000);

    return function () {
      clearInterval(fetchClaimedInterval);
    };
  }, [fetchClaimed]);

  return claimedAmount;
};

export const useCrowdloanRewardsClaimableRewards = (
  _crowdloanStep: CrowdloanStep,
  ethAccount: string | undefined,
  picassoAccount: string | undefined,
  api: ApiPromise | undefined,
  initialPayment: BigNumber
): BigNumber => {
  const [claimableAmount, setClaimableAmount] = useState(new BigNumber(0));
  const { isEthAccountEligible, isPicassoAccountEligible } =
    useCrowdloanRewardsEligibility(ethAccount, picassoAccount);

  const { onChainAssociations, ethereumContributions, kusamaContributions } =
    useCrowdloanRewardsSlice();

  const updateClaimable = useCallback(async (): Promise<BigNumber> => {
    if (!api) return new BigNumber(0);

    const vestingStep =
      await api.consts.crowdloanRewards.vestingStep.toNumber();
    const startTimestampOption =
      await api.query.crowdloanRewards.vestingTimeStart();

    if (startTimestampOption.isNone) {
      return new BigNumber(0)
    }

    const startTimestamp = new BigNumber(startTimestampOption.toString());
    const nowU64 = await api.query.timestamp.now();
    const now = new BigNumber(nowU64.toString());

    let totalRewards = new BigNumber(0),
      canClaim = false,
      accountAssociation: CrowdloanAssociation | undefined;
    if (isEthAccountEligible && !isPicassoAccountEligible) {
      totalRewards = ethereumContributions[ethAccount as string].totalRewards;
      accountAssociation = findAssociation(
        ethAccount,
        "ethereum",
        onChainAssociations
      );
      canClaim = isAssociatedAccountSameAsConnectedAccount(
        picassoAccount as string,
        accountAssociation
      );
    } else if (isPicassoAccountEligible && !isEthAccountEligible) {
      const ksmFormat = toKusamaAddressFormat(picassoAccount as string);
      totalRewards = kusamaContributions[ksmFormat].totalRewards;
      accountAssociation = findAssociation(
        picassoAccount,
        "picasso",
        onChainAssociations
      );
      canClaim = isAssociatedAccountSameAsConnectedAccount(
        picassoAccount as string,
        accountAssociation
      );
    }

    if (canClaim) {
      return await fetchClaimableRewards(api, picassoAccount as string);
    } else if (
      !accountAssociation &&
      (isEthAccountEligible || isPicassoAccountEligible)
    ) {
      const rewardsCodec = await api.query.crowdloanRewards.rewards(
        isEthAccountEligible ? {
          Ethereum: ethAccount
        } : {
          RelayChain: encodeAddress(decodeAddress(picassoAccount), SUBSTRATE_NETWORKS.kusama.ss58Format)
        }
      );
      const rewards = rewardsCodec.toJSON() as {
        vestingPeriod: number;
        total: string;
        claimed: string;
      } | null;

      if (!rewards) {
        return new BigNumber(0);
      }

      const vestingPoint = now.minus(startTimestamp);
      if (vestingPoint.gt(rewards.vestingPeriod)) {
        return fromChainIdUnit(BigInt(rewards.total))
      } else {
        const upfront = totalRewards.times(initialPayment);
        const vestingWindow = vestingPoint.minus(vestingPoint.mod(vestingStep));
        const vested = new BigNumber(fromChainIdUnit(BigInt(rewards.total))).minus(upfront);
        return upfront.plus(
          vested.times(vestingWindow.div(rewards.vestingPeriod))
        ).dp(4)
      }
    }
    return new BigNumber(0);
  }, [
    api,
    ethAccount,
    ethereumContributions,
    initialPayment,
    isEthAccountEligible,
    isPicassoAccountEligible,
    kusamaContributions,
    onChainAssociations,
    picassoAccount,
  ]);

  useEffect(() => {
    const interval = setInterval(() => {
      updateClaimable().then(setClaimableAmount);
    }, 30_000);

    updateClaimable().then(setClaimableAmount);

    return function () {
      clearInterval(interval);
    };
  }, [updateClaimable]);

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
export const useCrowdloanRewardsEligibility = (
  ethAccount?: string,
  picassoAccount?: string
): { isEthAccountEligible: boolean; isPicassoAccountEligible: boolean } => {
  const { ethereumContributions, kusamaContributions } =
    useCrowdloanRewardsSlice();

  return useMemo(() => {
    const isPicassoAccountEligible =
      picassoAccount !== undefined &&
      toKusamaAddressFormat(picassoAccount) in kusamaContributions;
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
 * @param {string | undefined} ethAccount Connected Ethereum Account
 * @param {ConnectedAccount | undefined} connectedPicassoAccount Currently Selected Polkadot Account
 * @param {ConnectedAccount[] | undefined} connectedPicassoAccounts All Connected Polkadot Accounts
 * @returns
 */
export const useCrowdloanRewardsEthereumAddressAssociatedAccount = (
  ethereumAccount?: string,
  connectedPicassoAccount?: InjectedAccountWithMeta,
  connectedPicassoAccounts?: InjectedAccountWithMeta[]
): InjectedAccountWithMeta | undefined => {
  const { onChainAssociations } = useCrowdloanRewardsSlice();

  return useMemo(() => {
    if (ethereumAccount && connectedPicassoAccount) {
      const ethAssociation = findAssociation(
        ethereumAccount,
        "ethereum",
        onChainAssociations
      );

      if (
        isAssociatedAccountSameAsConnectedAccount(
          connectedPicassoAccount.address,
          ethAssociation
        )
      ) {
        return connectedPicassoAccount;
      } else if (
        ethAssociation &&
        connectedPicassoAccounts &&
        connectedPicassoAccounts.length > 0
      ) {
        return connectedPicassoAccounts.find(({ address }) => {
          return address === ethAssociation[0];
        });
      } else if (!ethAssociation) {
        const connectedKsmHasOtherAssociation = findAssociatedByAccount(
          connectedPicassoAccount.address,
          onChainAssociations
        );
        if (
          connectedKsmHasOtherAssociation &&
          connectedKsmHasOtherAssociation[1] === null
        ) {
          return connectedPicassoAccount;
        } else {
          return {
            address: "DOT Wallet is associated with",
            meta: {
              genesisHash: null,
              name: connectedKsmHasOtherAssociation?.[1]
                ? connectedKsmHasOtherAssociation?.[1]
                : "-",
              source: "none",
            },
            type: "ethereum",
          };
        }
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
 * of crowdloan rewards
 * @param {ApiPromise} api API object
 * @returns {boolean}
 */
export const useCrowdloanRewardsHasStarted = (api?: ApiPromise): boolean => {
  const [hasStarted, setHasStarted] = useState(false);

  useEffect(() => {
    if (api) {
      api.query.crowdloanRewards.vestingTimeStart((timeStart) => {
        const bn = new BigNumber(timeStart.value.toString()); // converting to string and then BN (type safe)
        setHasStarted(bn.toNumber() < Date.now());
      });
    }
  }, [api]);

  return hasStarted;
};

export const useCrowdloanRewardsStepGivenConnectedAccounts = (
  selectedPicassoAccount: string | undefined,
  selectedEthereumAccount: string | undefined
): CrowdloanStep => {
  const { isEthAccountEligible, isPicassoAccountEligible } =
    useCrowdloanRewardsEligibility(
      selectedEthereumAccount,
      selectedPicassoAccount
    );
  const { onChainAssociations } = useCrowdloanRewardsSlice();

  return useMemo(() => {
    if (isEthAccountEligible && !isPicassoAccountEligible) {
      const ethAssociation = findAssociation(
        selectedEthereumAccount,
        "ethereum",
        onChainAssociations
      );

      if (ethAssociation) {
        if (
          isAssociatedAccountSameAsConnectedAccount(
            selectedPicassoAccount,
            ethAssociation
          )
        ) {
          return CrowdloanStep.Claim;
        } else {
          return CrowdloanStep.None;
        }
      } else {
        const ksmAssociation = findAssociatedByAccount(
          selectedPicassoAccount,
          onChainAssociations
        );
        if (ksmAssociation && ksmAssociation[1] === null) {
          return CrowdloanStep.AssociateEth;
        } else {
          return CrowdloanStep.None;
        }
      }
    } else if (isPicassoAccountEligible && !isEthAccountEligible) {
      const ksmAssociation = findAssociation(
        selectedPicassoAccount,
        "picasso",
        onChainAssociations
      );

      if (ksmAssociation) {
        if (
          isAssociatedAccountSameAsConnectedAccount(
            selectedPicassoAccount,
            ksmAssociation
          )
        ) {
          return CrowdloanStep.Claim;
        } else {
          return CrowdloanStep.None;
        }
      } else {
        return CrowdloanStep.AssociateKsm;
      }
    } else {
      return CrowdloanStep.None;
    }
  }, [
    isEthAccountEligible,
    isPicassoAccountEligible,
    onChainAssociations,
    selectedEthereumAccount,
    selectedPicassoAccount,
  ]);
};
