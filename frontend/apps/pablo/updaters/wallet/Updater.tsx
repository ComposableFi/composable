import useStore from "@/store/useStore";
import {
  useDotSamaContext,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import _ from "lodash";
import { useEffect } from "react";

const Updater = () => {
  const { setSelectedAccount, activate } = useDotSamaContext();
  const { parachainApi, accounts } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { ui, setUiState } = useStore();

  const { hasTriedEagerConnect } = ui;

  useEffect(() => {
    if (selectedAccount) {
      console.log("setting selectedAddress: ", selectedAccount.address);
      localStorage.setItem("selectedAddress", selectedAccount.address);
    }
  }, [selectedAccount]);

  useEffect(() => {
    if (parachainApi !== undefined && activate !== undefined) {
      activate(false);
    }
  }, [parachainApi, activate]);

  useEffect(() => {
    if (
      !hasTriedEagerConnect &&
      setSelectedAccount !== undefined &&
      accounts.length > 0 &&
      parachainApi !== undefined
    ) {
      let lastStoredAddress = localStorage.getItem("selectedAddress");
      const account = accounts.findIndex(
        (account) => account.address === lastStoredAddress
      );

      if (account !== -1) {
        setSelectedAccount(account);
      }
      setUiState({ hasTriedEagerConnect: true });
    }
  }, [
    hasTriedEagerConnect,
    accounts.length,
    setSelectedAccount,
    accounts,
    setUiState,
    parachainApi,
  ]);

  return null;
};

export default Updater;
