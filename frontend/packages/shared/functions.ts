export function callIf<T>(
  subject: T | undefined | null,
  fn: (subject: T) => any
) {
  if (subject) {
    return fn(subject);
  }

  return Promise.resolve(() => {});
}
