import { DEFI_CONFIG } from "@/defi/polkadot/config";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { TokenId } from "tokens";
import { useStore } from "@/stores/root";
import { ChevronLeft } from "@mui/icons-material";
import { Box, Button, IconButton, Typography, useTheme } from "@mui/material";
import { useState } from "react";
import { Select } from "../../Atom";
import { AccountIndicator } from "../../Molecules/AccountIndicator";
import { Modal } from "../../Molecules/Modal";
import { ConnectButton } from "./ConnectButton";
import { PolkadotAccountForm } from "./PolkadotAccountForm";
import { humanBalance } from "shared";
import { useDotSamaContext, useEagerConnect, SupportedWalletId } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/polkadot/constants";
import Image from "next/image";

const WALLETS_SUPPORTED: Array<{ walletId: SupportedWalletId, icon: string, name: string }> = [
  {
    walletId: SupportedWalletId.Polkadotjs,
    icon: "/networks/polkadot_js.svg",
    name: "Polkadot.js"
  },
  {
    walletId: SupportedWalletId.Talisman,
    icon: "/logo/talisman.svg",
    name: "Talisman"
  },
];

const Status = () => {
  const { extensionStatus, selectedAccount } = useDotSamaContext();
  const { accounts } = usePicassoProvider();
  const theme = useTheme();
  let label =
    accounts.length && selectedAccount !== -1
      ? accounts[selectedAccount].name
      : "";
  const assets = useStore(({ substrateBalances }) => substrateBalances.assets);
  const { openPolkadotModal } = useStore(({ ui }) => ui);
  const [selectedAsset, setSelectedAsset] = useState<TokenId | undefined>(
    "pica"
  );

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
          options={DEFI_CONFIG.networkIds.map((networkId) => ({
            value: assets[networkId].native.meta.id,
            label: humanBalance(assets[networkId].native.balance),
            icon: assets[networkId].native.meta.icon,
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
            openPolkadotModal();
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
        openPolkadotModal();
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
    useDotSamaContext();
  const theme = useTheme();
  const hasTriedEagerConnect = useEagerConnect(DEFAULT_NETWORK_ID);
  const { closePolkadotModal, openPolkadotModal, isPolkadotModalOpen } =
    useStore(({ ui }) => ui);

  const handleConnectPolkadot = async (walletId: SupportedWalletId) => {
    if (activate) {
      await activate(walletId);
    }
  };

  return (
    <>
      <Status />
      <Modal
        onClose={() => closePolkadotModal()}
        open={isPolkadotModalOpen}
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
              {WALLETS_SUPPORTED.map(wallet => (<Button
                sx={{
                  mt: "4rem",
                }}
                variant="outlined"
                color="primary"
                size="large"
                fullWidth
                onClick={() => handleConnectPolkadot(wallet.walletId)}
              >
                <Box sx={{ marginRight: theme.spacing(2) }}>
                  <Image
                    src={wallet.icon}
                    width="24"
                    height="24"
                    alt={wallet.name}
                  />
                </Box>
                <Typography variant="button">{wallet.name}</Typography>
              </Button>))}
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
