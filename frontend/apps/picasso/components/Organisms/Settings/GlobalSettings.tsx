import { TokenId } from "tokens";
import { SettingsModal } from "@/components/Organisms/Settings/SettingsModal";
import { usePicassoAccount } from "@/defi/polkadot/hooks";
import { useCallback, useRef, useState } from "react";
import { GasFeeDropdown } from "../GasFeeDropdown";
import { usePicassoProvider } from "substrate-react";

export const GlobalSettings = () => {
  const [settingsModal, setSettingsModal] = useState<boolean>(false);
  const account = usePicassoAccount();
  const picassoProvider = usePicassoProvider();
  const targetFeeItem = useRef<TokenId>("pica");

  const setTargetFeeItem = useCallback((feeItem: TokenId) => {
    targetFeeItem.current = feeItem;
  }, []);

  const toggleSettingsModal = useCallback(() => {
    setSettingsModal((s) => !s);
  }, []);

  if (picassoProvider.apiStatus !== "connected" && !account) {
    return null;
  }

  return (
    <>
      <GasFeeDropdown
        toggleModal={toggleSettingsModal}
        setTargetFeeItem={setTargetFeeItem}
      />
      <SettingsModal
        state={settingsModal}
        onClose={toggleSettingsModal}
        targetFeeItem={targetFeeItem.current}
      />
    </>
  );
};
