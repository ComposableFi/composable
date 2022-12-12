import { DotSamaExtensionStatus, SupportedWalletId } from "substrate-react";
import { Box, IconButton, Typography, useTheme } from "@mui/material";
import { Dispatch, useCallback } from "react";
import { ConnectorType } from "bi-lib";
import { ChevronLeft } from "@mui/icons-material";
import { Modal } from "./Molecules/Modal";
import { NetworkId, WalletConnectStep } from "../types";
import { ConnectionListItem } from "./Molecules/ConnectionListItem";
import { PolkadotAccountsSelection } from "./Molecules/PolkadotAccountsSelection";
import { EthereumAccountStatus } from "./Molecules/EthereumAccountStatus";
import { WalletProps } from "./Wallet";
import { useSnackbar } from "notistack";

export type ConnectionModalProps = {
  closeConnectionModal: () => void;
  walletConnectStep: WalletConnectStep;
  setWalletConnectStep: Dispatch<WalletConnectStep>;
  isOpenConnectionModal: boolean;
} & Omit<
  WalletProps,
  "connectedAccountNativeBalance" | "connectedWalletTransactions"
>;

function getTitle(walletStep: WalletConnectStep): string {
  switch (walletStep) {
    case WalletConnectStep.SelectNetwork:
      return "Wallets";
    case WalletConnectStep.SelectedDotsamaWallet:
      return "Connect Dotsama";
    case WalletConnectStep.SelectEthereumWallet:
      return "Connect EVM";
    case WalletConnectStep.SelectDotsamaAccount:
      return "Select Account";
  }
}

function getDescription(walletStep: WalletConnectStep): string {
  switch (walletStep) {
    case WalletConnectStep.SelectNetwork:
      return "Select a network to continue";
    case WalletConnectStep.SelectedDotsamaWallet:
    case WalletConnectStep.SelectEthereumWallet:
    case WalletConnectStep.SelectDotsamaAccount:
      return "Select a wallet to connect with";
  }
}

function takeOneStepBack(
  walletStep: WalletConnectStep,
  dotsamaExtensionStatus: DotSamaExtensionStatus
): WalletConnectStep {
  switch (walletStep) {
    case WalletConnectStep.SelectNetwork:
      return WalletConnectStep.SelectNetwork;
    case WalletConnectStep.SelectDotsamaAccount:
      return dotsamaExtensionStatus === "connected"
        ? WalletConnectStep.SelectNetwork
        : WalletConnectStep.SelectedDotsamaWallet;
    case WalletConnectStep.SelectedDotsamaWallet:
    case WalletConnectStep.SelectEthereumWallet:
      return WalletConnectStep.SelectNetwork;
  }
}

export const ConnectionModal: React.FC<ConnectionModalProps> = ({
  closeConnectionModal,
  onConnectPolkadotWallet,
  onConnectEthereumWallet,
  onSelectPolkadotAccount,
  onDisconnectEthereum,
  onDisconnectDotsamaWallet,
  ethereumConnectedAccount,
  isEthereumWalletActive,
  polkadotExtensionStatus,
  supportedPolkadotWallets,
  supportedEthereumWallets,
  selectedPolkadotAccount,
  polkadotAccounts,
  walletConnectStep,
  setWalletConnectStep,
  blockchainNetworksSupported,
  isOpenConnectionModal,
}) => {
  const theme = useTheme();
  const { enqueueSnackbar } = useSnackbar();

  const networksList = useCallback(() => {
    return blockchainNetworksSupported.map((network) => (
      <ConnectionListItem
        key={network.networkId}
        id={network.networkId}
        name={network.name}
        icon={network.icon}
        onClick={(networkId: NetworkId) => {
          networkId === NetworkId.Ethereum
            ? setWalletConnectStep(WalletConnectStep.SelectEthereumWallet)
            : polkadotExtensionStatus === "connected"
            ? setWalletConnectStep(WalletConnectStep.SelectDotsamaAccount)
            : setWalletConnectStep(WalletConnectStep.SelectedDotsamaWallet);
        }}
      />
    ));
  }, [blockchainNetworksSupported, polkadotExtensionStatus]);

  const polkadotWalletsList = useCallback(() => {
    return supportedPolkadotWallets.map((wallet) => (
      <ConnectionListItem
        key={wallet.walletId}
        onClick={(walletId: SupportedWalletId) => {
          onConnectPolkadotWallet(walletId)
            .then(() => {
              setWalletConnectStep(WalletConnectStep.SelectDotsamaAccount);
            })
            .catch((err) => {
              enqueueSnackbar(err.message, { variant: "error" });
            });
        }}
        name={wallet.name}
        icon={wallet.icon}
        id={wallet.walletId}
      />
    ));
  }, [supportedPolkadotWallets, onConnectPolkadotWallet]);

  const ethereumWalletsList = useCallback(() => {
    return supportedEthereumWallets.map((wallet) => (
      <ConnectionListItem
        key={wallet.walletId}
        onClick={(walletId: ConnectorType) => {
          onConnectEthereumWallet?.(walletId).catch((err) => {
            enqueueSnackbar(err.message, { variant: "error" });
          });
        }}
        name={wallet.name}
        icon={wallet.icon}
        id={wallet.walletId}
      />
    ));
  }, [supportedEthereumWallets, onConnectEthereumWallet]);

  const title = getTitle(walletConnectStep);
  const description = getDescription(walletConnectStep);

  return (
    <Modal
      onClose={() => closeConnectionModal()}
      open={isOpenConnectionModal}
      maxWidth="sm"
      dismissible
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
        <Box
          sx={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          {walletConnectStep !== WalletConnectStep.SelectNetwork && (
            <IconButton
              sx={{ marginRight: "1rem" }}
              color="primary"
              onClick={() => {
                setWalletConnectStep(
                  takeOneStepBack(walletConnectStep, polkadotExtensionStatus)
                );
              }}
            >
              <ChevronLeft />
            </IconButton>
          )}

          <Typography variant="h5">{title}</Typography>
        </Box>

        <Typography
          mt={theme.spacing(2)}
          variant="body1"
          color="text.secondary"
          gutterBottom
        >
          {description}
        </Typography>

        {/* Step 1: Choose Network */}
        {walletConnectStep === WalletConnectStep.SelectNetwork
          ? networksList()
          : null}

        {/* Ethereum Steps */}
        {/* When Ethereum connection is needed */}
        {walletConnectStep === WalletConnectStep.SelectEthereumWallet &&
        !isEthereumWalletActive
          ? ethereumWalletsList()
          : null}
        {/* When account is available */}
        {walletConnectStep === WalletConnectStep.SelectEthereumWallet &&
        ethereumConnectedAccount ? (
          <EthereumAccountStatus
            connectedAddress={ethereumConnectedAccount}
            handleEthereumDisconnect={() => {
              if (onDisconnectEthereum) {
                onDisconnectEthereum(ConnectorType.MetaMask);
              }
            }}
          />
        ) : null}

        {/* Polkadot Steps */}
        {/* We connection is needed */}
        {polkadotExtensionStatus !== "connected" &&
        walletConnectStep === WalletConnectStep.SelectedDotsamaWallet
          ? polkadotWalletsList()
          : null}
        {/* We wallet selection is needed */}
        {polkadotExtensionStatus === "connected" &&
        (walletConnectStep === WalletConnectStep.SelectedDotsamaWallet ||
          walletConnectStep === WalletConnectStep.SelectDotsamaAccount) ? (
          <PolkadotAccountsSelection
            disconnectWallet={async () => {
              if (onDisconnectDotsamaWallet) {
                onDisconnectDotsamaWallet()
                  .then((_x) => {
                    setWalletConnectStep(
                      WalletConnectStep.SelectedDotsamaWallet
                    );
                  })
                  .catch((err) => {
                    enqueueSnackbar(err.message, { variant: "error" });
                  });
              }
            }}
            accounts={polkadotAccounts}
            selectedAccount={selectedPolkadotAccount}
            onSelect={onSelectPolkadotAccount}
            closeConnectionModal={closeConnectionModal}
          />
        ) : null}
      </Box>
    </Modal>
  );
};
