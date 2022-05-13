import Image from "next/image";
import { AccountIndicator } from "../../Molecules/AccountIndicator";
import { ConnectButton } from "./ConnectButton";
import { Modal } from "../../Molecules/Modal";
import { closeMetamaskModal, openMetamaskModal } from "@/stores/ui/uiSlice";
import { connectMetamaskWallet } from "@/stores/defi/metamask";
import { useAppDispatch, useAppSelector } from "@/hooks/store";
import { useTheme, Box, Button, Typography } from "@mui/material";
import { useBlockchainProvider, useConnector } from "@integrations-lib/core";

import { Input } from "@/components";
import { FC } from "react";
const DEFAULT_EVM_ID = 1;

const Status = () => {
  const dispatch = useAppDispatch();
  const { isActive } = useConnector("metamask");
  const { account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const address = account
    ? account.slice(0, 6) + "..." + account.slice(-4)
    : "-";

  const theme = useTheme();
  if (isActive) {
    return (
      <Box
        sx={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          gap: theme.spacing(1),
        }}
      >
        <AccountIndicator
          onClick={() => {
            dispatch(openMetamaskModal());
          }}
          network="metamask"
          label={address}
        />
      </Box>
    );
  }

  return (
    <ConnectButton
      onClick={() => {
        dispatch(openMetamaskModal());
      }}
      imageSrc="/networks/mainnet_not_connected.svg"
      imageAlt="Ethereum Mainnet"
    >
      Connect EVM
    </ConnectButton>
  );
};

export const MetamaskConnect: FC<{}> = () => {
  const { isActive, activate, deactivate } = useConnector("metamask");
  const isModalOpen = useAppSelector((state) => state.ui.isMetamaskModalOpen);
  const { account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const address = account
    ? account.slice(0, 6) + "..." + account.slice(-4)
    : "-";
  const dispatch = useAppDispatch();

  const handleConnectMetamask = async () => {
    if (!isActive) {
      await activate();
      dispatch(connectMetamaskWallet());
      dispatch(closeMetamaskModal());
    }
  };

  const handleDisconnectMetamask = async () => {
    // handle disconnect
    if (deactivate) deactivate();
  };

  return (
    <>
      <Status />
      <Modal
        onClose={() => dispatch(closeMetamaskModal())}
        open={isModalOpen}
        maxWidth="sm"
        dismissible
      >
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          <Typography variant="h5" gutterBottom>
            {isActive ? "Account details" : "Connect wallet"}
          </Typography>
          {isActive ? (
            <Box width="100%">
              <Input
                value={address}
                disabled
                fullWidth
                sx={{
                  mt: 8,
                }}
                InputProps={{
                  inputProps: {
                    sx: {
                      textAlign: "center",
                    },
                  },
                }}
              />
              <Button
                fullWidth
                variant="text"
                size="large"
                onClick={() => handleDisconnectMetamask()}
                sx={{ mt: 4 }}
              >
                Disconnect wallet
              </Button>
            </Box>
          ) : (
            <>
              <Typography variant="body1" color="text.secondary" gutterBottom>
                Select a wallet to connect with.
              </Typography>
              <Button
                sx={{
                  mt: 8,
                  display: "flex",
                  gap: 2,
                  alignItems: "center",
                }}
                variant="outlined"
                color="primary"
                size="large"
                fullWidth
                onClick={() => handleConnectMetamask()}
              >
                <Image
                  src="/networks/metamask_wallet.svg"
                  width="24"
                  height="24"
                  alt="Metamask Mainnet"
                />
                <Typography variant="button">Metamask</Typography>
              </Button>
            </>
          )}
        </Box>
      </Modal>
    </>
  );
};
