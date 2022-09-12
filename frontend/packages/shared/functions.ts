type Falsy = undefined | null;

export function callIf<T>(subject: T | Falsy, fn: (subject: T) => any) {
  if (subject) {
    return fn(subject);
  }

  return Promise.resolve(() => {});
}
