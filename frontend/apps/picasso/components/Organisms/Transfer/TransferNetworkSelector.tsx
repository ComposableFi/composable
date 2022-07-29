import { Box, Button } from "@mui/material";
import {
  networksStyle,
  swapButtonStyle,
} from "@/components/Organisms/Transfer/transfer-styles";
import { NetworkSelect } from "@/components";
import { SwapHoriz } from "@mui/icons-material";
import React, { useEffect, useMemo } from "react";
import { useStore } from "@/stores/root";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import {
  availableTargetNetwork,
  getTransferToken,
} from "@/components/Organisms/Transfer/utils";

export const TransferNetworkSelector = () => {
  const { networks, updateNetworks, updateTokenId } = useStore(
    ({ transfers }) => transfers
  );

  const handleUpdateFromValue = (value: SubstrateNetworkId) => {
    const targetNetwork = networks.options.find(
      ({ networkId }) =>
        networkId !== value && availableTargetNetwork(networkId, value)
    );

    updateNetworks({
      ...networks,
      from: value,
      to: targetNetwork!.networkId,
    });
  };

  const handleSwapClick = () =>
    updateNetworks({ from: networks.to, to: networks.from });

  const handleUpdateToValue = (value: SubstrateNetworkId) =>
    updateNetworks({ ...networks, to: value });

  const networkToOptions = useMemo(
    () =>
      networks.options.filter(({ networkId }) => {
        return (
          networkId !== networks.from &&
          availableTargetNetwork(networkId, networks.from)
        );
      }),
    [networks.from, networks.options]
  );

  useEffect(() => {
    const transferableTokenId = getTransferToken(networks.from, networks.to);
    updateTokenId(transferableTokenId);
  }, [updateTokenId, networks.from, networks.to]);

  return (
    <Box display="flex" sx={networksStyle}>
      <NetworkSelect
        LabelProps={{ mainLabelProps: { label: "From network" } }}
        options={networks.options}
        value={networks.from}
        searchable
        substrateNetwork
        setValue={handleUpdateFromValue}
      />
      <Button
        sx={swapButtonStyle}
        variant="outlined"
        size="large"
        onClick={handleSwapClick}
      >
        <SwapHoriz />
      </Button>
      <NetworkSelect
        LabelProps={{ mainLabelProps: { label: "To network" } }}
        options={networkToOptions}
        value={networks.to}
        searchable
        substrateNetwork
        setValue={handleUpdateToValue}
      />
    </Box>
  );
};
