import {
  Lock,
  RepeatRounded,
  ContentCopy,
  OpenInNew,
} from "@mui/icons-material";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import {
  alpha,
  Box,
  BoxProps,
  Button,
  Dialog,
  DialogProps,
  Grid,
  IconButton,
  Link,
  Tab,
  Tabs,
  Typography,
  useMediaQuery,
  useTheme,
} from "@mui/material";
import React from "react";
import { useState } from "react";
import Image from "next/image";
import BigNumber from "bignumber.js";
import { TabPanel } from "./Atoms/TabPanel";
import "../styles/theme.d.ts";
import { ConnectorType } from "bi-lib";
import { SupportedWalletId } from "substrate-react";
import Identicon from "@polkadot/react-identicon";

function trimAddress(address: string): string {
  return (
    address.substring(0, 13) +
    "..." +
    address.substring(address.length - 13, address.length)
  );
}

enum WalletViewTabs {
  Wallets,
  Transactions,
}

export type ModalProps = DialogProps & {
  dismissible?: boolean;
  nativeIcon: string;
};

export type BadgeProps = {
  color: string;
  background: string;
  icon: JSX.Element;
  label: string;
} & BoxProps;

const Badge = ({ color, background, icon, label, ...props }: BadgeProps) => {
  const theme = useTheme();
  return (
    <Box
      sx={{
        display: "inline-flex",
        justifyContent: "center",
        alignItems: "center",
        height: "2.124rem",
        color: color,
        background: background,
        borderRadius: "12px",
        px: 1,
      }}
      {...props}
    >
      {icon}
      <Typography variant="inputLabel" marginLeft={theme.spacing(1)}>
        {label}
      </Typography>
    </Box>
  );
};

export type WalletViewProps = {
  nativeCurrencyIconUrl: string;
  balance: BigNumber;
  ethConnectedAccount?: string;
  polkadotSelectedAccount?: InjectedAccountWithMeta;
  supportedEthereumWallets?: Array<{
    walletId: ConnectorType;
    icon: string;
    name: string;
  }>;
  supportedPolkadotWallets?: Array<{
    walletId: SupportedWalletId;
    icon: string;
    name: string;
  }>;
  selectedPolkadotWalletId?: SupportedWalletId;
  selectedEthereumWalletId?: ConnectorType;
  onDisconnectEthereum: (...args: unknown[]) => Promise<void> | void;
  onDisconnectDotsamaWallet: (() => Promise<void>) | undefined;
  ethereumExplorerUrl: string;
  polkadotExplorerUrl: string;
  onChangePolkadotAccount: () => void;
  onConnectPolkadot: () => void;
  onConnectEVM: () => void;
} & DialogProps;

export const WalletViewModal: React.FC<WalletViewProps> = ({
  open,
  balance,
  ethConnectedAccount,
  polkadotSelectedAccount,
  nativeCurrencyIconUrl,
  onChangePolkadotAccount,

  onDisconnectDotsamaWallet,
  onDisconnectEthereum,
  onConnectPolkadot,
  ethereumExplorerUrl,
  polkadotExplorerUrl,
  onConnectEVM,
  ...props
}) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down("sm"));
  const [activePanel, setActivePanel] = useState<WalletViewTabs>(
    WalletViewTabs.Wallets
  );

  return (
    <Dialog
      PaperProps={{
        style: {
          position: "absolute",
          top: "50px",
          bottom: 0,
          left: isMobile ? 0 : "calc(100% - 40rem)",
          right: 0,
          borderRadius: "12px",
          maxWidth: "34rem",
          maxHeight: "45.875rem",
          height: "fit-content",
        },
      }}
      open={open}
      {...props}
    >
      <Grid container xs={12}>
        <Grid item xs={12} textAlign={"center"}>
          <Box
            sx={{
              display: "flex",
              justifyContent: "center",
              marginTop: theme.spacing(2),
            }}
          >
            <Image
              src={nativeCurrencyIconUrl}
              width="24"
              height="24"
              alt={"icon"}
            />
          </Box>
          <Box
            sx={{
              display: "flex",
              justifyContent: "center",
              marginTop: theme.spacing(2),
            }}
          >
            <Typography
              sx={{
                fontSize: "2rem",
              }}
            >
              {balance.toFixed(2)}
            </Typography>
            <Typography
              sx={{
                fontSize: "2rem",
                color: theme.palette.text.secondary,
                marginLeft: theme.spacing(1),
              }}
            >
              PICA
            </Typography>
          </Box>
          <Box
            sx={{
              display: "flex",
              justifyContent: "center",
              marginTop: theme.spacing(1),
            }}
          >
            <Typography
              sx={{
                fontSize: "1rem",
                color: theme.palette.text.secondary,
              }}
            >
              Wallet Balance
            </Typography>
          </Box>
          <Box
            sx={{
              display: "flex",
              justifyContent: "center",
              marginTop: theme.spacing(2),
            }}
          >
            <Badge
              icon={<Lock />}
              background={alpha(theme.palette.warning.main, 0.1)}
              color={theme.palette.warning.main}
              label="Locked"
            />
          </Box>
        </Grid>
        <Grid item xs={12} marginTop={theme.spacing(1)}>
          <Box sx={{ borderBottom: 1, borderColor: "divider" }}>
            <Tabs
              variant="fullWidth"
              value={activePanel}
              onChange={(evt, newVal) => {
                setActivePanel(newVal);
              }}
              aria-label="basic tabs example"
            >
              <Tab label="Wallets" />
              <Tab label="Transactions" />
            </Tabs>
          </Box>
          {polkadotSelectedAccount && (
            <TabPanel value={activePanel} index={WalletViewTabs.Wallets}>
              <Grid container xs={12}>
                <Grid item xs={8}>
                  <Typography variant="inputLabel">Connected with</Typography>
                  <Badge
                    marginLeft={theme.spacing(1)}
                    label="Polkadot.js"
                    icon={
                      <Image
                        src={"networks/polkadot_js.svg"}
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
                        onChangePolkadotAccount();
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
                    value={polkadotSelectedAccount.address}
                    size={32}
                    theme={"polkadot"}
                  />
                </Grid>
                <Grid item xs={11}>
                  <Box marginLeft={theme.spacing(1)}>
                    <Box>
                      <Typography variant="inputLabel">
                        {polkadotSelectedAccount.meta.name}
                      </Typography>
                    </Box>
                    <Box>
                      <Typography
                        color={theme.palette.text.secondary}
                        variant="caption"
                      >
                        {trimAddress(polkadotSelectedAccount.address)}
                      </Typography>
                      <IconButton
                        onClick={(_evt) => {
                          navigator.clipboard.writeText(
                            polkadotSelectedAccount.address
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
                            polkadotExplorerUrl +
                              "address/" +
                              polkadotSelectedAccount.address
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
                  onClick={onDisconnectDotsamaWallet}
                  color={theme.palette.text.secondary}
                  variant="inputLabel"
                >
                  Disconnect
                </Typography>
              </Box>
            </TabPanel>
          )}
          {ethConnectedAccount && (
            <TabPanel value={activePanel} index={WalletViewTabs.Wallets}>
              <Box>
                <Typography variant="inputLabel">Connected with</Typography>
                <Badge
                  marginLeft={theme.spacing(1)}
                  label="Metamask"
                  icon={
                    <Image
                      src={"networks/metamask_wallet.svg"}
                      height="16px"
                      width="16px"
                    />
                  }
                  color={theme.palette.text.primary}
                  background={alpha(theme.palette.text.primary, 0.1)}
                />
              </Box>
              <Box
                marginTop={theme.spacing(2)}
                display="flex"
                alignItems={"center"}
              >
                <Image
                  height="32"
                  width="32"
                  src="networks/mainnet.svg"
                ></Image>
                <Typography marginLeft={theme.spacing(1)} variant="inputLabel">
                  {trimAddress(ethConnectedAccount)}
                </Typography>
                <IconButton
                  onClick={(_evt) => {
                    navigator.clipboard.writeText(ethConnectedAccount);
                  }}
                  color="primary"
                  size="small"
                >
                  <ContentCopy></ContentCopy>
                </IconButton>
                <IconButton
                  onClick={(_evt) => {
                    window.open(
                      ethereumExplorerUrl + "address/" + ethConnectedAccount
                    );
                  }}
                  color="primary"
                  size="small"
                >
                  <OpenInNew></OpenInNew>
                </IconButton>
              </Box>
              <Box marginTop={theme.spacing(2)}>
                <Typography
                  onClick={onDisconnectEthereum}
                  color={theme.palette.text.secondary}
                  variant="inputLabel"
                >
                  Disconnect
                </Typography>
              </Box>
            </TabPanel>
          )}
          {!polkadotSelectedAccount && (
            <Box
              hidden={activePanel !== WalletViewTabs.Wallets}
              id={`tabpanel-0`}
              aria-labelledby={`tab-0`}
              sx={{
                marginTop: theme.spacing(2),
              }}
            >
              <Button onClick={onConnectPolkadot} fullWidth variant="outlined">
                <Image
                  src="/networks/polkadot_js.svg"
                  height="23.5"
                  width="23.5"
                />
                Connect Polkadot
              </Button>
            </Box>
          )}
          {!ethConnectedAccount && (
            <Box
              hidden={activePanel !== WalletViewTabs.Wallets}
              id={`tabpanel-0`}
              aria-labelledby={`tab-0`}
              sx={{
                marginTop: theme.spacing(2),
              }}
            >
              <Button onClick={onConnectEVM} fullWidth variant="outlined">
                <Image src="/networks/mainnet.svg" height="23.5" width="23.5" />
                Connect EVM
              </Button>
            </Box>
          )}

            <TabPanel value={activePanel} index={WalletViewTabs.Transactions}>
              <Grid container>
                <Grid item xs={12} display="flex" justifyContent={"space-between"}>
                    <Typography variant="inputLabel">
                        Recent Transactions
                    </Typography>
                    <Typography variant="inputLabel">
                        <Link>Clear All</Link>
                    </Typography>
                </Grid>

                <Grid item xs={12} marginTop={theme.spacing(2)}>
                    <Box sx={{
                        display: "flex",
                        justifyContent: "space-between",
                        height: "172px",
                        px: 0,
                        overflowY: "scroll"
                    }}>
                        <Typography variant="caption">
                            Recent Transactions
                        </Typography>
                        <Typography variant="caption">
                            12/11/2024
                        </Typography>
                    </Box>
                </Grid>
              </Grid>
            </TabPanel>
        </Grid>
      </Grid>
    </Dialog>
  );
};
