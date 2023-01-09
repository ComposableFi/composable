import BigNumber from "bignumber.js";
import { useEffect, useState } from "react";
import { useParachainApi } from "substrate-react";
import { ParachainId } from "shared";

export default function useBlockNumber(parachainId: ParachainId): BigNumber {
  const { parachainApi } = useParachainApi(parachainId);
  const [blockNumber, setBlockNumber] = useState(new BigNumber(0));

  useEffect(() => {
    if (parachainApi) {
      const sub = parachainApi.rpc.chain.subscribeNewHeads(
        (header: { number: { toString: () => BigNumber.Value } }) => {
          setBlockNumber(new BigNumber(header.number.toString()));
        }
      );

      return function () {
        sub.then((s) => s());
      };
    }
  }, [parachainApi]);

  return blockNumber;
}
