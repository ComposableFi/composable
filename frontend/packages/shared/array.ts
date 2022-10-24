export function head<T>(items: Array<T>): T | undefined {
  return items.at(0);
}

export function tail<T>(items: Array<T>): T | undefined {
  return items.at(-1);
}
