import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { useStore } from "@/stores/root";
import { CheckRounded } from "@mui/icons-material";
import { alpha, Box, Button, Typography, useTheme } from "@mui/material";
import Image from "next/image";
import { SupportedWalletId, useDotSamaContext } from "substrate-react";

export const PolkadotAccountForm: React.FC<{
  onSelectChange?: (accountIndex: number) => void;
}> = ({ onSelectChange }) => {
  const {
    selectedAccount,
    extensionStatus,
    setSelectedAccount,
    deactivate
  } = useDotSamaContext();
  const { accounts } = usePicassoProvider();

  const theme = useTheme();
  const { closePolkadotModal } = useStore(({ ui }) => ui);

  const handleConfirm = () => {
    // dispatch(setSelectedAccount(selected));
    closePolkadotModal();
  };

  const handleDisconnect = () => {
    if (deactivate) deactivate();
  };

  let currAccount = accounts.find((v, i) => i === selectedAccount);
  currAccount = !!currAccount ? currAccount : { name: "", address: "" };

  return (
    <Box
      sx={{
        marginTop: theme.spacing(5),
        display: "flex",
        flexDirection: "column",
        gap: 4,
        width: "100%"
      }}
    >
      <Box
        sx={{
          height: "60vh",
          overflowY: "scroll",
          width: "100%",
          display: "flex",
          flexDirection: "column",
          gap: 4,
          px: 3
        }}
      >
        {accounts.map((account, index) => (
          <Button
            key={index}
            variant="outlined"
            color="primary"
            size="large"
            fullWidth
            onClick={() => {
              // setSelected(account);
              if (setSelectedAccount) {
                setSelectedAccount(index);
              }
            }}
            sx={{
              backgroundColor:
                selectedAccount === index
                  ? alpha(theme.palette.primary.main, 0.1)
                  : "",
              display: "flex",
              alignItems: "center",
              gap: theme.spacing(2)
            }}
          >
            <Image
              src="/networks/polkadot_js_wallet.svg"
              width="24"
              height="24"
              alt="Polkadot.js"
            />
            <Typography variant="button">{account.name}</Typography>
            {selectedAccount !== -1 && selectedAccount === index && (
              <CheckRounded />
            )}
          </Button>
        ))}
      </Box>
      {extensionStatus !== "connected" && (
        <Button
          fullWidth
          variant="contained"
          color="primary"
          disabled={selectedAccount === -1}
          onClick={() => handleConfirm()}
        >
          Confirm account
        </Button>
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
