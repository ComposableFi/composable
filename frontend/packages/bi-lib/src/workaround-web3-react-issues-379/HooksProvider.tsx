import { getPriorityConnector } from "@web3-react/core";
import { FC, ReactElement, useEffect, useMemo, useState } from "react";
import { useDispatch } from "react-redux";

import { hooks as metaMaskHooks, metaMask } from "../connectors/metaMask";
import { ConnectorType } from "../hooks";
import { Connectors, STORE_CONNECTORS, STORE_PRIORITY_CONNECTOR } from "./HooksStore";

const HooksProvider: FC = ({ children }) => {
  const dispatch = useDispatch();
  const [isReady, setReady] = useState<boolean>(false);

  const connectors : Connectors = useMemo(
    () => ({ [ConnectorType.MetaMask]: [metaMask, metaMaskHooks] }),
    []
  )

  const priorityConnector = useMemo(
    () => {
      const connectorsArray = Object.values(connectors);
      return getPriorityConnector(...connectorsArray);
    },
    [connectors]
  );

  useEffect(
    () => {
      dispatch({
        type: STORE_CONNECTORS,
        payload: connectors,
      });

      dispatch({
        type: STORE_PRIORITY_CONNECTOR,
        payload: priorityConnector,
      });

      setReady(true);
    },
    [connectors, dispatch, priorityConnector]
  );

  return isReady ? children as ReactElement : null;
};

export default HooksProvider;
