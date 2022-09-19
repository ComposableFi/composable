import { StakingRewardPool } from "@/defi/types";
import BigNumber from "bignumber.js";
import moment from "moment";
import { DAYS } from "../constants";

type Period = {
    days?: number;
    years?: number;
    months?: number;
    weeks?: number;
};

export function createDurationPresetLabel(period: Period): string {
    let suf = period.days
        ? period.days <= 1
            ? "Day"
            : "Days"
        : period.weeks
            ? period.weeks <= 1
                ? "Week"
                : "Weeks"
            : period.months
                ? period.months <= 1
                    ? "Month"
                    : "Months"
                : period.years
                    ? period.years <= 1
                        ? "Year"
                        : "Years"
                    : "-";

    return `${period.days
        ? period.days
        : period.weeks
            ? period.weeks
            : period.months
                ? period.months
                : period.years
                    ? period.years
                    : "-"
        } ${suf}`;
}

export function calculatePeriod(durationInSeconds: string | number): Period {
    return durationInSeconds < 7 * (DAYS / 1000) ?
        { days: moment.duration(durationInSeconds, "seconds").asDays() } :
        durationInSeconds < 30 * (DAYS / 1000) ?
            { weeks: moment.duration(durationInSeconds, "seconds").asWeeks() } :
            durationInSeconds < 365 * (DAYS / 1000) ?
                { months: moment.duration(durationInSeconds, "seconds").asMonths() } :
                { years: moment.duration(durationInSeconds, "seconds").asYears() };
}

export function extractDurationPresets(
    stakingPool: StakingRewardPool | undefined
): Array<{
    label: string;
    period: Period;
    multiplier: BigNumber;
    value: string;
}> {
    if (!stakingPool) return [];

    return Object.keys(stakingPool.lock.durationPresets).map((i) => {
        const seconds = Number(i);
        const period = calculatePeriod(seconds);
        const label = createDurationPresetLabel(period);

        return {
            label,
            period,
            multiplier: stakingPool.lock.durationPresets[i],
            value: i,
        };
    });
}
