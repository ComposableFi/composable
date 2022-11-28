import { Lock } from "@mui/icons-material";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import {
  alpha,
  Box,
  Button,
  Dialog,
  DialogProps,
  Grid,
  Tab,
  Tabs,
  Typography,
  useMediaQuery,
  useTheme,
} from "@mui/material";
import Image from "next/image";
import BigNumber from "bignumber.js";
import { Badge } from "./Atoms/Badge";
import { FC, useState } from "react";
import { PolkadotAccountView } from "./Molecules/PolkadotAccountView";
import { BlockchainNetwork, EthereumWallet, PolkadotWallet } from "../types";
import { EthereumAccountView } from "./Molecules/EthereumAccountView";
import { TransactionsPanel } from "./Molecules/TransactionsPanel";

export function trimAddress(address: string): string {
  return (
    address.substring(0, 13) +
    "..." +
    address.substring(address.length - 13, address.length)
  );
}

export enum WalletViewTabs {
  Wallets,
  Transactions,
}

export type ModalProps = DialogProps & {
  dismissible?: boolean;
  nativeIcon: string;
};

export type WalletViewProps = {
  balance: BigNumber;

  polkadotNetwork?: BlockchainNetwork;
  ethereumNetwork?: BlockchainNetwork;
  connectedEthereumAccount?: string;
  selectedPolkadotAccount?: InjectedAccountWithMeta;
  selectedPolkadotWallet?: PolkadotWallet;
  selectedEthereumWallet?: EthereumWallet;
  connectedWalletTransactions: Array<{ title: string; timestamp: number }>;

  onDisconnectEthereum: (...args: unknown[]) => Promise<void> | void;
  onDisconnectDotsamaWallet: (() => Promise<void>) | undefined;
  onChangePolkadotAccount: () => void;
  onConnectPolkadot: () => void;
  onConnectEVM: () => void;
} & DialogProps;

export const WalletViewModal: FC<WalletViewProps> = ({
  balance,
  polkadotNetwork,
  ethereumNetwork,
  connectedEthereumAccount,
  selectedPolkadotAccount,
  selectedEthereumWallet,
  selectedPolkadotWallet,
  onChangePolkadotAccount,
  onDisconnectDotsamaWallet,
  onDisconnectEthereum,
  onConnectPolkadot,
  connectedWalletTransactions,
  open,
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
      <Grid container>
        <Grid item xs={12} textAlign={"center"}>
          <Box
            sx={{
              display: "flex",
              justifyContent: "center",
              marginTop: theme.spacing(2),
            }}
          >
            {polkadotNetwork && (
              <Image
                src={polkadotNetwork.nativeCurrencyIcon}
                width="24"
                height="24"
                alt={"icon"}
              />
            )}
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
          {selectedPolkadotAccount && selectedPolkadotWallet && (
            <PolkadotAccountView
              selectedPolkadotAccount={selectedPolkadotAccount}
              selectedPolkadotWallet={selectedPolkadotWallet}
              nativeCurrencyIcon={polkadotNetwork?.nativeCurrencyIcon}
              onChangeAccount={onChangePolkadotAccount}
              onDisconnectWallet={onDisconnectDotsamaWallet}
              subscanUrl={
                polkadotNetwork?.explorerUrl ?? "http://picasso.subscan.io/"
              }
              activePanel={activePanel}
            />
          )}
          {connectedEthereumAccount && selectedEthereumWallet && (
            <EthereumAccountView
              connectedEthereumAccount={connectedEthereumAccount}
              selectedEthereumWallet={selectedEthereumWallet}
              onDisconnectWallet={onDisconnectEthereum}
              activePanel={activePanel}
              etherscanUrl={
                ethereumNetwork?.explorerUrl ?? "http://etherscan.io/"
              }
            />
          )}

          {!selectedPolkadotAccount && (
            <Box
              hidden={activePanel !== WalletViewTabs.Wallets}
              id={`tabpanel-0`}
              aria-labelledby={`tab-0`}
              sx={{
                marginTop: theme.spacing(2),
              }}
            >
              <Button
                onClick={onConnectPolkadot}
                fullWidth
                variant="outlined"
                sx={{
                  display: "flex",
                  justifyContent: "center",
                  gap: theme.spacing(1),
                }}
              >
                <Image
                  src="/networks/polkadot_js.svg"
                  height="23.5"
                  width="23.5"
                  alt="polkadot_wallet"
                />
                Connect DotSama
              </Button>
            </Box>
          )}

          {!connectedEthereumAccount && (
            <Box
              hidden={activePanel !== WalletViewTabs.Wallets}
              id={`tabpanel-0`}
              aria-labelledby={`tab-0`}
              sx={{
                marginTop: theme.spacing(2),
              }}
            >
              <Button
                onClick={onConnectEVM}
                fullWidth
                variant="outlined"
                sx={{
                  display: "flex",
                  justifyContent: "center",
                  gap: theme.spacing(1),
                }}
              >
                <Image
                  src="/networks/mainnet.svg"
                  height="23.5"
                  width="23.5"
                  alt="ethereum_wallet"
                />
                Connect EVM
              </Button>
            </Box>
          )}

          <TransactionsPanel
            activePanel={activePanel}
            transactions={connectedWalletTransactions}
          />
        </Grid>
      </Grid>
    </Dialog>
  );
};
