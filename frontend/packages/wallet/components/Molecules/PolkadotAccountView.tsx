import { ContentCopy, OpenInNew, RepeatRounded } from "@mui/icons-material";
import {
  alpha,
  Box,
  Button,
  Grid,
  IconButton,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import Image from "next/image";
import Identicon from "@polkadot/react-identicon";
import { TabPanel } from "../Atoms/TabPanel";
import { Badge } from "../Atoms/Badge";
import { WalletViewTabs } from "../WalletViewModal";
import { SupportedWalletId } from "substrate-react";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";

export type PolkadotAccountViewProps = {
  activePanel: WalletViewTabs;
  selectedPolkadotWallet: {
    name: string;
    icon: string;
    walletId: SupportedWalletId;
  };
  selectedPolkadotAccount: InjectedAccountWithMeta;
  onChangeAccount: () => void;
  onDisconnectWallet: (() => void) | undefined;
  subscanUrl: string;
  nativeCurrencyIcon?: string;
};

export const PolkadotAccountView = ({
  activePanel,
  selectedPolkadotAccount,
  selectedPolkadotWallet,
  onChangeAccount,
  onDisconnectWallet,
  nativeCurrencyIcon,
  subscanUrl,
}: PolkadotAccountViewProps) => {
  const theme = useTheme();
  return (
    <TabPanel
      value={activePanel}
      index={WalletViewTabs.Wallets}
      sx={{
        padding: theme.spacing(4),
      }}
    >
      <Stack direction="column" gap={2}>
        <Grid container>
          <Grid item xs={8}>
            <Typography variant="body2" component="span">
              Connected with
            </Typography>
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
            justifyContent="flex-end"
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
                <RepeatRounded
                  sx={{
                    fontSize: "1rem",
                  }}
                />
              </IconButton>
            </Typography>
          </Grid>
        </Grid>

        <Grid container>
          <Grid
            item
            display="flex"
            xs={1}
            alignItems="flex-start"
            justifyContent="center"
            pt={1}
          >
            <Identicon
              value={selectedPolkadotAccount.address}
              size={32}
              theme={"polkadot"}
            />
          </Grid>
          <Grid item xs={11}>
            <Box marginLeft={theme.spacing(1)}>
              <Box
                display="flex"
                alignItems="center"
                justifyContent="space-between"
              >
                <Typography variant="body2" fontWeight="bold">
                  {selectedPolkadotAccount.meta.name}
                </Typography>
                <Box>
                  <IconButton
                    onClick={async (_evt) => {
                      await navigator.clipboard.writeText(
                        selectedPolkadotAccount.address
                      );
                    }}
                    color="primary"
                    size="small"
                  >
                    <ContentCopy
                      sx={{
                        fontSize: "1rem",
                      }}
                    />
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
                    <OpenInNew
                      sx={{
                        fontSize: "1rem",
                      }}
                    />
                  </IconButton>
                </Box>
              </Box>
              <Box
                sx={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "flex-start",
                }}
              >
                {nativeCurrencyIcon ? (
                  <Image src={nativeCurrencyIcon} height="16px" width="16px" />
                ) : null}
                <Typography
                  ml={1}
                  color={theme.palette.text.secondary}
                  variant="caption"
                  sx={{
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                  }}
                >
                  {selectedPolkadotAccount.address}
                </Typography>
              </Box>
            </Box>
          </Grid>
        </Grid>

        <Box>
          <Button
            variant="text"
            onClick={onDisconnectWallet}
            sx={{
              cursor: "pointer",
              color: theme.palette.text.secondary,
              "&:hover": {
                color: theme.palette.common.white,
                background: "transparent",
              },
              padding: 0,
              background: "transparent",
            }}
            disableRipple
          >
            Disconnect
          </Button>
        </Box>
      </Stack>
    </TabPanel>
  );
};
