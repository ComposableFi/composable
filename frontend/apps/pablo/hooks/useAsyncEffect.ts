import { useEffect } from "react";

export function useAsyncEffect(
  f: () => Promise<void>,
  dependencies: unknown[]
) {
  useEffect(function () {
    f().catch(console.error);
  }, dependencies); // eslint-disable-line react-hooks/exhaustive-deps
}
