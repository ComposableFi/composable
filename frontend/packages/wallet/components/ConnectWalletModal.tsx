import { ConnectedAccount, DotSamaExtensionStatus, SupportedWalletId } from "substrate-react";
import { alpha, Box, Button, IconButton, Input, Typography, useTheme } from "@mui/material";
import { useState, useCallback } from "react";
import { ConnectorType } from "bi-lib";
import { ChevronLeft } from "@mui/icons-material";
import { Modal } from "./Modal";
import Identicon from '@polkadot/react-identicon';
import Image from "next/image";
import React from "react";

enum WalletConnectStep {
    SelectNetwork,
    SelectEthereumWallet,
    SelectedDotsamaWallet,
    SelectDotsamaAccount,
}

export enum NetworkId {
    Polkadot,
    Ethereum
}

const ConnectListItem = ({ icon, name, onClick, id }: { id: SupportedWalletId | ConnectorType | NetworkId, icon: string; name: string; onClick?: Function }) => {
    const theme = useTheme();
    return (<Button
        sx={{
            mt: "2rem",
            justifyContent: "flex-start"
        }}
        variant="outlined"
        color="primary"
        size="large"
        fullWidth
        onClick={() => {
            onClick?.(id);
        }}
    >
        <Box sx={{ marginLeft: theme.spacing(1.75), marginTop: theme.spacing(0.5) }}>
            <Image
                src={icon}
                width="24"
                height="24"
                alt={name}
            />
        </Box>
        <Box sx={{ justifyContent: "center", flexGrow: 1 }}>
            <Typography variant="button">{name}</Typography>
        </Box>
    </Button>)
}

const PolkadotAccount = ({ account, onSelect, isSelected, identiconTheme = "polkadot" }: {
    account: ConnectedAccount;
    onSelect: (account: ConnectedAccount) => void;
    isSelected: boolean;
    identiconTheme?: "substrate" | "polkadot" | "ethereum" | "jdenticon"
}) => {
    const theme = useTheme();
    return (
        <Button
            key={account.address}
            variant="outlined"
            color="primary"
            size="large"
            fullWidth
            onClick={() => {
                onSelect(account)
            }}
            sx={{
                height: "6.375rem",
                backgroundColor:
                    isSelected
                        ? alpha(theme.palette.primary.main, 0.1)
                        : "",
                display: "flex",
                justifyContent: "flex-start",
                alignItems: "center",
                gap: theme.spacing(2)
            }}
        >
            <Box sx={{ marginLeft: theme.spacing(1.75), marginTop: theme.spacing(0.5) }}>
                <Identicon
                    value={account.address}
                    size={24}
                    theme={identiconTheme}
                />
            </Box>
            <Box>
                <Typography textAlign={"left"}>{account.name}</Typography>
                <Typography sx={{ display: { xs: 'none', sm: 'block' } }} textAlign={"left"} variant="inputLabel" color="text.secondary">
                    {account.address}
                </Typography>
            </Box>
        </Button>
    )
}

const MetamaskAccountConnected = ({
    connectedAddress,
    handleEthereumDisconnect
}: { handleEthereumDisconnect: () => void; connectedAddress: string }) => {
    return (
        <Box width="100%">
              <Input
                value={connectedAddress.toLowerCase()}
                disabled
                fullWidth
                sx={{
                  mt: 8,
                }}
                inputProps={{
                  inputProps: {
                    sx: {
                      textAlign: "center",
                    },
                  },
                }}
              />
              <Button
                fullWidth
                variant="text"
                size="large"
                onClick={() => handleEthereumDisconnect()}
                sx={{ mt: 4 }}
              >
                Disconnect wallet
              </Button>
        </Box>
    )
}

const PolkadotAccounts = ({ accounts, onSelect, selectedAccount }: {
    accounts: ConnectedAccount[];
    onSelect: (account: ConnectedAccount) => void;
    selectedAccount?: ConnectedAccount;
}) => {
    const theme = useTheme();
    const [selectedActiveAccount, setSelectedActiveAccount] = useState<ConnectedAccount | undefined>(selectedAccount)
    return (
        <>
            <Box
                sx={{
                    marginTop: theme.spacing(2),
                    height: "40vh",
                    overflowY: "scroll",
                    width: "100%",
                    display: "flex",
                    flexDirection: "column",
                    gap: 4,

                }}
            >
                {accounts.map((account) => (
                    <PolkadotAccount
                        key={account.address}
                        account={account}
                        onSelect={setSelectedActiveAccount}
                        isSelected={selectedActiveAccount ? selectedActiveAccount.address === account.address : false}
                    />
                ))}

            </Box>
            <Box sx={{ width: "100%", marginTop: theme.spacing(2) }}>
                <Button onClick={() => {
                    if (selectedActiveAccount) {
                        onSelect(selectedActiveAccount)
                    }
                }} fullWidth variant="contained" disabled={selectedAccount && selectedActiveAccount && selectedAccount.address === selectedActiveAccount.address}>
                    Confirm Account
                </Button>
            </Box>
        </>
    )
}

type WalletConnectModalProps = {
    closeWalletConnectModal: () => void;
    onConnectPolkadotWallet: (walletId?: SupportedWalletId, selectedDefaultAccount?: boolean) => Promise<any[] | undefined>;
    onConnectEthereumWallet: (walletId: ConnectorType) => Promise<any>;
    onSelectPolkadotAccount: (account: ConnectedAccount) => void;
    onDisconnectEthereum: (...args: unknown[]) => Promise<void> | void;
    supportedPolkadotWallets: Array<{ walletId: SupportedWalletId, icon: string, name: string }>;
    supportedEthereumWallets: Array<{ walletId: ConnectorType, icon: string, name: string }>;
    networks: Array<{ icon: string, name: string; networkId: NetworkId }>;
    polkadotAccounts: Array<ConnectedAccount>;
    polkadotSelectedAccount: ConnectedAccount | undefined;
    ethereumSelectedAccount?: string;
    isOpen: boolean;
    dotsamaExtensionStatus: DotSamaExtensionStatus;
    isEthereumWalletActive: boolean;
}

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

function takeOneStepBack(walletStep: WalletConnectStep): WalletConnectStep {
    switch (walletStep) {
        case WalletConnectStep.SelectNetwork:
            return WalletConnectStep.SelectNetwork;
        case WalletConnectStep.SelectDotsamaAccount:
            return WalletConnectStep.SelectedDotsamaWallet;
        case WalletConnectStep.SelectedDotsamaWallet:
        case WalletConnectStep.SelectEthereumWallet:
            return WalletConnectStep.SelectNetwork;
    }
}

export const ConnectWalletModal: React.FC<WalletConnectModalProps> = ({
    closeWalletConnectModal,
    onConnectPolkadotWallet,
    onConnectEthereumWallet,
    onSelectPolkadotAccount,
    onDisconnectEthereum,
    ethereumSelectedAccount,
    isEthereumWalletActive,
    dotsamaExtensionStatus,
    supportedPolkadotWallets,
    supportedEthereumWallets,
    polkadotSelectedAccount,
    polkadotAccounts,
    networks,
    isOpen
}) => {
    const theme = useTheme();
    const [walletConnectStep, setWalletConnectStep] = useState(WalletConnectStep.SelectNetwork);

    const networksList = useCallback(() => {
        return networks.map(network => (
            <ConnectListItem
                key={network.networkId}
                id={network.networkId}
                name={network.name}
                icon={network.icon}
                onClick={(networkId: NetworkId) => {
                    networkId === NetworkId.Ethereum ? setWalletConnectStep(WalletConnectStep.SelectEthereumWallet) :
                        setWalletConnectStep(WalletConnectStep.SelectedDotsamaWallet)
                }}
            />
        ))
    }, [networks]);

    const polkadotWalletsList = useCallback(() => {
        return supportedPolkadotWallets.map(wallet => (
            <ConnectListItem
                key={wallet.walletId}
                onClick={(walletId: SupportedWalletId) => {
                    onConnectPolkadotWallet(walletId).then((walletConnected) => {
                        setWalletConnectStep(WalletConnectStep.SelectDotsamaAccount);
                    });
                }}
                name={wallet.name}
                icon={wallet.icon}
                id={wallet.walletId}
            />
        ))
    }, [supportedPolkadotWallets, onConnectPolkadotWallet]);

    const ethereumWalletsList = useCallback(() => {
        return supportedEthereumWallets.map(wallet => (
            <ConnectListItem
                key={wallet.walletId}
                onClick={(walletId: ConnectorType) => {
                    onConnectEthereumWallet(walletId);
                }}
                name={wallet.name}
                icon={wallet.icon}
                id={wallet.walletId}
            />
        ))
    }, [supportedEthereumWallets, onConnectEthereumWallet])

    const title = getTitle(walletConnectStep);
    const description = getDescription(walletConnectStep);

    return (
        <Modal
            onClose={() => closeWalletConnectModal()}
            open={isOpen}
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
                    {walletConnectStep !== WalletConnectStep.SelectNetwork &&
                        <IconButton sx={{ marginRight: "1rem" }} color="primary" onClick={() => {
                            setWalletConnectStep(takeOneStepBack(walletConnectStep))
                        }}>
                            <ChevronLeft />
                        </IconButton>}

                    <Typography variant="h5">{title}</Typography>
                </Box>

                <Typography mt={theme.spacing(2)} variant="body1" color="text.secondary" gutterBottom>
                    {description}
                </Typography>

                {/* Step 1: Choose Network */}
                {walletConnectStep === WalletConnectStep.SelectNetwork ? networksList() : null}

                {/* Ethereum Steps */}
                {/* We connection is needed */}
                {walletConnectStep === WalletConnectStep.SelectEthereumWallet && !isEthereumWalletActive ? ethereumWalletsList() : null}
                {/* We account is available, TODO: show ETH account and disconnection */}
                {walletConnectStep === WalletConnectStep.SelectEthereumWallet && ethereumSelectedAccount ? <MetamaskAccountConnected 
                    connectedAddress={ethereumSelectedAccount}
                    handleEthereumDisconnect={() => {
                        if (onDisconnectEthereum) {
                            onDisconnectEthereum(ConnectorType.MetaMask);
                        }
                    }}
                /> : null}

                {/* Polkadot Steps */}
                {/* We connection is needed */}
                {dotsamaExtensionStatus !== "connected" && walletConnectStep === WalletConnectStep.SelectedDotsamaWallet ? polkadotWalletsList() : null}
                {/* We wallet selection is needed */}
                {dotsamaExtensionStatus === "connected" && walletConnectStep === WalletConnectStep.SelectedDotsamaWallet ?
                    <PolkadotAccounts accounts={polkadotAccounts}
                        selectedAccount={polkadotSelectedAccount}
                        onSelect={onSelectPolkadotAccount} /> : null}

            </Box>
        </Modal>
    );
};
