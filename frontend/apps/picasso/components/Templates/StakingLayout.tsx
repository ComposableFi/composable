import { FC, useEffect } from "react";
import Default from "@/components/Templates/Default";
import { usePicassoProvider } from "substrate-react";
import { subscribeRewardPools } from "@/stores/defi/polkadot/stakingRewards/subscribeRewardPools";
import { useStore } from "@/stores/root";
import { Skeleton } from "@mui/material";

export const StakingLayout: FC = ({ children }) => {
  const { parachainApi } = usePicassoProvider();
  const isLoaded = useStore((store) => store.isRewardPoolLoaded);
  useEffect(() => {
    return subscribeRewardPools(parachainApi);
  }, [parachainApi]);

  if (!isLoaded) {
    return (
      <Default>
        <Skeleton variant="rounded" width="100%" height="300px" />
      </Default>
    );
  }
  return <Default>{children}</Default>;
};
