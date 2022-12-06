import moment from "moment";

export const SECONDS = 1 * 1000;
export const MINUTES = 60 * SECONDS;
export const HOURS = 60 * MINUTES;
/** 1 Day in milliseconds */
export const DAYS = 24 * HOURS;

export type Period = {
  days?: number;
  years?: number;
  months?: number;
  weeks?: number;
};

export function createDurationPresetLabel(period: Period): string {
  let suf = period.days
    ? "Day"
    : period.weeks
    ? "Week"
    : period.months
    ? "Month"
    : period.years
    ? "Year"
    : "-";

  let _sufVal = period.days
    ? period.days
    : period.weeks
    ? period.weeks
    : period.months
    ? period.months
    : period.years
    ? period.years
    : 0;

  if (_sufVal > 0) {
    suf += "s";
  }

  return `${_sufVal.toFixed(2)} ${suf}`;
}

export function calculatePeriod(durationInSeconds: string | number): Period {
  return durationInSeconds < 7 * (DAYS / 1000)
    ? { days: moment.duration(durationInSeconds, "seconds").asDays() }
    : durationInSeconds < 30 * (DAYS / 1000)
    ? { weeks: moment.duration(durationInSeconds, "seconds").asWeeks() }
    : durationInSeconds < 365 * (DAYS / 1000)
    ? { months: moment.duration(durationInSeconds, "seconds").asMonths() }
    : { years: moment.duration(durationInSeconds, "seconds").asYears() };
}
