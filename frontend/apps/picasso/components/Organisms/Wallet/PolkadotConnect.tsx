import { usePicassoAccount } from "@/defi/polkadot/hooks";
import {
  SupportedWalletId,
  useConnectedAccounts,
  useDotSamaContext,
  useEagerConnect,
  useTransactions,
} from "substrate-react";
import { NetworkId, Wallet } from "wallet";
import { ConnectorType, useBlockchainProvider, useConnector } from "bi-lib";
import { useStore } from "@/stores/root";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import config from "@/constants/config";
import { FC } from "react";

const BLOCKCHAIN_NETWORKS_SUPPORTED = [
  {
    name: "DotSama",
    icon: "/networks/picasso.svg",
    networkId: NetworkId.Polkadot,
    explorerUrl: "https://picasso.subscan.io/",
    nativeCurrencyIcon: "/logos/picasso.svg",
  },
  {
    name: "Ethereum",
    icon: "/networks/mainnet.svg",
    networkId: NetworkId.Ethereum,
    explorerUrl: "https://etherscan.io/",
    nativeCurrencyIcon: "/logos/ethereum.svg",
  },
];

const POLKADOT_WALLETS_SUPPORTED: Array<{
  walletId: SupportedWalletId;
  icon: string;
  name: string;
}> = [
  {
    walletId: SupportedWalletId.Polkadotjs,
    icon: "/logos/polkadotjs.svg",
    name: "Polkadot.js",
  },
  {
    walletId: SupportedWalletId.Talisman,
    icon: "/logos/talisman.svg",
    name: "Talisman",
  },
];

const ETHEREUM_WALLETS_SUPPORTED = [
  {
    name: "Metamask",
    icon: "/logos/metamask.svg",
    walletId: ConnectorType.MetaMask,
  },
];

export const PolkadotConnect: FC = () => {
  const { deactivate, extensionStatus, activate, setSelectedAccount } =
    useDotSamaContext();
  const accounts = useConnectedAccounts(config.defaultNetworkId);
  const { account, connectorType } = useBlockchainProvider(
    config.evm.defaultNetworkId
  );
  const connectedAccount = usePicassoAccount();
  const biLibConnector = useConnector(ConnectorType.MetaMask);
  useEagerConnect(config.defaultNetworkId);

  const balance = useStore(
    ({ substrateBalances }) => substrateBalances.balances.picasso.pica
  );

  const transactions = useTransactions(connectedAccount?.address ?? "-");

  return (
    <Wallet
      connectedWalletTransactions={transactions.map((tx) => {
        return {
          title: `${tx.section} ${tx.method}`,
          timestamp: tx.timestamp,
        };
      })}
      ethereumConnectorInUse={connectorType}
      connectedAccountNativeBalance={balance}
      onDisconnectDotsamaWallet={deactivate}
      onConnectPolkadotWallet={activate as any}
      blockchainNetworksSupported={BLOCKCHAIN_NETWORKS_SUPPORTED}
      supportedPolkadotWallets={POLKADOT_WALLETS_SUPPORTED}
      supportedEthereumWallets={ETHEREUM_WALLETS_SUPPORTED}
      polkadotAccounts={accounts}
      ethereumConnectedAccount={account}
      onConnectEthereumWallet={biLibConnector.activate as any}
      isEthereumWalletActive={
        biLibConnector.isActive ? biLibConnector.isActive : false
      }
      polkadotExtensionStatus={extensionStatus}
      selectedPolkadotAccount={connectedAccount}
      onDisconnectEthereum={biLibConnector.deactivate}
      onSelectPolkadotAccount={(account: InjectedAccountWithMeta) => {
        const index = accounts.findIndex(
          (_account) => account.address === _account.address
        );
        if (index >= 0 && setSelectedAccount) {
          setSelectedAccount(index);
        }
      }}
    />
  );
};
