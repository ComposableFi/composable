import { alpha, Box, Button, Typography, useTheme } from "@mui/material";
import { CheckRounded } from "@mui/icons-material";
import { useConnectedAccounts, useDotSamaContext } from "substrate-react";
import Image from "next/image";
import useStore from "@/store/useStore";

export const PolkadotAccountForm: React.FC<{}> = () => {
  const { closePolkadotModal } = useStore();
  const {
    deactivate,
    extensionStatus,
    selectedAccount,
    setSelectedAccount,
  } = useDotSamaContext();
  const theme = useTheme();

  const handleConfirm = () => {
    closePolkadotModal();
  };

  const handleDisconnect = () => {
    if (deactivate) deactivate();
  };

  const accounts = useConnectedAccounts("picasso");

  return (
    <Box
      sx={{
        height: "60vh",
        overflowY: "scroll",
        width: "100%",
        display: "flex",
        flexDirection: "column",
        gap: 4,
        px: 3,
      }}
    >
      {accounts.map((account, index) => (
        <Button
          key={account.address + index}
          variant="outlined"
          color="primary"
          size="large"
          fullWidth
          onClick={() => {
            if (setSelectedAccount) {
              setSelectedAccount(index);
            }
          }}
          sx={{
            backgroundColor:
              selectedAccount !== -1
                ? accounts[selectedAccount].address === account.address
                  ? alpha(theme.palette.primary.main, 0.1)
                  : ""
                : "",
            display: "flex",
            alignItems: "center",
            gap: theme.spacing(2),
          }}
        >
          <Image
            src="/networks/polkadot_js_wallet.svg"
            width="24"
            height="24"
            alt="Polkadot.js"
          />
          <Typography variant="button">{account.meta.name ?? account.address}</Typography>
          {selectedAccount !== -1 &&
            account.address === accounts[selectedAccount].address && <CheckRounded />}
        </Button>
      ))}

      {extensionStatus !== "connected" && (
        <>
          <Button
            fullWidth
            variant="contained"
            color="primary"
            disabled={selectedAccount === -1}
            onClick={() => handleConfirm()}
          >
            Confirm account
          </Button>
          <Box mt={4} display="flex" justifyContent="center">
            <Typography
              textAlign="center"
              variant="body2"
              color={alpha(
                theme.palette.common.white,
                theme.custom.opacity.darker
              )}
            >
              By connecting wallet, you agree to our &nbsp;
            </Typography>
            <Typography textAlign="center" variant="body2" color="primary">
              Terms of service.
            </Typography>
          </Box>
        </>
      )}
      {extensionStatus === "connected" && (
        <Button
          fullWidth
          variant="text"
          color="primary"
          onClick={() => handleDisconnect()}
        >
          Disconnect wallet
        </Button>
      )}
    </Box>
  );
};
