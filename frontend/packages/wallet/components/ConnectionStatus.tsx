import React from "react";
import { Box, useTheme } from "@mui/material";
import { Select } from "./Atoms/Select";
import { WalletIndicator } from "./WalletIndicator";

export type ConnectionStatusProps = {
  label: string;
  isPolkadotActive: boolean;
  isEthereumActive?: boolean;
  onOpenConnectionModal: () => void;
  selectedAsset: string;
  setSelectedAsset: () => void;
  ownedAssets: Array<{
    value: string;
    icon: string;
    label: string;
  }>;
};

export const ConnectionStatus = ({
  label,
  isPolkadotActive,
  isEthereumActive,
  selectedAsset,
  ownedAssets,
  onOpenConnectionModal,
  setSelectedAsset,
}: ConnectionStatusProps) => {
  const theme = useTheme();

  return (
    <Box
      sx={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        gap: theme.spacing(1),
      }}
    >
      {isPolkadotActive && ownedAssets.length > 0 && (
        <Select
          value={selectedAsset}
          setValue={setSelectedAsset}
          options={ownedAssets.map((_ownedAsset) => ({
            value: _ownedAsset.value,
            label: _ownedAsset.label,
            icon: _ownedAsset.icon,
          }))}
          sx={{
            "& .MuiOutlinedInput-root": {
              height: "56px",
              minWidth: "170px",
            },
          }}
        />
      )}
      <WalletIndicator
        isEthereumConnected={isEthereumActive}
        onClick={() => {
          onOpenConnectionModal();
        }}
        isPolkadotConnected={isPolkadotActive}
        label={label}
      />
    </Box>
  );
};
