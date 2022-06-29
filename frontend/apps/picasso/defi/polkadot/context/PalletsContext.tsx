import React, { createContext, useEffect, useState } from "react";
import { usePicassoProvider } from "../hooks";
import { CrowdloanRewards } from "../pallets/CrowdloanRewards";

export const PalletsContext = createContext({
  crowdloanRewards: undefined as CrowdloanRewards | undefined,
});

export const PalletsContextProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const { parachainApi, apiStatus } = usePicassoProvider();

  useEffect(() => {
    if (parachainApi && apiStatus === "connected") {
      setPallets({
        crowdloanRewards: new CrowdloanRewards(parachainApi),
      });
    }
  }, [parachainApi, apiStatus]);

  const [pallets, setPallets] = useState({
    crowdloanRewards: undefined as CrowdloanRewards | undefined,
  });

  return (
    <PalletsContext.Provider value={pallets}>
      {children}
    </PalletsContext.Provider>
  );
};
