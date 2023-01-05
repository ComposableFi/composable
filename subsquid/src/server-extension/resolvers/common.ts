export function getStartAndStep(range: string): {
  startHoursAgo: number;
  step: number;
} {
  if (range !== "day" && range !== "week" && range !== "month" && range !== "year") {
    throw new Error("Invalid range. It should be 'day', 'week', 'month' or 'year'.");
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
    step
  };
}

export const DAY_IN_MS = 24 * 60 * 60 * 1000;

export function getRange(range: string): Date[] {
  const dates: Date[] = [];
  const now = new Date();

  if (range === "day") {
    for (let i = -22; i <= 0; i += 1) {
      dates.push(new Date(now.getFullYear(), now.getMonth(), now.getDate(), now.getHours() + i));
    }
  } else if (range === "week") {
    for (let i = -5; i <= 0; i += 1) {
      dates.push(new Date(now.getFullYear(), now.getMonth(), now.getDate() + i, 0));
    }
  } else if (range === "month") {
    for (let i = -28; i <= 0; i += 1) {
      dates.push(new Date(now.getFullYear(), now.getMonth(), now.getDate() + i, 0));
    }
  } else if (range === "year") {
    for (let i = -10; i <= 0; i += 1) {
      dates.push(new Date(now.getFullYear(), now.getMonth() + i, 1, 0));
    }
  } else {
    throw new Error("Invalid range. It should be 'day', 'week', 'month' or 'year'.");
  }

  dates.push(now);

  return dates;
}
