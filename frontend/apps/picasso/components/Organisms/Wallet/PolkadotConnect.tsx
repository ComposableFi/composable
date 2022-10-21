import { DEFI_CONFIG } from "@/defi/polkadot/config";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { TokenId } from "tokens";
import { useStore } from "@/stores/root";
import { Box, useTheme } from "@mui/material";
import { useState } from "react";
import { Select } from "../../Atom";
import { AccountIndicator } from "../../Molecules/AccountIndicator";
import { humanBalance } from "shared";
import { useDotSamaContext, useEagerConnect, SupportedWalletId, useConnectedAccounts } from "substrate-react";
import { DEFAULT_EVM_ID, DEFAULT_NETWORK_ID } from "@/defi/polkadot/constants";
import { Wallet } from "wallet";
import { ConnectorType, useBlockchainProvider, useConnector } from "bi-lib";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { NetworkId } from "wallet";
import BigNumber from "bignumber.js";

const BLOCKCHAIN_NETWORKS_SUPPORTED = [
  { name: "Polkadot", icon: "/networks/polkadot_js.svg", networkId: NetworkId.Polkadot },
  { name: "Ethereum", icon: "/networks/mainnet.svg", networkId: NetworkId.Ethereum }
];

const POLKADOT_WALLETS_SUPPORTED: Array<{ walletId: SupportedWalletId, icon: string, name: string }> = [
  {
    walletId: SupportedWalletId.Polkadotjs,
    icon: "/networks/polkadot_js.svg",
    name: "Polkadot.js"
  },
  {
    walletId: SupportedWalletId.Talisman,
    icon: "/logo/talisman.svg",
    name: "Talisman"
  },
];

const ETHEREUM_WALLETS_SUPPORTED = [
  { name: "Metamask", icon: "/networks/metamask_wallet.svg", walletId: ConnectorType.MetaMask }
];

const Status = ({ label, isPolkadotActive, isEthereumActive }: { label: string; isPolkadotActive: boolean; isEthereumActive: boolean }) => {
  const theme = useTheme();

  const assets = useStore(({ substrateBalances }) => substrateBalances.assets);
  const { openPolkadotModal } = useStore(({ ui }) => ui);
  const [selectedAsset, setSelectedAsset] = useState<TokenId | undefined>(
    "pica"
  );

  return (
    <Box
      sx={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        gap: theme.spacing(1),
      }}
    >
      {isPolkadotActive && <Select
        value={selectedAsset}
        setValue={setSelectedAsset}
        options={DEFI_CONFIG.networkIds.map((networkId) => ({
          value: assets[networkId].native.meta.id,
          label: humanBalance(assets[networkId].native.balance),
          icon: assets[networkId].native.meta.icon,
        }))}
        sx={{
          "& .MuiOutlinedInput-root": {
            height: "56px",
            minWidth: "170px",
          },
        }}
      />}
      <AccountIndicator
        isEthereumConnected={isEthereumActive}
        onClick={() => {
          openPolkadotModal();
        }}
        isPolkadotConnected={isPolkadotActive}
        label={label}
      />
    </Box>
  );

};


export const PolkadotConnect: React.FC<{}> = () => {
  const theme = useTheme();
  const { deactivate, extensionStatus, activate, setSelectedAccount } = useDotSamaContext();
  const accounts = useConnectedAccounts(DEFAULT_NETWORK_ID);
  const { account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const connectedAccount = useSelectedAccount();
  const biLibConnector = useConnector(ConnectorType.MetaMask);
  useEagerConnect(DEFAULT_NETWORK_ID);

  return (
    <>
      <Wallet
        connectedAccountNativeBalance={new BigNumber(0)}
        onDisconnectDotsamaWallet={deactivate}
        onConnectPolkadotWallet={activate as any}
        blockchainNetworksSupported={BLOCKCHAIN_NETWORKS_SUPPORTED}
        supportedPolkadotWallets={POLKADOT_WALLETS_SUPPORTED}
        supportedEthereumWallets={ETHEREUM_WALLETS_SUPPORTED}
        polkadotAccounts={accounts}
        ethereumConnectedAccount={account}
        onConnectEthereumWallet={biLibConnector.activate as any}
        isEthereumWalletActive={biLibConnector.isActive ? biLibConnector.isActive : false}
        polkadotExtensionStatus={extensionStatus}
        polkadotSelectedAccount={connectedAccount}
        onDisconnectEthereum={biLibConnector.deactivate}
        onSelectPolkadotAccount={(account: InjectedAccountWithMeta) => {
          const index = accounts.findIndex(_account => account.address === _account.address);
          if (index >= 0 && setSelectedAccount) {
            setSelectedAccount(index)
          }
        }}
      />
    </>
  );
};
