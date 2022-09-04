import { Web3ReactHooks } from "@web3-react/core";
import { MetaMask } from "@web3-react/metamask";
import { Connector as Web3ReactConnector } from "@web3-react/types";
import { useSelector } from "react-redux";

import {
  Connectors,
  HooksState,
} from "../workaround-web3-react-issues-379/HooksStore";

export enum ConnectorType {
  MetaMask = "metamask",
  Static = "static",
}

export type NonStaticConnectorType = Exclude<
  ConnectorType,
  ConnectorType.Static
>;

export const connectorToConnectorType = (connector: Web3ReactConnector) => {
  if (connector instanceof MetaMask) {
    return ConnectorType.MetaMask;
  }

  return ConnectorType.Static;
};

export interface Connector {
  accounts?: ReturnType<Web3ReactHooks["useAccounts"]>;
  activate: Web3ReactConnector["activate"];
  chainId?: ReturnType<Web3ReactHooks["useChainId"]>;
  deactivate: Web3ReactConnector["deactivate"];
  error?: ReturnType<Web3ReactHooks["useError"]>;
  isActivating?: ReturnType<Web3ReactHooks["useIsActivating"]>;
  isActive?: ReturnType<Web3ReactHooks["useIsActive"]>;
}

export const useConnector = (type: NonStaticConnectorType): Connector => {
  const connectors = useSelector<HooksState, any>(
    (state) => state.connectors as Connectors
  );

  const [connector, hooks] = connectors[type];

  const activate = () => connector.activate();
  const deactivate = () => connector.deactivate();

  const { useAccounts, useChainId, useError, useIsActivating, useIsActive } =
    hooks;

  return {
    accounts: useAccounts(),
    activate,
    chainId: useChainId(),
    deactivate,
    error: useError(),
    isActivating: useIsActivating(),
    isActive: useIsActive(),
  };
};
