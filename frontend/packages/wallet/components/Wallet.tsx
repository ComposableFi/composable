import { FC, ReactNode, useEffect, useState } from "react";
import { ConnectionStatus } from "./ConnectionStatus";
import { ConnectionModal } from "./ConnectionModal";
import { WalletViewModal } from "./WalletViewModal";
import { ConnectorType } from "bi-lib";
import { DotSamaExtensionStatus, SupportedWalletId } from "substrate-react";
import {
  BlockchainNetwork,
  EthereumWallet,
  NetworkId,
  PolkadotWallet,
  WalletConnectStep,
} from "../types";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import BigNumber from "bignumber.js";

export type WalletProps = {
  onConnectEthereumWallet?: (walletId: ConnectorType) => Promise<any>;
  onConnectPolkadotWallet: (
    walletId?: SupportedWalletId,
    selectedDefaultAccount?: boolean
  ) => Promise<any[] | undefined>;
  onDisconnectEthereum?: (...args: unknown[]) => Promise<void> | void;
  onDisconnectDotsamaWallet: (() => Promise<void>) | undefined;
  onSelectPolkadotAccount: (account: InjectedAccountWithMeta) => void;
  blockchainNetworksSupported: Array<BlockchainNetwork>;
  ethereumConnectedAccount?: string;
  ethereumConnectorInUse?: ConnectorType;
  isEthereumWalletActive?: boolean;
  polkadotAccounts: Array<InjectedAccountWithMeta>;
  polkadotExtensionStatus: DotSamaExtensionStatus;
  connectedWalletTransactions: Array<{ title: string; timestamp: number }>;
  selectedPolkadotAccount: InjectedAccountWithMeta | undefined;
  supportedEthereumWallets: Array<EthereumWallet>;
  supportedPolkadotWallets: Array<PolkadotWallet>;
  connectedAccountNativeBalance: {
    free: BigNumber;
    locked: BigNumber;
  };
  hideEth?: boolean;
  Disclaimer?: ReactNode;
};

export const Wallet: FC<WalletProps> = ({
  onConnectEthereumWallet,
  onConnectPolkadotWallet,
  onDisconnectEthereum,
  onDisconnectDotsamaWallet,
  onSelectPolkadotAccount,
  blockchainNetworksSupported,
  ethereumConnectedAccount,
  isEthereumWalletActive,
  polkadotAccounts,
  polkadotExtensionStatus,
  selectedPolkadotAccount,
  supportedPolkadotWallets,
  supportedEthereumWallets,
  connectedAccountNativeBalance,
  connectedWalletTransactions,
  ethereumConnectorInUse,
  hideEth,
  Disclaimer,
}) => {
  const label =
    isEthereumWalletActive || polkadotExtensionStatus === "connected"
      ? "Connected"
      : "Wallets";
  const [isOpenConnectionModal, setIsOpenConnectionModal] = useState(false);
  const [isOpenWalletViewModal, setIsOpenWalletViewModal] = useState(false);
  const [walletConnectStep, setWalletConnectStep] = useState(
    WalletConnectStep.SelectNetwork
  );

  useEffect(() => {
    const openWalletModal = () => {
      if (polkadotExtensionStatus === "connected") {
        setIsOpenWalletViewModal(true);
      } else {
        setIsOpenConnectionModal(true);
      }
    };

    document.addEventListener("WalletConnect", openWalletModal, {
      passive: true,
    });

    return () => document.removeEventListener("WalletConnect", openWalletModal);
  }, []);

  const selectedPolkadotWallet = supportedPolkadotWallets.find((x) => {
    return x.walletId === localStorage.getItem("wallet-id");
  });

  const selectedEthereumWallet = supportedEthereumWallets.find((x) => {
    return x.walletId === ethereumConnectorInUse;
  });

  return (
    <>
      <ConnectionStatus
        setSelectedAsset={() => {}}
        selectedAsset={""}
        onOpenConnectionModal={() => {
          if (polkadotExtensionStatus === "connected") {
            setIsOpenWalletViewModal(true);
          } else {
            setIsOpenConnectionModal(true);
          }
        }}
        label={label}
        isEthereumActive={isEthereumWalletActive}
        isPolkadotActive={polkadotExtensionStatus === "connected"}
        ownedAssets={[]}
        hideEth={hideEth}
        blockchainNetworksSupported={blockchainNetworksSupported}
      />

      <ConnectionModal
        walletConnectStep={walletConnectStep}
        setWalletConnectStep={setWalletConnectStep}
        onDisconnectDotsamaWallet={onDisconnectDotsamaWallet}
        onConnectPolkadotWallet={onConnectPolkadotWallet}
        blockchainNetworksSupported={blockchainNetworksSupported}
        supportedPolkadotWallets={supportedPolkadotWallets}
        supportedEthereumWallets={supportedEthereumWallets}
        isOpenConnectionModal={isOpenConnectionModal}
        closeConnectionModal={() => {
          setIsOpenConnectionModal(false);
        }}
        polkadotAccounts={polkadotAccounts}
        ethereumConnectedAccount={ethereumConnectedAccount}
        onConnectEthereumWallet={onConnectEthereumWallet}
        isEthereumWalletActive={isEthereumWalletActive}
        polkadotExtensionStatus={polkadotExtensionStatus}
        selectedPolkadotAccount={selectedPolkadotAccount}
        onDisconnectEthereum={onDisconnectEthereum}
        onSelectPolkadotAccount={onSelectPolkadotAccount}
        Disclaimer={Disclaimer}
      />

      <WalletViewModal
        connectedWalletTransactions={connectedWalletTransactions}
        selectedEthereumWallet={selectedEthereumWallet}
        selectedPolkadotWallet={selectedPolkadotWallet}
        onDisconnectDotsamaWallet={onDisconnectDotsamaWallet}
        onDisconnectEthereum={onDisconnectEthereum}
        ethereumNetwork={blockchainNetworksSupported.find((x) => {
          return x.networkId === NetworkId.Ethereum;
        })}
        polkadotNetwork={blockchainNetworksSupported.find((x) => {
          return x.networkId === NetworkId.Polkadot;
        })}
        onConnectPolkadot={() => {
          setWalletConnectStep(WalletConnectStep.SelectedDotsamaWallet);
          setIsOpenWalletViewModal(false);
          setIsOpenConnectionModal(true);
        }}
        onChangePolkadotAccount={() => {
          setWalletConnectStep(WalletConnectStep.SelectDotsamaAccount);
          setIsOpenWalletViewModal(false);
          setIsOpenConnectionModal(true);
        }}
        onConnectEVM={() => {
          setWalletConnectStep(WalletConnectStep.SelectEthereumWallet);
          setIsOpenWalletViewModal(false);
          setIsOpenConnectionModal(true);
        }}
        balance={connectedAccountNativeBalance}
        connectedEthereumAccount={ethereumConnectedAccount}
        selectedPolkadotAccount={selectedPolkadotAccount}
        open={isOpenWalletViewModal}
        onClose={(_evt, _reason) => {
          setIsOpenWalletViewModal(false);
        }}
      />
    </>
  );
};
