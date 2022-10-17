export function getTimelineParams(
  intervalMinutes: number,
  dateFrom?: string,
  dateTo?: string
): {
  where: string[];
  params: number[];
} {
  const intervalMilliseconds = intervalMinutes * 60 * 1000;
  const params: number[] = [intervalMilliseconds];
  const where: string[] = [];
  let from: number;

  // Set "from" filter
  if (dateFrom) {
    from = new Date(dateFrom).valueOf();
  } else {
    from = 0;
  }
  from = Math.floor(from / intervalMilliseconds) * intervalMilliseconds;
  where.push(`timestamp > $${params.push(from)}`);

  // Set "to" filter
  if (dateTo) {
    let to = new Date(dateTo).valueOf();
    to = Math.ceil(to / intervalMilliseconds) * intervalMilliseconds;
    where.push(`timestamp < $${params.push(to)}`);
  }

  return {
    where,
    params,
  };
}
