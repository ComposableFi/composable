import { useAppDispatch } from "@/hooks/store";
import React, { createContext, useEffect, useState } from "react";
import { usePicassoProvider } from "../hooks";
import { CrowdloanRewards } from "../pallets/CrowdloanRewards";
import { ApolloStatsPrices } from "../pallets/ApolloStatsPrices";

export const PalletsContext = createContext({
  crowdloanRewards: undefined as CrowdloanRewards | undefined,
  apolloStats: undefined as ApolloStatsPrices | undefined,
});

export const PalletsContextProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const appDispatch = useAppDispatch();
  const { parachainApi, apiStatus } = usePicassoProvider();

  useEffect(() => {
    if (parachainApi && apiStatus === "connected") {
      setPallets({
        crowdloanRewards: new CrowdloanRewards(parachainApi, appDispatch),
        apolloStats: new ApolloStatsPrices(parachainApi, appDispatch),
      });
    }
  }, [parachainApi, apiStatus]);

  const [pallets, setPallets] = useState({
    crowdloanRewards: undefined as CrowdloanRewards | undefined,
    apolloStats: undefined as ApolloStatsPrices | undefined,
  });

  return (
    <PalletsContext.Provider value={pallets}>
      {children}
    </PalletsContext.Provider>
  );
};
