import { ConnectorType } from "bi-lib";
import { SupportedWalletId } from "substrate-react";

export enum WalletConnectStep {
    SelectNetwork,
    SelectEthereumWallet,
    SelectedDotsamaWallet,
    SelectDotsamaAccount,
}

export enum NetworkId {
    Polkadot,
    Ethereum
}

export type PolkadotWallet = {
    name: string;
    icon: string;
    walletId: SupportedWalletId;
}

export type EthereumWallet = {
    name: string;
    icon: string;
    walletId: ConnectorType;
}

export type BlockchainNetwork = {
    icon: string;
    name: string;
    networkId: NetworkId;
    explorerUrl: string;
    nativeCurrencyIcon: string;
}