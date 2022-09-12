export function callIf<T>(subject: T | undefined, fn: (subject: T) => any) {
  if (subject) {
    return fn(subject);
  }

  return Promise.resolve(() => {});
}
