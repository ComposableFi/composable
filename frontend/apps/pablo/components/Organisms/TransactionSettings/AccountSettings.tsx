import { Modal } from "@/components/Molecules";
import { setUiState, useUiSlice } from "@/store/ui/ui.slice";
import {
  CloseOutlined,
  ContentCopy,
  OpenInNewRounded,
} from "@mui/icons-material";
import { alpha, Box, Button, Typography, useTheme } from "@mui/material";
import Image from "next/image";

const AccountSettings: React.FC<{}> = () => {
  const theme = useTheme();
  const { isAccountSettingsModalOpen } = useUiSlice();
  const isModalOpen = isAccountSettingsModalOpen;

  return (
    <Modal
      onClose={() => setUiState({ isAccountSettingsModalOpen: false })}
      open={isModalOpen}
      maxWidth="sm"
      BackdropProps={{
        sx: {
          backgroundImage: theme.palette.gradient.backdrop,
        },
      }}
    >
      <Box
        sx={{
          width: 550,
          padding: theme.spacing(4),
          [theme.breakpoints.down("sm")]: {
            width: "100%",
            padding: theme.spacing(2),
          },
          background: theme.palette.gradient.secondary,
          boxShadow: `-1px -1px ${alpha(
            theme.palette.common.white,
            theme.custom.opacity.light
          )}`,
          borderRadius: 1,
        }}
      >
        <Box display="flex" alignItems="center" justifyContent="space-between">
          <Typography>Account</Typography>
          <CloseOutlined
            sx={{ cursor: "pointer" }}
            onClick={() => setUiState({ isAccountSettingsModalOpen: false })}
          />
        </Box>
        <Box
          sx={{
            width: "100%",
            display: "flex",
            justifyContent: "center",
            flexDirection: "column",
            alignItems: "center",
          }}
        >
          <Image
            src={"/networks/polkadot_js_wallet.svg"}
            width="50"
            height="50"
            alt="account"
          />
          <Typography mt={3} variant="body2">
            {"John Doe"}
          </Typography>
        </Box>
        <Box display="flex" justifyContent="space-between" mt={3}>
          <Box display="flex" gap={1}>
            <ContentCopy sx={{ color: theme.palette.primary.main }} />
            <Typography variant="body2">{"Copy Address"}</Typography>
          </Box>
          <Box display="flex" gap={1}>
            <OpenInNewRounded sx={{ color: theme.palette.primary.main }} />
            <Typography variant="body2">{"View on the snowtrace"}</Typography>
          </Box>
        </Box>
        <Box mt={4}>
          <Button
            onClick={() => setUiState({ isPolkadotModalOpen: true })}
            variant="contained"
            fullWidth
          >
            Change Account
          </Button>
        </Box>
        <Box mt={3}>
          <Button
            variant="contained"
            sx={{ backgroundColor: alpha(theme.palette.common.white, 0.05) }}
            fullWidth
          >
            <Image
              src="/networks/polkadot_js.svg"
              width="24"
              height="24"
              alt="Polkadot.js"
            />
            &nbsp;&nbsp;{`Connected with Polkadot{.js}`}
          </Button>
        </Box>
        <Box mt={3}>
          <Typography mb={1} variant="body2">
            Transaction history
          </Typography>
          <Box
            sx={{
              background: theme.palette.background.transparentCharcoal,
              borderRadius: 1,
              padding: 4,
              border: `1px solid ${alpha(theme.palette.common.white, 0.1)}`,
            }}
            textAlign="center"
          >
            <Image
              src="/static/lemonade.png"
              css={{ mixBlendMode: "luminosity" }}
              width="96"
              height="96"
              alt="lemonade"
            />
            <Typography variant="body2" paddingTop={2} color="text.secondary">
              No transaction history
            </Typography>
          </Box>
        </Box>
      </Box>
    </Modal>
  );
};

export default AccountSettings;
