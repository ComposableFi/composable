import { DEFI_CONFIG } from "@/defi/polkadot/config";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { TokenId } from "tokens";
import { useStore } from "@/stores/root";
import { Box, useTheme } from "@mui/material";
import { useState } from "react";
import { Select } from "../../Atom";
import { AccountIndicator } from "../../Molecules/AccountIndicator";
import { ConnectButton } from "./ConnectButton";
import { humanBalance } from "shared";
import { useDotSamaContext, useEagerConnect, SupportedWalletId, useParachainApi, ConnectedAccount } from "substrate-react";
import { DEFAULT_EVM_ID, DEFAULT_NETWORK_ID } from "@/defi/polkadot/constants";
import { ConnectWalletModal, NetworkId } from "polkadot-wallet";
import { ConnectorType, useBlockchainProvider, useConnector } from "bi-lib";

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

const Status = () => {
  const { extensionStatus, selectedAccount } = useDotSamaContext();
  const { accounts } = usePicassoProvider();
  const theme = useTheme();
  let label =
    accounts.length && selectedAccount !== -1
      ? accounts[selectedAccount].name
      : "";
  const assets = useStore(({ substrateBalances }) => substrateBalances.assets);
  const { openPolkadotModal } = useStore(({ ui }) => ui);
  const [selectedAsset, setSelectedAsset] = useState<TokenId | undefined>(
    "pica"
  );

  if (extensionStatus === "connected") {
    return (
      <Box
        sx={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          gap: theme.spacing(1),
        }}
      >
        <Select
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
        />
        <AccountIndicator
          onClick={() => {
            openPolkadotModal();
          }}
          network="polkadot"
          label={label}
        />
      </Box>
    );
  }

  return (
    <ConnectButton
      onClick={() => {
        openPolkadotModal();
      }}
      imageSrc="/networks/dotsama_polkadot_not_connected.svg"
      imageAlt="DotSama Polkadot"
    >
      Connect DotSama
    </ConnectButton>
  );
};


const MetamaskStatus = () => {
  const { openMetamaskModal } = useStore(({ ui }) => ui);
  const { isActive } = useConnector(ConnectorType.MetaMask);
  const { account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const address = account
    ? account.slice(0, 6) + "..." + account.slice(-4)
    : "-";

  const theme = useTheme();
  if (isActive) {
    return (
      <Box
        sx={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          gap: theme.spacing(1),
        }}
      >
        <AccountIndicator
          onClick={() => {
            openMetamaskModal();
          }}
          network="metamask"
          label={address}
        />
      </Box>
    );
  }

  return (
    <ConnectButton
      onClick={() => {
        openMetamaskModal();
      }}
      imageSrc="/networks/mainnet_not_connected.svg"
      imageAlt="Ethereum Mainnet"
    >
      Connect EVM
    </ConnectButton>
  );
};

export const PolkadotConnect: React.FC<{}> = () => {
  const { deactivate, extensionStatus, activate, setSelectedAccount } = useDotSamaContext();
  const { accounts } = useParachainApi(DEFAULT_NETWORK_ID);
  const connectedAccount = useSelectedAccount();
  const biLibConnector = useConnector(ConnectorType.MetaMask);
  const { account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const theme = useTheme();
  const hasTriedEagerConnect = useEagerConnect(DEFAULT_NETWORK_ID);
  const { closePolkadotModal, isPolkadotModalOpen } =
    useStore(({ ui }) => ui);

  return (
    <>
      <Status />
      {/* <MetamaskStatus /> */}
      <ConnectWalletModal
        onConnectPolkadotWallet={activate as any}
        networks={BLOCKCHAIN_NETWORKS_SUPPORTED}
        supportedPolkadotWallets={POLKADOT_WALLETS_SUPPORTED}
        supportedEthereumWallets={ETHEREUM_WALLETS_SUPPORTED}
        isOpen={isPolkadotModalOpen}
        closeWalletConnectModal={closePolkadotModal}
        polkadotAccounts={accounts}
        ethereumSelectedAccount={account}
        onConnectEthereumWallet={biLibConnector.activate as any}
        isEthereumWalletActive={biLibConnector.isActive ? biLibConnector.isActive : false}
        dotsamaExtensionStatus={extensionStatus}
        polkadotSelectedAccount={connectedAccount}
        onSelectPolkadotAccount={(account: ConnectedAccount) => {
          const index = accounts.findIndex(_account => account.address === _account.address);
          if (index >= 0 && setSelectedAccount) {
            setSelectedAccount(index)
          }
        }}
      />
    </>
  );
};
