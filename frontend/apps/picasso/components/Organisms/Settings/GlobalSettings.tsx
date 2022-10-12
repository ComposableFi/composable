import { SettingsModal } from "@/components/Organisms/Settings/SettingsModal";
import * as React from "react";
import { useState } from "react";
import { Settings } from "@mui/icons-material";
import { Button } from "@mui/material";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";

export const GlobalSettings = () => {
  const [settingsModal, setSettingsModal] = useState<boolean>(false);
  const account = useSelectedAccount();
  const picassoProvider = usePicassoProvider();

  const toggleSettingsModal = () => {
    setSettingsModal(s => !s);
  };
  if (picassoProvider.apiStatus !== "connected" && !account) {
    return null;
  }

  return (
    <>
      <Button variant="outlined" onClick={toggleSettingsModal}>
        <Settings />
      </Button>
      <SettingsModal state={settingsModal} onClose={toggleSettingsModal} />
    </>
  );
};
