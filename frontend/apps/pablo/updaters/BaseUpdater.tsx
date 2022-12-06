import PoolsUpdater from "@/updaters/pools/Updater";
import LiquidityUpdater from "@/updaters/liquidity/Updater";
import PoolStatsUpdater from "@/updaters/poolStats/Updater";
import AssetsUpdater from "@/updaters/assets/Updater";
import BondsUpdater from "@/updaters/bonds/Updater";
import StakingRewardsUpdater from "@/updaters/stakingRewards/Updater";

import { subscription } from "@/updaters/oracle/oracle";
import { useEffect } from "react";

const BaseUpdater = () => {

  useEffect(() => {
    return () => {
      subscription();
    }
  }, []);

  return (
    <>
      <AssetsUpdater />
      <LiquidityUpdater />
      <PoolStatsUpdater />
      <PoolsUpdater />
      <BondsUpdater />
      <StakingRewardsUpdater />
    </>
  );
};

export default BaseUpdater;
