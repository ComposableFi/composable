import { AccountIndicator } from "../../Molecules/AccountIndicator";
import { Modal } from "../../Molecules/Modal";
import { PolkadotAccountForm } from "./PolkadotAccountForm";
import { Select } from "../../Atoms/Select";
import { ChevronLeft } from "@mui/icons-material";
import {
  alpha,
  Box,
  Button,
  IconButton,
  Typography,
  useTheme,
} from "@mui/material";
import Image from "next/image";
import { useEffect, useState } from "react";
import { useDotSamaContext, SupportedWalletId, useEagerConnect, useConnectedAccounts } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useAssetsWithBalance } from "@/defi/hooks";
import { useSnackbar } from "notistack";
import { useUiSlice, setUiState } from "@/store/ui/ui.slice";

const WALLETS_SUPPORTED: Array<{
  walletId: SupportedWalletId;
  icon: string;
  name: string;
}> = [
  {
    walletId: SupportedWalletId.Polkadotjs,
    icon: "/networks/polkadot_js.svg",
    name: "Polkadot.js",
  },
  {
    walletId: SupportedWalletId.Talisman,
    icon: "/logos/talisman.svg",
    name: "Talisman",
  },
];

const Status = () => {
  const { extensionStatus, selectedAccount } = useDotSamaContext();
  const theme = useTheme();
  const assetsWithBalance = useAssetsWithBalance();

  useEagerConnect(DEFAULT_NETWORK_ID);
  const accounts = useConnectedAccounts(DEFAULT_NETWORK_ID);
  const [selectedAsset, setSelectedAsset] = useState<string>("");

  useEffect(() => {
    if (assetsWithBalance.length > 0) {
      setSelectedAsset(assetsWithBalance[0].getSymbol());
    }
  }, [assetsWithBalance]);

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
          options={assetsWithBalance.map((asset) => {
            return {
              value: asset.getSymbol(),
              label: asset.getBalance().lte(1000)
                ? asset.getBalance().toFixed(2)
                : asset.getBalance().div(1000).toFixed(2) + "K",
              icon: asset.getIconUrl(),
            };
          })}
          sx={{
            "& .MuiOutlinedInput-root": {
              height: "56px",
              minWidth: "170px",
            },
          }}
        />
        <AccountIndicator
          onClick={() => {
            setUiState({ isPolkadotModalOpen: true });
          }}
          network="polkadot"
          label={
            selectedAccount !== -1 && accounts[selectedAccount]
              ? (
                !!accounts[selectedAccount].meta.name ?
                accounts[selectedAccount].meta.name as string :
                accounts[selectedAccount].address
              )
              : "Select account"
          }
        />
      </Box>
    );
  }

  return (
    <Button
      onClick={() => {
        setUiState({ isPolkadotModalOpen: true });
      }}
      variant="contained"
    >
      Connect wallet
    </Button>
  );
};

export const PolkadotConnect: React.FC<{}> = () => {
  const theme = useTheme();
  const {
    isPolkadotModalOpen
  } = useUiSlice();
  const { extensionStatus, activate } = useDotSamaContext();
  const { enqueueSnackbar } = useSnackbar();

  const handleConnectPolkadot = async (walletId: SupportedWalletId) => {
    try {
      if (activate) await activate(walletId);
    } catch (err: any) {
      console.log("Logged ", err);
      enqueueSnackbar(err.message, { variant: "error", persist: true });
    }
  };

  return (
    <>
      <Status />
      <Modal
        onClose={() => {
          setUiState({ isPolkadotModalOpen: false });
        }}
        open={isPolkadotModalOpen}
        maxWidth="sm"
        dismissible
        BackdropProps={{
          sx: {
            backgroundImage: theme.palette.gradient.backdrop,
          },
        }}
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
          {extensionStatus !== "connected" && extensionStatus !== "connecting" && (
            <>
              <Typography variant="h5" gutterBottom>
                Connect wallet
              </Typography>
              <Typography
                variant="body1"
                color={alpha(
                  theme.palette.common.white,
                  theme.custom.opacity.darker
                )}
                gutterBottom
              >
                Select a wallet to connect with.
              </Typography>
              {WALLETS_SUPPORTED.map(({ name, walletId, icon }) => {
                return (
                  <Button
                    sx={{
                      mt: "4rem",
                    }}
                    variant="outlined"
                    color="primary"
                    size="large"
                    key={walletId}
                    fullWidth
                    onClick={() => handleConnectPolkadot(walletId)}
                  >
                    <Box sx={{ marginRight: theme.spacing(2) }}>
                      <Image src={icon} width="24" height="24" alt={walletId} />
                    </Box>
                    <Typography variant="button">{name}</Typography>
                  </Button>
                );
              })}
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
              <PolkadotAccountForm />
            </>
          )}
          {extensionStatus !== "connected" && extensionStatus === "connecting" && (
            <>
              <Box
                sx={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                }}
              >
                <IconButton color="primary" onClick={() => {}}>
                  <ChevronLeft />
                </IconButton>

                <Typography variant="h5">Select account</Typography>
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
