import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { TokenId } from "@/defi/Tokens";
import { useAppDispatch, useAppSelector } from "@/hooks/store";
import {
  closePolkadotModal,
  openPolkadotModal,
  setHasTriedEagerConnect,
} from "@/stores/ui/uiSlice";
import { ChevronLeft } from "@mui/icons-material";
import { Box, Button, IconButton, Typography, useTheme } from "@mui/material";
import Image from "next/image";
import { useContext, useEffect, useState } from "react";
import { Select } from "../../Atom";
import { AccountIndicator } from "../../Molecules/AccountIndicator";
import { Modal } from "../../Molecules/Modal";
import { ConnectButton } from "./ConnectButton";
import { PolkadotAccountForm } from "./PolkadotAccountForm";

const Status = () => {
  const { extensionStatus, selectedAccount } = useContext(ParachainContext);
  const { accounts } = usePicassoProvider();
  const theme = useTheme();
  const dispatch = useAppDispatch();
  let label =
    accounts.length && selectedAccount !== -1
      ? accounts[selectedAccount].name
      : "";

  const assets = useAppSelector((state) =>
    Object.values(state.substrateBalances)
  );
  const [selectedAsset, setSelectedAsset] =
    useState<TokenId | undefined>("pica");

  if (extensionStatus === "connected") {
    return (
      <Box
        sx={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          gap: theme.spacing(1),
        }}
      >
        <Select
          value={selectedAsset}
          setValue={setSelectedAsset}
          options={assets.map((asset) => ({
            value: asset.tokenId,
            label:
              Number(asset.balance) < 1000
                ? asset.balance
                : (Number(asset.balance) / 1000).toFixed(1) + "K",
            icon: asset.icon,
          }))}
          sx={{
            "& .MuiOutlinedInput-root": {
              height: "56px",
              minWidth: "170px",
            },
          }}
        />
        <AccountIndicator
          onClick={() => {
            dispatch(openPolkadotModal());
          }}
          network="polkadot"
          label={label}
        />
      </Box>
    );
  }

  return (
    <ConnectButton
      onClick={() => {
        dispatch(openPolkadotModal());
      }}
      imageSrc="/networks/dotsama_polkadot_not_connected.svg"
      imageAlt="DotSama Polkadot"
    >
      Connect DotSama
    </ConnectButton>
  );
};

export const PolkadotConnect: React.FC<{}> = () => {
  const { deactivate, extensionStatus, activate, setSelectedAccount } =
    useContext(ParachainContext);
  const theme = useTheme();
  const dispatch = useAppDispatch();
  const isModalOpen = useAppSelector((state) => state.ui.isPolkadotModalOpen);
  const hasTriedEagerConnect = useAppSelector(
    (state) => state.ui.hasTriedEagerConnect
  );

  const handleConnectPolkadot = async () => {
    if (activate) {
      await activate();
    }
  };

  useEffect(() => {
    if (!hasTriedEagerConnect) {
      setTimeout(() => {
        dispatch(openPolkadotModal());
        dispatch(setHasTriedEagerConnect());
        // wait 2P secs
      }, 1000);
    }
    // only to be called on page load
    // therefore no param effect
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <>
      <Status />
      <Modal
        onClose={() => dispatch(closePolkadotModal())}
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
            height: "100%",
          }}
        >
          {extensionStatus !== "connecting" && extensionStatus !== "connected" && (
            <>
              <Typography variant="h5" gutterBottom>
                Connect wallet
              </Typography>
              <Typography variant="body1" color="text.secondary" gutterBottom>
                Select a wallet to connect with.
              </Typography>
              <Button
                sx={{
                  mt: "4rem",
                }}
                variant="outlined"
                color="primary"
                size="large"
                fullWidth
                onClick={() => handleConnectPolkadot()}
              >
                <Box sx={{ marginRight: theme.spacing(2) }}>
                  <Image
                    src="/networks/polkadot_js.svg"
                    width="24"
                    height="24"
                    alt="Polkadot.js"
                  />
                </Box>
                <Typography variant="button">Polkadot.js</Typography>
              </Button>
            </>
          )}

          {extensionStatus === "connected" && (
            <>
              <Box
                sx={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                }}
              >
                <Typography variant="h5" gutterBottom>
                  Your accounts
                </Typography>
              </Box>
              <PolkadotAccountForm
                onSelectChange={(account) => {
                  if (setSelectedAccount) setSelectedAccount(account);
                  // dispatch(setSelectedAccount(account));
                }}
              />
            </>
          )}
          {extensionStatus === "connecting" && (
            <>
              <Box
                sx={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                }}
              >
                <IconButton
                  color="primary"
                  onClick={() => {
                    if (deactivate) deactivate();
                  }}
                >
                  <ChevronLeft />
                </IconButton>

                <Typography variant="h5" gutterBottom>
                  Select account
                </Typography>
              </Box>
              <Typography variant="body1" color="text.secondary">
                Choose an account to connect with
              </Typography>
              <PolkadotAccountForm />
            </>
          )}
        </Box>
      </Modal>
    </>
  );
};
