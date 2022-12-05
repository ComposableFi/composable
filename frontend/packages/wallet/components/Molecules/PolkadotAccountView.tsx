import { RepeatRounded, ContentCopy, OpenInNew } from "@mui/icons-material";
import {
  alpha,
  Box,
  Grid,
  IconButton,
  Typography,
  useTheme,
} from "@mui/material";
import React from "react";
import Image from "next/image";
import Identicon from "@polkadot/react-identicon";
import { TabPanel } from "../Atoms/TabPanel";
import { Badge } from "../Atoms/Badge";
import { trimAddress, WalletViewTabs } from "../WalletViewModal";
import { SupportedWalletId } from "substrate-react";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";

export type PolkadotAccountViewProps = {
    activePanel: WalletViewTabs;
    selectedPolkadotWallet: { name: string; icon: string; walletId: SupportedWalletId };
    selectedPolkadotAccount: InjectedAccountWithMeta;
    onChangeAccount: () => void;
    onDisconnectWallet: (() => void) | undefined;
    subscanUrl: string;
}

export const PolkadotAccountView = ({
    activePanel,
    selectedPolkadotAccount,
    selectedPolkadotWallet,
    onChangeAccount,
    onDisconnectWallet,
    subscanUrl
}: PolkadotAccountViewProps) => {
  const theme = useTheme();
  return (
    <TabPanel value={activePanel} index={WalletViewTabs.Wallets}>
      <Grid container xs={12}>
        <Grid item xs={8}>
          <Typography variant="inputLabel">Connected with</Typography>
          <Badge
            marginLeft={theme.spacing(1)}
            label={selectedPolkadotWallet.name}
            icon={
              <Image
                src={selectedPolkadotWallet.icon}
                height="16px"
                width="16px"
              />
            }
            color={theme.palette.text.primary}
            background={alpha(theme.palette.text.primary, 0.1)}
          />
        </Grid>
        <Grid
          item
          xs={4}
          display="flex"
          justifyContent="center"
          alignItems="center"
        >
          <Typography variant="caption">
            Change
            <IconButton
              color="primary"
              onClick={(_evt) => {
                onChangeAccount();
              }}
            >
              <RepeatRounded />
            </IconButton>
          </Typography>
        </Grid>
      </Grid>

      <Grid container>
        <Grid
          item
          display="flex"
          xs={1}
          alignItems="center"
          justifyContent="center"
        >
          <Identicon
            value={selectedPolkadotAccount.address}
            size={32}
            theme={"polkadot"}
          />
        </Grid>
        <Grid item xs={11}>
          <Box marginLeft={theme.spacing(1)}>
            <Box>
              <Typography variant="inputLabel">
                {selectedPolkadotAccount.meta.name}
              </Typography>
            </Box>
            <Box>
              <Typography
                color={theme.palette.text.secondary}
                variant="caption"
              >
                {trimAddress(selectedPolkadotAccount.address)}
              </Typography>
              <IconButton
                onClick={(_evt) => {
                  navigator.clipboard.writeText(
                    selectedPolkadotAccount.address
                  );
                }}
                color="primary"
                size="small"
              >
                <ContentCopy></ContentCopy>
              </IconButton>
              <IconButton
                onClick={(_evt) => {
                  window.open(
                    subscanUrl +
                      "address/" +
                      selectedPolkadotAccount.address
                  );
                }}
                color="primary"
                size="small"
              >
                <OpenInNew></OpenInNew>
              </IconButton>
            </Box>
          </Box>
        </Grid>
      </Grid>

      <Box marginTop={theme.spacing(2)}>
        <Typography
          onClick={onDisconnectWallet}
          color={theme.palette.text.secondary}
          variant="inputLabel"
        >
          Disconnect
        </Typography>
      </Box>
    </TabPanel>
  );
};
