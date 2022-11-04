import { AVERAGE_BLOCK_TIME, DAYS, DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useState } from "react";
import { PabloLiquidityBootstrappingPool } from "shared";
import { useBlockInterval } from "../useBlockInterval";
import { useAsyncEffect } from "@/hooks/useAsyncEffect";
import useBlockNumber from "../useBlockNumber";
import BigNumber from "bignumber.js";

export function useAuctionTiming(
    auction: PabloLiquidityBootstrappingPool | null
): {
    duration: number;
    isActive: boolean;
    isEnded: boolean;
    willStart: boolean;
    startTimestamp: number;
    endTimestamp: number;
} {
    const startBlock = auction?.getSaleConfig().start ?? new BigNumber(0);
    const endBlock = auction?.getSaleConfig().end ?? new BigNumber(0);
    
    const blockInterval = useBlockInterval();
    const blockNumber = useBlockNumber(DEFAULT_NETWORK_ID);
    
    const isActive = blockNumber.gt(startBlock) && blockNumber.lt(endBlock)
    const isEnded = blockNumber.gt(endBlock)
    const willStart = !isActive && !isEnded && blockNumber.lt(startBlock)

    const [duration, setDuration] = useState(0);
    const [startTimestamp, setStartTimestamp] = useState(0);
    const [endTimestamp, setEndTimestamp] = useState(0);
    useAsyncEffect(async (): Promise<void> => {
        if (!auction) return;

        let _blockInterval = new BigNumber(AVERAGE_BLOCK_TIME);
        if (blockInterval) {
            _blockInterval = new BigNumber(blockInterval.toString());
        }

        const { startTimestamp, endTimestamp } = await auction.getSaleTiming(
            blockNumber,
            _blockInterval
        )

        setStartTimestamp(startTimestamp)
        setEndTimestamp(endTimestamp)
        setDuration(
            Math.round((endTimestamp - startTimestamp) / DAYS)
        );
    }, [blockNumber, blockInterval, auction]);

    return {
        duration,
        isActive,
        isEnded,
        willStart,
        startTimestamp,
        endTimestamp
    }
}