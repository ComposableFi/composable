import { useEffect, useState } from "react";
import { useParachainApi, useSelectedAccount } from ".";
import { ParachainId } from "../types";
import { useDotSamaContext } from "./useDotSamaContext";

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
    if (parachainApi !== undefined && activate !== undefined && !hasTriedEagerConnect && extensionStatus === "initializing") {
      activate(false);
    }
  }, [activate, parachainApi, hasTriedEagerConnect, extensionStatus]);

  useEffect(() => {
    if (selectedAccount) {
      localStorage.setItem("selectedAccount", selectedAccount.address);
    }
  }, [selectedAccount]);

  useEffect(() => {
    if (
      accounts.length > 0 &&
      !hasTriedEagerConnect &&
      parachainApi !== undefined &&
      setSelectedAccount && 
      extensionStatus === 'connected'
    ) {
      const storedAccount = localStorage.getItem("selectedAccount");
      const accountIndex = accounts.findIndex(
        (account) => account.address === storedAccount
      );

      if (accountIndex !== -1) {
        setSelectedAccount(accountIndex);
      }
      setHasTriedEagerConnect(true);
    }
  }, [hasTriedEagerConnect, parachainApi, accounts, setSelectedAccount, extensionStatus]);

  return hasTriedEagerConnect;
};
