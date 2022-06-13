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
import { useState } from "react";
import useStore from "@/store/useStore";
import { useDotSamaContext, useParachainApi } from "substrate-react";
import { AssetId } from "@/defi/polkadot/types";
import BigNumber from "bignumber.js";

const Status = () => {
  const theme = useTheme();
  const {assets,assetBalances} = useStore();

  const { openPolkadotModal } = useStore();
  const { extensionStatus, selectedAccount } = useDotSamaContext();
  const { accounts } = useParachainApi("picasso");
  const [selectedAsset, setSelectedAsset] = useState<AssetId>(
    "pica"
  );

  if (extensionStatus === 'connected') {
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
          options={Object.values(assets).map((asset) => {
            let balance = new BigNumber(assetBalances[asset.assetId].picasso);

            return {
              value: asset.assetId,
              label:
              balance.lte(1000)
                  ? balance.toString()
                  : (balance.div(1000)).toFixed(1) + "K",
              icon: asset.icon,
            }
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
            openPolkadotModal();
          }}
          network="polkadot"
          label={selectedAccount !== -1 && accounts.length ? accounts[selectedAccount].name : "Connect Wallet"}
        />
      </Box>
    );
  }

  return (
    <Button
      onClick={() => {
        openPolkadotModal();
      }}
      variant="contained"
    >
      Connect wallet
    </Button>
  );
};

export const PolkadotConnect: React.FC<{}> = () => {
  const theme = useTheme();

  const { ui: { isPolkadotModalOpen }, closePolkadotModal } = useStore();
  const { extensionStatus, activate } = useDotSamaContext();

  const handleConnectPolkadot = async () => {
    if (activate) await activate();
  };

  return (
    <>
      <Status />
      <Modal
        onClose={() => { closePolkadotModal() }}
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
              />
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
