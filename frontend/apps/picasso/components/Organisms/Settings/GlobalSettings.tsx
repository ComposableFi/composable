import { TokenId } from "tokens";
import { SettingsModal } from "@/components/Organisms/Settings/SettingsModal";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { useRef, useState } from "react";
import { GasFeeDropdown } from "../GasFeeDropdown";

export const GlobalSettings = () => {
  const [settingsModal, setSettingsModal] = useState<boolean>(false);
  const account = useSelectedAccount();
  const picassoProvider = usePicassoProvider();
  const targetFeeItem = useRef<TokenId>("pica");

  const setTargetFeeItem = (feeItem: TokenId) => {
    targetFeeItem.current = feeItem;
  };

  const toggleSettingsModal = () => {
    setSettingsModal((s) => !s);
  };

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
