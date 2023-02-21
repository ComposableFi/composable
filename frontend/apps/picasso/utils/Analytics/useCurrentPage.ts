import { useRouter } from "next/router";
import { useMemo } from "react";
import { routesConfig } from "@/utils/routesConfig";

export function useCurrentPage() {
  const router = useRouter();
  const currentPath = useMemo(() => router.asPath, [router.asPath]);

  return {
    get label() {
      const route = routesConfig.find((route) =>
        route.matches?.includes(currentPath)
      );
      return route?.label;
    },
  };
}
