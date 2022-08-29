import { TextSwitch } from "@/components";
import React from "react";
import { useStore } from "@/stores/root";

export const TransferKeepAliveSwitch = () => {
  const { keepAlive, flipKeepAlive } = useStore(({ transfers }) => transfers);

  return (
    <TextSwitch
      label="Keep alive"
      checked={keepAlive}
      TooltipProps={{
        title: "This will prevent account of being removed due to low balance.",
      }}
      onChange={flipKeepAlive}
    />
  );
};
