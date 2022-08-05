import { formatToken } from "shared";
import { FeeDisplay } from "@/components";
import React from "react";
import { useStore } from "@/stores/root";

export const TransferFeeDisplay = () => {
  const { tokenId, fee } = useStore(({ transfers }) => transfers);
  return (
    <FeeDisplay
      label="Fee"
      feeText={formatToken(fee, tokenId)}
      TooltipProps={{
        title: "Fee tooltip title",
      }}
    />
  );
};
