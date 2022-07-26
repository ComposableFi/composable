import { DEFI_CONFIG } from "@/defi/polkadot/config";
import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { TokenId } from "tokens";
import { useStore } from "@/stores/root";
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
          options={DEFI_CONFIG.networkIds.map(networkId => ({
            value: substrateBalances[networkId].tokenId,
            label:
              Number(substrateBalances[networkId].balance) < 1000
                ? substrateBalances[networkId].balance
                : (Number(substrateBalances[networkId].balance) / 1000).toFixed(
                    1
                  ) + "K",
            icon: substrateBalances[networkId].icon
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
      imageSrc="/networks/dotsama_polkadot_not_connected.svg"
      imageAlt="DotSama Polkadot"
    >
      Connect DotSama
    </ConnectButton>
  );
};

export const PolkadotConnect: React.FC<{}> = () => {
  const {
    deactivate,
    extensionStatus,
    activate,
    setSelectedAccount
  } = useContext(ParachainContext);
  const theme = useTheme();

  const {
    closePolkadotModal,
    openPolkadotModal,
    setHasTriedEagerConnect,
    isPolkadotModalOpen,
    hasTriedEagerConnect
  } = useStore(({ ui }) => ui);

  const handleConnectPolkadot = async () => {
    if (activate) {
      await activate();
    }
  };

  useEffect(() => {
    if (!hasTriedEagerConnect) {
      setTimeout(() => {
        openPolkadotModal();
        setHasTriedEagerConnect();
        // wait 2P secs
      }, 1000);
    }
    // Only to be called on page load therefore we can omit dependencies.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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
                  justifyContent: "center"
                }}
              >
                <Typography variant="h5" gutterBottom>
                  Your accounts
                </Typography>
              </Box>
              <PolkadotAccountForm
                onSelectChange={account => {
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
