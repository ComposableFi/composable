import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import useStore from "@/store/useStore";

export const usePoolDetail = () => {
  const router = useRouter();
  const [poolId, setPoolId] = useState<string>("");
  const pools = useStore((store) => store.pools.config);
  const isPoolConfigLoaded = useStore((store) => store.pools.isLoaded);
  const pool = isPoolConfigLoaded
    ? pools.find((pool) => pool.poolId.toString() === poolId) || null
    : null;
  const isTokensLoaded = useStore(
    (store) => store.substrateTokens.hasFetchedTokens
  );
  const shouldRedirect = isTokensLoaded && isPoolConfigLoaded;

  useEffect(() => {
    if (!router.isReady) return;
    const { poolId } = router.query;
    if (isNaN(Number(poolId)) && shouldRedirect) {
      router.push("/pool");
      return;
    }

    if (!isNaN(Number(poolId))) {
      setPoolId(poolId as string);
    }
  }, [router, shouldRedirect]);
  return {
    poolId,
    pool,
  };
};
