import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { usePicaPriceDiscovery } from "@/defi/hooks/overview/usePicaPriceDiscovery";
import { flow, pipe } from "fp-ts/function";
import * as TE from "fp-ts/TaskEither";
import { fetchPabloOverviewDailyVolume } from "@/defi/subsquid/overview";
import * as E from "fp-ts/Either";
import { parseLockedValue } from "@/components/Organisms/overview/parseLockedValue";

export const useDailyVolume = () => {
  const [isLoading, setIsLoading] = useState(false);
  const [dailyVolume, setDailyVolume] = useState(new BigNumber(0));
  const getTokenById = useStore((store) => store.substrateTokens.getTokenById);
  const picaPrice = usePicaPriceDiscovery();

  useEffect(() => {
    const task = pipe(
      TE.fromIO(() => setIsLoading(true)),
      TE.chain(fetchPabloOverviewDailyVolume),
      TE.chainFirst(() => TE.fromIO(() => setIsLoading(false)))
    );

    task().then(
      flow(
        E.match(
          () => setDailyVolume(new BigNumber(0)),
          ({ pabloOverviewStats }) => {
            const volume = pabloOverviewStats.dailyVolume.reduce(
              parseLockedValue(getTokenById, picaPrice),
              new BigNumber(0)
            );
            setDailyVolume(volume);
          }
        )
      )
    );
  }, [getTokenById, picaPrice]);

  return [dailyVolume, isLoading] as const;
};
