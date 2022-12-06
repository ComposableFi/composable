import { Add } from "@mui/icons-material";
import {
  Box,
  Paper,
  Typography,
  useMediaQuery,
  useTheme,
  alpha,
  Button,
} from "@mui/material";
import Image from "next/image";
import { useCallback } from "react";
import "../styles/theme.d.ts";

type ConnectionButtonProps = {
  label: string;
  onClick: () => void;
  isEthereumConnected?: boolean;
  isPolkadotConnected: boolean;
};

export const WalletIndicator: React.FC<ConnectionButtonProps> = ({
  label,
  onClick,
  isEthereumConnected = false,
  isPolkadotConnected,
}) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down("sm"));
  const polkaIcon = "/networks/polkadot_js.svg";
  const ethIcon = "/networks/mainnet.svg";

  const networkIcons = useCallback(() => {
    if (isEthereumConnected && isPolkadotConnected) {
      return (
        <>
          <Box sx={{ display: "flex" }}>
            <Image src={polkaIcon} width="24" height="24" alt="Account" />
          </Box>
          <Box sx={{ display: "flex", marginLeft: -1.5 }}>
            <Image src={ethIcon} width="24" height="24" alt="Account" />
          </Box>
        </>
      );
    } else if (isEthereumConnected || isPolkadotConnected) {
      const icon = isEthereumConnected ? ethIcon : polkaIcon;
      return (
        <Box sx={{ display: "flex" }}>
          <Image src={icon} width="24" height="24" alt="Account" />
        </Box>
      );
    } else {
      return (
        <>
          <Box sx={{ display: "flex" }}>
            <Image
              style={{ filter: "grayscale(100%)" }}
              src={polkaIcon}
              width="24"
              height="24"
              alt="Account"
            />
          </Box>
          <Box sx={{ display: "flex" }}>
            <Image
              style={{ filter: "grayscale(100%)" }}
              src={ethIcon}
              width="24"
              height="24"
              alt="Account"
            />
          </Box>
        </>
      );
    }
  }, [isEthereumConnected, isPolkadotConnected]);

  return (
    <Button
      variant="outlined"
      onClick={onClick}
      sx={{
        display: "flex",
        alignContent: "center",
        gap: theme.spacing(2),
        flexShrink: 0,
        cursor: "pointer",
        "&:hover": {
          background: alpha(
            theme.palette.primary.main,
            theme.custom.opacity.main
          ),
        },
      }}
    >
      <Box
        sx={{
          height: theme.spacing(3),
          display: "flex",
          flexGrow: isMobile ? 1 : undefined,
          justifyContent: isMobile ? "center" : undefined,
        }}
      >
        {networkIcons()}
      </Box>

      {!isMobile ? (
        <>
          <Typography variant="body2">{label}</Typography>
          {!isEthereumConnected && isPolkadotConnected ? <Add /> : null}
        </>
      ) : null}
    </Button>
  );
};
