import { useEffect, useState } from "react";
import { ParachainId, SupportedWalletId } from "../types";
import { useDotSamaContext } from "./useDotSamaContext";
import { useParachainApi } from "./useParachainApi";
import { useSelectedAccount } from "./useSelectedAccount";

/**
 * Idea is to have substrate-react
 * connect polkadot JS wallet eagerly
 * once the user lands on consumer web
 * app
 */
export const useEagerConnect = (chainId: ParachainId): boolean => {
  const { activate, setSelectedAccount, extensionStatus } = useDotSamaContext();
  const { parachainApi, accounts } = useParachainApi(chainId);
  const [hasTriedEagerConnect, setHasTriedEagerConnect] =
    useState<boolean>(false);
  const selectedAccount = useSelectedAccount(chainId);

  useEffect(() => {
    if (
      parachainApi !== undefined &&
      activate !== undefined &&
      !hasTriedEagerConnect &&
      extensionStatus === "initializing"
    ) {
      const usedWallet = localStorage.getItem("wallet-id");
      if (
        (usedWallet && usedWallet === SupportedWalletId.Talisman) ||
        usedWallet === SupportedWalletId.Polkadotjs
      ) {
        activate(usedWallet, false);
      }
    }
  }, [activate, parachainApi, hasTriedEagerConnect, extensionStatus]);

  useEffect(() => {
    if (selectedAccount) {
      localStorage.setItem(
        `selectedAccount-${chainId}`,
        selectedAccount.address
      );
    }
  }, [selectedAccount]);

  useEffect(() => {
    if (
      accounts.length > 0 &&
      !hasTriedEagerConnect &&
      parachainApi !== undefined &&
      setSelectedAccount &&
      extensionStatus === "connected"
    ) {
      const storedAccount = localStorage.getItem(`selectedAccount-${chainId}`);
      const accountIndex = accounts.findIndex(
        (account) => account.address === storedAccount
      );

      if (accountIndex !== -1) {
        setSelectedAccount(accountIndex);
      }
      setHasTriedEagerConnect(true);
    }
  }, [
    hasTriedEagerConnect,
    parachainApi,
    accounts,
    setSelectedAccount,
    extensionStatus,
  ]);

  return hasTriedEagerConnect;
};
