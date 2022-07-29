import React, { useCallback, useEffect, useMemo, useState } from "react";
import { BigNumberInput } from "@/components";
import { amountInputStyle } from "@/components/Organisms/Transfer/transfer-styles";
import { useStore } from "@/stores/root";
import { humanBalance } from "shared";
import { getTransferToken } from "@/components/Organisms/Transfer/utils";

export const AmountTokenDropdown = () => {
  const tokenId = useStore((state) => state.transfers.tokenId);
  const amount = useStore((state) => state.transfers.amount);
  const updateAmount = useStore((state) => state.transfers.updateAmount);
  const from = useStore((state) => state.transfers.networks.from);
  const to = useStore((state) => state.transfers.networks.to);

  const { native, assets } = useStore(
    ({ substrateBalances }) => substrateBalances[from]
  );

  const isNativeToNetwork = useMemo(() => {
    const transferableTokenId = getTransferToken(from, to);
    return assets[transferableTokenId].meta.supportedNetwork[from] === 1;
  }, [assets, from, to]);
  const balance = isNativeToNetwork ? native.balance : assets[tokenId].balance;
  const handleMaxClick = () => updateAmount(balance);

  return (
    <BigNumberInput
      buttonLabel="Max"
      value={amount}
      LabelProps={{
        mainLabelProps: {
          label: "Amount",
        },
        balanceLabelProps: {
          label: "Balance:",
          balanceText: humanBalance(balance) + " " + tokenId.toUpperCase(),
        },
      }}
      ButtonProps={{
        onClick: handleMaxClick,
      }}
      InputProps={{
        sx: amountInputStyle,
      }}
      maxValue={balance}
      setter={updateAmount}
    />
  );
};
