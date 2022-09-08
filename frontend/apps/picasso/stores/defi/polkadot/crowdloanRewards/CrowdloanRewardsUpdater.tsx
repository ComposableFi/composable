import { PalletsContext } from "@/defi/polkadot/context/PalletsContext";
import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";
import { useKusamaAccounts, usePicassoProvider } from "@/defi/polkadot/hooks";
import { useContext, useEffect } from "react";

import { useStore } from "@/stores/root";
import rewards from "@/defi/polkadot/constants/pica-rewards.json";
import contributions from "@/defi/polkadot/constants/contributions.json";
import devRewards from "@/defi/polkadot/constants/pica-rewards-dev.json";
import { useBlockchainProvider } from "bi-lib";

const DEFAULT_EVM_ID = 1;

const CrowdloanRewardsUpdater = () => {
  const { account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const kusamaAccounts = useKusamaAccounts();
  const { selectedAccount } = useContext(ParachainContext);
  const picassoProvider = usePicassoProvider();
  const {
    ui,
    setEvmAlreadyAssociated,
    setInitialPayment,
    setUserAssociatedWith,
    setUserClaimEligibility,
    setUserClaimablePICA,
    setUserNetVestedPICA,
    setUserContribution,
    setUserClaimedPICA,
  } = useStore(({ crowdloanRewards }) => crowdloanRewards);
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

        setEvmAlreadyAssociated(evmAlreadyAssociated);
      });
    }
  }, [
    account,
    kusamaAccounts.length,
    crowdloanRewards,
    selectedAccount,
    kusamaAccounts,
    setEvmAlreadyAssociated,
  ]);

  useEffect(() => {
    if (crowdloanRewards) {
      crowdloanRewards.queryInitialPayment().then((ip) => {
        console.log("initial payment", ip);
        setInitialPayment(ip);
      });
    }
  }, [crowdloanRewards, setInitialPayment]);

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
            setUserAssociatedWith(null);
            setUserClaimEligibility(true);
          } else {
            setUserAssociatedWith(
              !!association.Ethereum ? "ethereum" : "relayChain"
            );
            setUserClaimEligibility(true);
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
    picassoProvider,
    setUserAssociatedWith,
    setUserClaimEligibility,
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
          setUserClaimablePICA(availableClaim);
        });
    }
  }, [
    selectedAccount,
    picassoProvider.apiStatus,
    picassoProvider.accounts.length,
    crowdloanRewards,
    picassoProvider,
    setUserClaimablePICA,
  ]);

  useEffect(() => {
    let netVestedPICA = "0";
    let contribution = "0";
    if (account && ui.useAssociationMode === "ethereum") {
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
      ui.useAssociationMode === "relayChain"
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

    setUserNetVestedPICA(netVestedPICA);

    setUserContribution(contribution);
  }, [
    selectedAccount,
    kusamaAccounts.length,
    ui.useAssociationMode,
    account,
    kusamaAccounts,
    setUserNetVestedPICA,
    setUserContribution,
  ]);

  useEffect(() => {
    if (crowdloanRewards) {
      const setClaimedZero = () => {
        setUserClaimedPICA("0");
      };

      let addrToQuery = "";
      let isRelayChain = ui.useAssociationMode === "relayChain";

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

              setUserClaimedPICA(claimed);
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
    ui.useAssociationMode,
    account,
    crowdloanRewards,
    setUserClaimedPICA,
    kusamaAccounts,
  ]);

  return null;
};

export default CrowdloanRewardsUpdater;
