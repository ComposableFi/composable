import LiquidityUpdater from "@/updaters/liquidity/Updater";
import PoolStatsUpdater from "@/updaters/poolStats/Updater";
import AssetsUpdater from "@/updaters/assets/Updater";

import { subscription } from "@/updaters/oracle/oracle";
import { useEffect } from "react";

const BaseUpdater = () => {
  useEffect(() => {
    return () => {
      subscription();
    };
  }, []);

  return (
    <>
      <AssetsUpdater />
      <LiquidityUpdater />
      <PoolStatsUpdater />
    </>
  );
};

export default BaseUpdater;
