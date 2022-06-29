import { PalletsContext } from "@/defi/polkadot/context/PalletsContext";
import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";
import { useKusamaAccounts, usePicassoProvider } from "@/defi/polkadot/hooks";
import { useContext, useEffect } from "react";
import { useDispatch } from "react-redux";
import {
  setUserAssociatedWith,
  setUserClaimEigibility,
  setUserClaimablePICA,
  setUserNetVestedPICA,
  setInitialPayment,
  selectCrowdloanRewardsUIHelper,
  setUserContribution,
  setUserClaimedPICA,
  setEvmAlreadyAssociated,
} from "./slice";
import rewards from "@/defi/polkadot/constants/pica-rewards.json";
import contributions from "@/defi/polkadot/constants/contributions.json";
import devRewards from "@/defi/polkadot/constants/pica-rewards-dev.json";
import { useAppSelector } from "@/hooks/store";
import { useBlockchainProvider } from "@integrations-lib/core";

const DEFAULT_EVM_ID = 1;

const CrowdloanRewardsUpdater = () => {
  const appDispatch = useDispatch();
  const { account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const kusamaAccounts = useKusamaAccounts();
  const { selectedAccount } = useContext(ParachainContext);
  const picassoProvider = usePicassoProvider();
  const crUiState = useAppSelector(selectCrowdloanRewardsUIHelper);
  const { crowdloanRewards } = useContext(PalletsContext);

  useEffect(() => {
    if (
      crowdloanRewards &&
      kusamaAccounts.length &&
      account &&
      selectedAccount != -1
    ) {
      let promises: Promise<any>[] = [];
      kusamaAccounts.forEach((account) => {
        promises.push(crowdloanRewards.association(account.address));
      });

      Promise.all(promises).then((associations) => {
        associations = associations.filter((i) => !!i);

        let evmAlreadyAssociated = false;
        if (associations.length) {
          associations.forEach((assoc) => {
            if (
              assoc.Ethereum &&
              assoc.Ethereum.toLowerCase() === account.toLowerCase()
            ) {
              evmAlreadyAssociated = true;
            }
          });
        } else {
          evmAlreadyAssociated = false;
        }

        appDispatch(
          setEvmAlreadyAssociated({
            evmAlreadyAssociated,
          })
        );
      });
    }
  }, [account, kusamaAccounts.length, crowdloanRewards, selectedAccount]);

  useEffect(() => {
    if (crowdloanRewards) {
      crowdloanRewards.queryInitialPayment().then((ip) => {
        console.log("initial payment", ip);
        appDispatch(setInitialPayment({ initialPayment: ip }));
      });
    }
  }, [crowdloanRewards]);

  useEffect(() => {
    const { accounts, apiStatus, parachainApi } = picassoProvider;
    if (
      selectedAccount !== -1 &&
      accounts.length &&
      apiStatus === "connected" &&
      parachainApi &&
      crowdloanRewards
    ) {
      // dispatch something
      crowdloanRewards
        .association(accounts[selectedAccount].address)
        .then((association: any) => {
          if (association === null) {
            appDispatch(setUserAssociatedWith({ associatedWith: null }));
            appDispatch(setUserClaimEigibility({ isEligible: true }));
          } else {
            appDispatch(
              setUserAssociatedWith({
                associatedWith: !!association.Ethereum
                  ? "ethereum"
                  : "relayChain",
              })
            );
            appDispatch(setUserClaimEigibility({ isEligible: true }));
          }
        })
        .catch((err: any) => {
          console.log("ye error", err);
        });
    }
  }, [
    selectedAccount,
    picassoProvider.apiStatus,
    picassoProvider.accounts.length,
    crowdloanRewards,
  ]);

  useEffect(() => {
    const { accounts, apiStatus, parachainApi } = picassoProvider;
    if (
      selectedAccount !== -1 &&
      accounts.length &&
      apiStatus === "connected" &&
      parachainApi &&
      crowdloanRewards
    ) {
      crowdloanRewards
        .queryAvailableToClaim(accounts[selectedAccount].address)
        .then((availableClaim) => {
          appDispatch(
            setUserClaimablePICA({
              claimablePICA: availableClaim,
            })
          );
        });
    }
  }, [
    selectedAccount,
    picassoProvider.apiStatus,
    picassoProvider.accounts.length,
    crowdloanRewards,
  ]);

  useEffect(() => {
    let netVestedPICA = "0";
    let contribution = "0";
    if (account && crUiState.useAssociationMode === "ethereum") {
      const addr = account.toLowerCase();
      if (addr && (rewards as any)[addr]) {
        netVestedPICA = (rewards as any)[addr];
      }
      // dev
      if (process.env.VERCEL_ENV !== "production") {
        if (addr && (devRewards as any)[addr]) {
          netVestedPICA = (devRewards as any)[addr];
        }
      }

      if (addr && (contributions.stablesContributedAmounts as any)[addr]) {
        contribution = (contributions.stablesContributedAmounts as any)[addr];
      }
    } else if (
      selectedAccount !== -1 &&
      kusamaAccounts.length &&
      crUiState.useAssociationMode === "relayChain"
    ) {
      const addr = kusamaAccounts[selectedAccount].address;

      if (addr && (rewards as any)[addr]) {
        netVestedPICA = (rewards as any)[addr];
      }
      // dev
      if (process.env.VERCEL_ENV !== "production") {
        if (addr && (devRewards as any)[addr]) {
          netVestedPICA = (devRewards as any)[addr];
        }
      }

      if (addr && (contributions.contributedAmountsWithoutBoost as any)[addr]) {
        contribution = (contributions.contributedAmountsWithoutBoost as any)[
          addr
        ];
      }
    }
    appDispatch(
      setUserNetVestedPICA({
        netVestedPICA,
      })
    );
    appDispatch(
      setUserContribution({
        contribution,
      })
    );
  }, [
    selectedAccount,
    kusamaAccounts.length,
    crUiState.useAssociationMode,
    account,
  ]);

  useEffect(() => {
    if (crowdloanRewards) {
      const setClaimedZero = () => {
        appDispatch(
          setUserClaimedPICA({
            claimedPICA: "0",
          })
        );
      };

      let addrToQuery = "";
      let isRelayChain = crUiState.useAssociationMode === "relayChain";

      if (isRelayChain) {
        if (selectedAccount !== -1 && kusamaAccounts.length) {
          addrToQuery = kusamaAccounts[selectedAccount].address;
        }
      } else {
        if (account) {
          addrToQuery = account;
        }
      }

      if (!addrToQuery.length) {
        setClaimedZero();
      } else {
        crowdloanRewards
          .queryRewards(addrToQuery, isRelayChain)
          .then((rewards) => {
            if (rewards) {
              let { claimed } = rewards;
              claimed = claimed.replaceAll(",", "");
              appDispatch(
                setUserClaimedPICA({
                  claimedPICA: claimed,
                })
              );
            } else {
              // set zero
              setClaimedZero();
            }
          });
      }
    }
  }, [
    selectedAccount,
    kusamaAccounts.length,
    crUiState.useAssociationMode,
    account,
    crowdloanRewards,
  ]);

  return null;
};

export default CrowdloanRewardsUpdater;
