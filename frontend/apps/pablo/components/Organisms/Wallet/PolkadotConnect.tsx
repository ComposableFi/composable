import {
  SupportedWalletId,
  useConnectedAccounts,
  useDotSamaContext,
  useEagerConnect,
  useSelectedAccount,
  useTransactions,
} from "substrate-react";
import { NetworkId, Wallet } from "wallet";
import { ConnectorType } from "bi-lib";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { FC } from "react";
import useStore from "@/store/useStore";

const BLOCKCHAIN_NETWORKS_SUPPORTED = [
  {
    name: "DotSama",
    icon: "/networks/polkadot_js.svg",
    networkId: NetworkId.Polkadot,
    explorerUrl: "https://picasso.subscan.io/",
    nativeCurrencyIcon: "/logos/picasso.svg",
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
  const accounts = useConnectedAccounts(DEFAULT_NETWORK_ID);
  const connectedAccount = useSelectedAccount("picasso");
  useEagerConnect(DEFAULT_NETWORK_ID);
  const transactions = useTransactions(connectedAccount?.address ?? "-");
  const picaBalance = useStore(
    (store) => store.substrateBalances.tokenBalances.picasso.pica.free
  );

  return (
    <Wallet
      connectedWalletTransactions={transactions.map((tx) => {
        return {
          title: `${tx.section} ${tx.method}`,
          timestamp: tx.timestamp,
        };
      })}
      connectedAccountNativeBalance={picaBalance}
      onDisconnectDotsamaWallet={deactivate}
      onConnectPolkadotWallet={activate as any}
      blockchainNetworksSupported={BLOCKCHAIN_NETWORKS_SUPPORTED}
      supportedPolkadotWallets={POLKADOT_WALLETS_SUPPORTED}
      supportedEthereumWallets={ETHEREUM_WALLETS_SUPPORTED}
      polkadotAccounts={accounts}
      polkadotExtensionStatus={extensionStatus}
      selectedPolkadotAccount={connectedAccount}
      onSelectPolkadotAccount={(account: InjectedAccountWithMeta) => {
        const index = accounts.findIndex(
          (_account) => account.address === _account.address
        );
        if (index >= 0 && setSelectedAccount) {
          setSelectedAccount(index);
        }
      }}
      isEthereumWalletActive={false} // TODO mark EVM related stuff as optional.
      onConnectEthereumWallet={() => Promise.resolve()}
      onDisconnectEthereum={() => Promise.resolve()}
    />
  );
};
