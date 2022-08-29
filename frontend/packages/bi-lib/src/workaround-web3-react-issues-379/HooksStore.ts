import { getPriorityConnector, Web3ReactHooks } from '@web3-react/core'
import { Connector } from '@web3-react/types'
import { createStore } from "redux";

import type { NonStaticConnectorType } from '../hooks';

export type Connectors = {
  [key in NonStaticConnectorType]: [Connector, Web3ReactHooks];
}

export type PriorityConnector = ReturnType<typeof getPriorityConnector>;

export interface HooksState {
  connectors: Connectors | undefined;
  priorityConnector: PriorityConnector | undefined;
}

export const initialState: HooksState = {
  connectors: undefined,
  priorityConnector: undefined,
};

export const STORE_CONNECTORS = "store_connectors";

export const STORE_PRIORITY_CONNECTOR = "store_priority_connector";

interface StoreConnectorsAction {
  type: typeof STORE_CONNECTORS;
  payload: Connectors;
}

interface StorePriorityConnectorAction {
  type: typeof STORE_PRIORITY_CONNECTOR;
  payload: PriorityConnector;
}

export const storeConnectorsReducer = (
  state: HooksState,
  connectors: Connectors,
) => {
  return {
    ...state,
    connectors,
  };
};

export const storePriorityConnectorReducer = (
  state: HooksState,
  priorityConnector: PriorityConnector,
) => {
  return {
    ...state,
    priorityConnector,
  };
};

type Action =
  | StoreConnectorsAction
  | StorePriorityConnectorAction;

const hooksReducer = (
  state: HooksState | undefined,
  action: Action
): HooksState => {
  const usedState = state ? state : initialState;

  switch (action.type) {
    case STORE_CONNECTORS:
      return storeConnectorsReducer(
        usedState,
        action.payload
      );
    case STORE_PRIORITY_CONNECTOR:
      return storePriorityConnectorReducer(
        usedState,
        action.payload
      );
    default:
      return usedState;
  }
};

export const HooksStore = createStore(
  hooksReducer,
  initialState
);
