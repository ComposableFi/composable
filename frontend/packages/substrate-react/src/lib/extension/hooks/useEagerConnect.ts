import { ChainId, SupportedWalletId } from "../../types";
import { useEffect, useState } from "react";
import { activate } from "../lib";
import { setSelectedAccount, useSubstrateReact } from "../store/extension.slice";
import { useSubstrateNetwork } from "./useSubstrateNetwork";

/**
 * Idea is to have substrate-react
 * connect polkadot JS wallet eagerly
 * once the user lands on consumer web
 * app
 */
export const useEagerConnect = (chainId: ChainId, appName: string): boolean => {
  const { extensionStatus, chainApi, selectedAccount, hasInitialized } = useSubstrateReact();
  const [hasTriedEagerConnect, setHasTriedEagerConnect] =
    useState<boolean>(false);

  useEffect(() => {
    if (
      !hasTriedEagerConnect &&
      hasInitialized &&
      extensionStatus === "initializing"
    ) {
      const usedWallet = localStorage.getItem("wallet-id");
      if (
        (usedWallet && usedWallet === SupportedWalletId.Talisman) ||
        usedWallet === SupportedWalletId.Polkadotjs
      ) {
        activate(appName, usedWallet);
      }
    }
  }, [hasTriedEagerConnect, extensionStatus, chainApi, chainId, hasInitialized, appName]);

  useEffect(() => {
    if (selectedAccount) {
      localStorage.setItem("selectedAccount", selectedAccount.address);
    }
  }, [selectedAccount]);

  const { connectedAccounts } = useSubstrateNetwork(chainId);
  useEffect(() => {
    if (
      connectedAccounts.length > 0 &&
      !hasTriedEagerConnect &&
      hasInitialized &&
      extensionStatus === "connected"
    ) {

      const storedAccount = localStorage.getItem("selectedAccount");
      const account = connectedAccounts.find(
        (account) => account.address === storedAccount
      );

      if (account) {
        setSelectedAccount(account);
      }
      setHasTriedEagerConnect(true);
    }
  }, [hasTriedEagerConnect, extensionStatus, connectedAccounts, hasInitialized]);

  return hasTriedEagerConnect;
};
