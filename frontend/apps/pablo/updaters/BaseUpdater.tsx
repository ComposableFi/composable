import LiquidityUpdater from "@/updaters/liquidity/Updater";
import AssetsUpdater from "@/updaters/assets/Updater";

import { subscribePrices } from "@/updaters/oracle/oracle";
import { useEffect } from "react";

const BaseUpdater = () => {
  useEffect(() => {
    return subscribePrices();
  }, []);

  return (
    <>
      <AssetsUpdater />
      <LiquidityUpdater />
    </>
  );
};

export default BaseUpdater;
