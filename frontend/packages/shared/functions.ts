export type Falsy = null | undefined;

export function callIf<T>(subject: T | Falsy, fn: (subject: T) => any) {
  if (subject) {
    return fn(subject);
  }

  return Promise.resolve(() => {});
}

type NonNullableArrItems<T extends readonly unknown[]> = T extends [
  infer A,
  ...infer B
]
  ? [NonNullable<A>, ...NonNullableArrItems<B>]
  : [];

const filterExist = (arr: readonly unknown[]) =>
  arr.every((dep) => dep !== undefined && dep !== null);

export function callbackGate<
  T extends readonly unknown[],
  Y extends NonNullableArrItems<T>
>(fn: (...args: [...deps: Y]) => any, ...requiredDeps: T): any {
  if (!filterExist(requiredDeps) || requiredDeps.length === 0) {
    return Promise.resolve(() => {});
  }

  return fn(...(requiredDeps as unknown as Y));
}
