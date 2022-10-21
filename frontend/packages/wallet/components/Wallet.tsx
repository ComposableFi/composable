import React from "react";
import { useState } from "react";
import { ConnectionStatus } from "./ConnectionStatus";
import { ConnectionModal } from "./ConnectionModal";
import { WalletViewModal } from "./WalletViewModal";
import { ConnectorType } from "bi-lib";
import { DotSamaExtensionStatus, SupportedWalletId } from "substrate-react";
import { NetworkId, WalletConnectStep } from "../types";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import BigNumber from "bignumber.js";

export type WalletProps = {
  onConnectEthereumWallet: (walletId: ConnectorType) => Promise<any>;
  onConnectPolkadotWallet: (
    walletId?: SupportedWalletId,
    selectedDefaultAccount?: boolean
  ) => Promise<any[] | undefined>;
  onDisconnectEthereum: (...args: unknown[]) => Promise<void> | void;
  onDisconnectDotsamaWallet: (() => Promise<void>) | undefined;
  onSelectPolkadotAccount: (account: InjectedAccountWithMeta) => void;
  blockchainNetworksSupported: Array<{
    icon: string;
    name: string;
    networkId: NetworkId;
  }>;
  ethereumConnectedAccount?: string;
  isEthereumWalletActive: boolean;
  polkadotAccounts: Array<InjectedAccountWithMeta>;
  polkadotExtensionStatus: DotSamaExtensionStatus;
  polkadotSelectedAccount: InjectedAccountWithMeta | undefined;
  supportedEthereumWallets: Array<{
    walletId: ConnectorType;
    icon: string;
    name: string;
  }>;
  supportedPolkadotWallets: Array<{
    walletId: SupportedWalletId;
    icon: string;
    name: string;
  }>;
  connectedAccountNativeBalance: BigNumber;
};

export const Wallet: React.FC<WalletProps> = ({
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
  polkadotSelectedAccount,
  supportedPolkadotWallets,
  supportedEthereumWallets,
  connectedAccountNativeBalance,
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

  return (
    <>
      <ConnectionStatus
        setSelectedAsset={() => {
          console.log("Hello");
        }}
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
        polkadotSelectedAccount={polkadotSelectedAccount}
        onDisconnectEthereum={onDisconnectEthereum}
        onSelectPolkadotAccount={onSelectPolkadotAccount}
      />

      <WalletViewModal
        onDisconnectDotsamaWallet={onDisconnectDotsamaWallet}
        onDisconnectEthereum={onDisconnectEthereum}
        ethereumExplorerUrl="https://etherscan.io/"
        polkadotExplorerUrl="https://picasso.subscan.io/"
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
        ethConnectedAccount={ethereumConnectedAccount}
        polkadotSelectedAccount={polkadotSelectedAccount}
        open={isOpenWalletViewModal}
        onClose={(evt, reason) => {
          console.log(reason);
          setIsOpenWalletViewModal(false);
        }}
        nativeCurrencyIconUrl="/tokens/pica_bg_white.svg"
      />
    </>
  );
};
