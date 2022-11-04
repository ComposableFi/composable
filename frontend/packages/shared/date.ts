export const getHumanizedDateDiff = (timestamp1: number, timestamp2: number) => {
  const diff = Math.abs(timestamp2 - timestamp1);
  const diffDays = diff / 1000 / 3600 / 24;
  const diffMonths = diffDays / 31;

  if (diffDays > 31) {
    return diffMonths.toFixed(1) + " months";
  } else {
    return diffDays.toFixed(1) + " days";
  }
};

export const getFullHumanizedDateDiff = (timestamp1: number, timestamp2: number) => {
  let diff = Math.abs(timestamp2 - timestamp1);
  const days = Math.floor(diff / (1000 * 3600 * 24));
  diff = diff % (1000 * 3600 * 24);
  const hours = Math.floor(diff / (1000 * 3600));
  diff = diff % (1000 * 3600);
  const minutes = Math.floor(diff / (1000 * 60));
  const seconds = Math.floor(diff % (1000 * 60) / 1000);

  return (days > 0 ? days + ":" : "")
    + (hours < 10 ? "0" + hours : hours) + ":"
    + (minutes < 10 ? "0" + minutes : minutes) + ":"
    + (seconds < 10 ? "0" + seconds : seconds);
};

export const PRESET_RANGE = ["24h", "1w", "1m", "1y", "ALL"] as const;
export type PresetRange = typeof PRESET_RANGE[number];

export function getDiffInMinutes(source: Date, target: Date, options = {
  abs: true
}) {
  const diff = (source.getTime() - target.getTime()) / 1000;
  const out = Math.round(diff * 60);
  return options.abs ? Math.abs(out) : out;
}

/**
 * Calculates ranges in well-known preset, which can be used via PRESET_RANGE
 *
 * @param {PresetRange} preset
 * @returns {[(string | null), string, number]}
 */
export function getRange(preset: PresetRange): [(string | null), string, number] {
  const today = new Date();
  const tomorrow = new Date(today);
  tomorrow.setDate(tomorrow.getDate() + 1);
  const dateTo = tomorrow.toISOString();

  const range: {
    [key in PresetRange]: () => [string | null, string, number]
  } = {
    "24h": () => {
      const yesterday = new Date(today);
      yesterday.setHours(yesterday.getHours() - 24);
      return [yesterday.toISOString(), dateTo, 60];
    },
    "1w": () => {
      const lastWeek = new Date(today);
      lastWeek.setDate(lastWeek.getDate() - 7);
      return [lastWeek.toISOString(), dateTo, 24 * 60];
    },
    "1m": () => {
      const lastMonth = new Date(today);
      lastMonth.setMonth(lastMonth.getMonth() - 1);
      return [lastMonth.toISOString(), dateTo, 24 * 60];
    },
    "1y": () => {
      const lastYear = new Date(today);
      lastYear.setFullYear(lastYear.getFullYear() - 1);
      return [lastYear.toISOString(), dateTo, 30 * 24 * 60];
    },
    "ALL": () => {
      return [null, tomorrow.toISOString(), 30 * 24 * 60];
    }
  };

  return range[preset]();
}
