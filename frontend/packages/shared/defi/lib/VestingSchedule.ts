import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "../unit";
import BigNumber from "bignumber.js";

export type VestingScheduleType = "block" | "moment";

export class VestingScheduleWindow {
    protected readonly __start: BigNumber;
    protected readonly __period: BigNumber;

    static fromJSON(scheduleWindow: any): VestingScheduleWindow {
        try {
            const start = scheduleWindow.blockNumberBased
                ? new BigNumber(scheduleWindow.blockNumberBased.start)
                : new BigNumber(scheduleWindow.momentBased.start);

            const period = scheduleWindow.blockNumberBased
                ? new BigNumber(scheduleWindow.blockNumberBased.period)
                : new BigNumber(scheduleWindow.momentBased.period);

            return new VestingScheduleWindow(start, period);
        } catch (err: any) {
            throw new Error(err.message);
        }
    }

    toJSON(): {
        start: string;
        period: string
    } {
        return {
            start: this.__start.toString(),
            period: this.__period.toString()
        }
    }

    constructor(start: BigNumber, period: BigNumber) {
        this.__start = start;
        this.__period = period;
    }

    getStart(): BigNumber {
        return this.__start;
    }

    getPeriod(): BigNumber {
        return this.__period;
    }
}

export class VestingSchedule {
    protected readonly __api: ApiPromise;
    protected readonly __perPeriod: BigNumber;
    protected readonly __periodCount: BigNumber;
    protected readonly __alreadyClaimed: BigNumber;
    protected readonly __vestingScheduleId: BigNumber;
    protected readonly __type: VestingScheduleType;
    protected readonly __window: VestingScheduleWindow;

    static async fromAddressAndAssetId(
        api: ApiPromise,
        address: string,
        assetId: string
    ): Promise<VestingSchedule[]> {
        try {
            const vestingSchedule = await api.query.vesting.vestingSchedules(
              address,
              assetId
            );
        
            const _schedules = vestingSchedule.toJSON();
            return Object.values(_schedules as any)
                .map((i) => VestingSchedule.fromJSON(api, i))
          } catch (err: any) {
            console.error('[fromAddressAndAssetId] ', err.message);
            throw new Error(err.message);
          }
    }

    static fromJSON(api: ApiPromise, vestingSchedule: any): VestingSchedule {
        try {
            const type = vestingSchedule.window.blockNumberBased ? "block" : "moment";
            const window = VestingScheduleWindow.fromJSON(vestingSchedule.window);
            const perPeriod = fromChainIdUnit(vestingSchedule.perPeriod);
            const alreadyClaimed = fromChainIdUnit(vestingSchedule.alreadyClaimed);
            const vestingScheduleId = new BigNumber(
                vestingSchedule.vestingScheduleId
            );
            const periodCount = new BigNumber(vestingSchedule.periodCount);

            return new VestingSchedule(
                api,
                perPeriod,
                periodCount,
                alreadyClaimed,
                vestingScheduleId,
                type,
                window
            );
        } catch (err: any) {
            throw new Error(err.message);
        }
    }

    constructor(
        api: ApiPromise,
        perPeriod: BigNumber,
        periodCount: BigNumber,
        alreadyClaimed: BigNumber,
        vestingScheduleId: BigNumber,
        type: VestingScheduleType,
        window: VestingScheduleWindow
    ) {
        this.__api = api;
        this.__perPeriod = perPeriod;
        this.__periodCount = periodCount;
        this.__alreadyClaimed = alreadyClaimed;
        this.__vestingScheduleId = vestingScheduleId;
        this.__type = type;
        this.__window = window;
    }

    async getClaimableAt(): Promise<BigNumber> {
        const alreadyClaimed = this.__alreadyClaimed;
        const perPeriod = this.__perPeriod;
        const periodCount = this.__periodCount;
        const claimable = new BigNumber(0);

        if (this.__type === "block") {
            const currentBlockBN = await this.__api.query.system.number();
            const currentBlockBn = new BigNumber(currentBlockBN.toString());
        } else {
            const currentTimestampBN = await this.__api.query.timestamp.now();
            const currentTimestampBn = new BigNumber(currentTimestampBN.toString());

        }

        return claimable;
    }
}
