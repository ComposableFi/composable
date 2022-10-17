import { StakingRewardPool } from "@/defi/types";
import { Mark } from "@mui/base";
import { DAYS } from "../constants";
import moment from "moment";

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
        ? period.days.toFixed(2)
        : period.weeks
            ? period.weeks.toFixed(2)
            : period.months
                ? period.months.toFixed(2)
                : period.years
                    ? period.years.toFixed(2)
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

export interface DurationPresetMark extends Mark {
    period: Period;
    periodInSeconds: string;
    periodInString: string;
}

export function extractDurationPresets(
    stakingPool: StakingRewardPool | undefined
): Array<DurationPresetMark> {
    if (!stakingPool) return [];

    return Object.keys(stakingPool.lock.durationPresets).map((i) => {
        const seconds = Number(i);
        const period = calculatePeriod(seconds);
        const periodInString = createDurationPresetLabel(period);

        return {
            periodInString,
            period,
            periodInSeconds: i,
            value: stakingPool.lock.durationPresets[i].toNumber(),
        };
    });
}

/**
 * Get preset duration expiry
 * @param presetDuration amount of lock time in seconds
 * @returns 
 */
export function calculatePresetExpiry(presetDuration: number): moment.Moment {
    const timeNow = moment();
    timeNow.add(presetDuration, "seconds");
    return timeNow;
}