export function callIf<T>(
  subject: T | undefined,
  fn: (subject: T) => Promise<() => void>
) {
  if (subject) {
    return fn(subject);
  }

  return Promise.resolve(() => {});
}
