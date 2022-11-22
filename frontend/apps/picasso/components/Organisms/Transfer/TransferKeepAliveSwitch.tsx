import { TextSwitch } from "@/components";
import React from "react";
import { useStore } from "@/stores/root";

export const TransferKeepAliveSwitch = () => {
  const { keepAlive, flipKeepAlive } = useStore(({ transfers }) => transfers);

  return (
    <TextSwitch
      disabled={true}
      label="Keep alive"
      checked={keepAlive}
      TooltipProps={{
        title:
          "Check for enough balance in the account to keep the existential deposit.",
      }}
      onChange={flipKeepAlive}
    />
  );
};
