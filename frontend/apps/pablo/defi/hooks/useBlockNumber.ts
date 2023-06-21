import BigNumber from "bignumber.js";
import { useEffect, useState } from "react";
import { ParachainId, useParachainApi } from "substrate-react";

export default function useBlockNumber(parachainId: ParachainId): BigNumber {
  const { parachainApi } = useParachainApi(parachainId);
  const [blockNumber, setBlockNumber] = useState(new BigNumber(0));

  useEffect(() => {
    if (parachainApi) {
      const sub = parachainApi.rpc.chain.subscribeNewHeads((header) => {
        setBlockNumber(new BigNumber(header.number.toString()));
      });

      return function () {
        sub.then((s) => s());
      };
    }
  }, [parachainApi]);

  return blockNumber;
}
