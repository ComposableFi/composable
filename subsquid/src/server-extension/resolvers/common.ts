export function getStartAndStep(range: string): {
  startHoursAgo: number;
  step: number;
} {
  if (
    range !== "day" &&
    range !== "week" &&
    range !== "month" &&
    range !== "year"
  ) {
    throw new Error(
      "Invalid range. It should be 'day', 'week', 'month' or 'year'."
    );
  }

  let startHoursAgo: number;
  let step: number;

  switch (range) {
    case "day": {
      startHoursAgo = 22;
      step = 1;
      break;
    }
    case "week": {
      startHoursAgo = 5 * 24;
      step = 24;
      break;
    }
    case "month": {
      startHoursAgo = 28 * 24;
      step = 24;
      break;
    }
    case "year":
    default: {
      startHoursAgo = 10 * 30 * 24;
      step = 24 * 30;
    }
  }

  return {
    startHoursAgo,
    step,
  };
}

export const DAY_IN_MS = 24 * 60 * 60 * 1000;
