import { useEffect } from "react";
import { useBlockInterval } from "@/defi/polkadot/hooks/useBlockInterval";
import { useStore } from "@/stores/root";
import { usePicassoProvider } from "@/defi/polkadot/hooks/index";
import { callbackGate } from "shared";
import BigNumber from "bignumber.js";

export const useTelemetry = () => {
  const time = useBlockInterval();
  const { finalizedBlock, lastBlock } = useStore(
    ({ statsTelemetry }) => statsTelemetry.telemetryData
  );
  const getBlockAverage = useStore(
    ({ statsTelemetry }) => statsTelemetry.getBlockAverage
  );
  const setFinalizedBlock = useStore(
    ({ statsTelemetry }) => statsTelemetry.setFinalizedBlock
  );
  const setLastBlock = useStore(
    ({ statsTelemetry }) => statsTelemetry.setLastBlock
  );

  const pushAverageTime = useStore(
    ({ statsTelemetry }) => statsTelemetry.pushAverageTime
  );

  const { parachainApi } = usePicassoProvider();

  useEffect(() => {
    pushAverageTime(time);
    let lastTime = Date.now();
    let fullBlockCheck = false; // We set this to wait 1 additional block to ensure we have a full block time and not a partial block time
    const unsubPromise = callbackGate(
      (api) =>
        api.rpc.chain.subscribeNewHeads((header) => {
          setLastBlock(new BigNumber(header.number.toString()));
          const diff = new BigNumber(Date.now() - lastTime);
          if (fullBlockCheck) {
            pushAverageTime(diff);
            lastTime = Date.now();
          }
          fullBlockCheck = true;
        }),
      parachainApi
    );

    return () => {
      unsubPromise.then((unsubscribe: () => void) => unsubscribe?.());
    };
  }, [parachainApi]); // eslint-disable-line react-hooks/exhaustive-deps

  useEffect(() => {
    const unsubPromise = callbackGate(
      (api) =>
        api.rpc.chain.subscribeFinalizedHeads((header) =>
          setFinalizedBlock(new BigNumber(header.number.toString()))
        ),
      parachainApi
    );

    return () => {
      unsubPromise.then((unsubscribe: () => void) => unsubscribe?.());
    };
  }, [parachainApi]); // eslint-disable-line react-hooks/exhaustive-deps

  return {
    finalizedBlock,
    lastBlock,
    getBlockAverage,
  };
};
