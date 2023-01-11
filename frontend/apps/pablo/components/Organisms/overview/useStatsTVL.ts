import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { usePicaPriceDiscovery } from "@/defi/hooks/overview/usePicaPriceDiscovery";
import { flow, pipe } from "fp-ts/function";
import * as TE from "fp-ts/TaskEither";
import { fetchPabloOverviewStatsTVL } from "@/defi/subsquid/overview";
import * as E from "fp-ts/Either";
import { parseLockedValue } from "@/components/Organisms/overview/parseLockedValue";

export const useStatsTVL = () => {
  const [totalValueLocked, setTotalValueLocked] = useState(new BigNumber(0));
  const [statsLoading, setStatsLoading] = useState(false);
  const getTokenById = useStore((store) => store.substrateTokens.getTokenById);
  const picaPrice = usePicaPriceDiscovery();
  useEffect(() => {
    const task = pipe(
      TE.fromIO(() => setStatsLoading(true)),
      TE.chain(fetchPabloOverviewStatsTVL),
      TE.chainFirst(() => TE.fromIO(() => setStatsLoading(false)))
    );

    task().then(
      flow(
        E.match(
          () => setTotalValueLocked(new BigNumber(0)),
          ({ pabloOverviewStats }) => {
            const tvl = pabloOverviewStats.totalValueLocked.reduce(
              parseLockedValue(getTokenById, picaPrice),
              new BigNumber(0)
            );

            setTotalValueLocked(tvl);
          }
        )
      )
    );
  }, [getTokenById, picaPrice]);

  return [totalValueLocked, statsLoading] as const;
};
