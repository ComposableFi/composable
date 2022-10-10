import { DEFI_CONFIG } from "@/defi/polkadot/config";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { TokenId } from "tokens";
import { useStore } from "@/stores/root";
import { ChevronLeft } from "@mui/icons-material";
import { Box, Button, IconButton, Typography, useTheme } from "@mui/material";
import Image from "next/image";
import { useState } from "react";
import { Select } from "../../Atom";
import { AccountIndicator } from "../../Molecules/AccountIndicator";
import { Modal } from "../../Molecules/Modal";
import { ConnectButton } from "./ConnectButton";
import { PolkadotAccountForm } from "./PolkadotAccountForm";
import { humanBalance } from "shared";
import { useDotSamaContext, useEagerConnect } from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/polkadot/constants";
import { getImageURL } from "@/utils/nextImageUrl";

const Status = () => {
  const { extensionStatus, selectedAccount } = useDotSamaContext();
  const { accounts } = usePicassoProvider();
  const theme = useTheme();
  let label =
    accounts.length && selectedAccount !== -1
      ? accounts[selectedAccount].name
      : "";
  const substrateBalances = useStore(
    ({ substrateBalances }) => substrateBalances
  );
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
          gap: theme.spacing(1)
        }}
      >
        <Select
          value={selectedAsset}
          setValue={setSelectedAsset}
          options={DEFI_CONFIG.networkIds.map((networkId) => ({
            value: substrateBalances[networkId].native.meta.id,
            label: humanBalance(substrateBalances[networkId].native.balance),
            icon: substrateBalances[networkId].native.meta.icon
          }))}
          sx={{
            "& .MuiOutlinedInput-root": {
              height: "56px",
              minWidth: "170px"
            }
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
      imageSrc={getImageURL("/networks/dotsama_polkadot_not_connected.svg")}
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
  const {
    closePolkadotModal,
    openPolkadotModal,
    isPolkadotModalOpen
  } = useStore(({ ui }) => ui);

  const handleConnectPolkadot = async () => {
    if (activate) {
      await activate();
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
            height: "100%"
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
                  mt: "4rem"
                }}
                variant="outlined"
                color="primary"
                size="large"
                fullWidth
                onClick={() => handleConnectPolkadot()}
              >
                <Box sx={{ marginRight: theme.spacing(2) }}>
                  <Image
                    src={getImageURL("/networks/polkadot_js.svg")}
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
                  justifyContent: "center"
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
                  justifyContent: "center"
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
