import { TokenId } from "tokens";
import { SettingsModal } from "@/components/Organisms/Settings/SettingsModal";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { useRef, useState } from "react";

export const GlobalSettings = () => {
  const [settingsModal, setSettingsModal] = useState<boolean>(false);
  const account = useSelectedAccount();
  const picassoProvider = usePicassoProvider();
  const targetFeeItem = useRef<TokenId>("pica");

  const toggleSettingsModal = () => {
    setSettingsModal((s) => !s);
  };

  if (picassoProvider.apiStatus !== "connected" && !account) {
    return null;
  }

  return (
    <SettingsModal
      state={settingsModal}
      onClose={toggleSettingsModal}
      targetFeeItem={targetFeeItem.current}
    />
  );
};
