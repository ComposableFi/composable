import { ApiPromise } from "@polkadot/api";
import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";

export function useCurrentBlockAndTime(api?: ApiPromise) {
  const [time, setTime] = useState<Date>(new Date());
  const [block, setBlock] = useState(new BigNumber(0));

  useEffect(() => {
    let unsubTime = Promise.resolve(() => {});
    let unsubBlock = Promise.resolve(() => {});
    if (api) {
      // @ts-ignore
      unsubBlock = api.query.system.number((block) => {
        setBlock(new BigNumber(block.toString()));
      });
      // @ts-ignore
      unsubTime = api.query.timestamp.now((moment) => {
        setTime(new Date(moment.toJSON()));
      });
    }

    return () => {
      unsubBlock.then((v) => v());
      unsubTime.then((v) => v());
    };
  }, [api]);

  return {
    block,
    time,
  };
}
