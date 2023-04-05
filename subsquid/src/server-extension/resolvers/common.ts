export function getRange(range: string): Date[] {
  const dates: Date[] = [];
  const now = new Date();

  if (range === "day") {
    for (let i = -24; i <= 0; i += 1) {
      dates.push(new Date(now.getFullYear(), now.getMonth(), now.getDate(), now.getHours() + i));
    }
  } else if (range === "week") {
    for (let i = -7; i <= 0; i += 1) {
      dates.push(new Date(now.getFullYear(), now.getMonth(), now.getDate() + i, 0));
    }
  } else if (range === "month") {
    for (let i = -30; i <= 0; i += 1) {
      dates.push(new Date(now.getFullYear(), now.getMonth(), now.getDate() + i, 0));
    }
  } else if (range === "year") {
    for (let i = -12; i <= 0; i += 1) {
      dates.push(new Date(now.getFullYear(), now.getMonth() + i, 1, 0));
    }
  } else if (range === "all") {
    for (
      let i = new Date(2023, 0, 1);
      i <= new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0);
      i.setDate(i.getDate() + 1)
    ) {
      dates.push(new Date(i.getFullYear(), i.getMonth(), i.getDate(), 0));
    }
  } else {
    throw new Error("Invalid range. It should be 'day', 'week', 'month', 'year' or 'all'.");
  }

  return dates;
}

export function getVolumeRange(range: string): Date[] {
  const dates: Date[] = [];
  const now = new Date();

  if (range === "now") {
    dates.push(now);
  } else if (range === "month") {
    for (let day = -30; day <= 0; day += 1) {
      dates.push(new Date(now.getFullYear(), now.getMonth(), now.getDate() + day, 0));
    }
  } else if (range === "year") {
    for (let week = -52; week <= 0; week += 1) {
      dates.push(new Date(now.getFullYear(), now.getMonth(), now.getDate() + 7 * week, 0));
    }
  } else {
    throw new Error("Invalid range. It should be 'now', 'month' or 'year'.");
  }

  return dates;
}
