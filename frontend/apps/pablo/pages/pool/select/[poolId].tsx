import type { NextPage } from "next";
import { Box, Container, Typography } from "@mui/material";
import Default from "@/components/Templates/Default";
import { ConnectWalletFeaturedBox, Link, PageTitle } from "@/components";
import { PoolDetails } from "@/components/Organisms/pool/PoolDetails";
import { useDotSamaContext, useParachainApi } from "substrate-react";
import { useRouter } from "next/router";
import useStore from "@/store/useStore";
import { pipe } from "fp-ts/lib/function";
import { option } from "fp-ts";
import { useEffect, useState } from "react";
import { subscribePools } from "@/store/pools/subscribePools";
import { PoolSelectSkeleton } from "@/components/Templates/pools/PoolSelectSkeleton";

const isConnected = (status: string) => status === "connected";

const breadcrumbs = [
  <Link key="pool" underline="none" color="primary" href="/pool">
    Pool
  </Link>,
  <Typography key="create-pool" color="text.primary">
    Select
  </Typography>,
];

const PoolDetailsPage: NextPage = () => {
  const router = useRouter();
  const getPoolById = useStore((store) => store.pools.getPoolById);
  const { extensionStatus } = useDotSamaContext();
  const { parachainApi } = useParachainApi("picasso");
  const [poolId, setPoolId] = useState<string>("");
  const isPoolConfigLoaded = useStore((store) => store.pools.isLoaded);

  useEffect(() => {
    if (!router.isReady) return;
    const { poolId } = router.query;
    if (isNaN(Number(poolId))) {
      router.push("/pool");
      return;
    }
    setPoolId(poolId as string);
  }, [router]);

  useEffect(() => {
    if (parachainApi) {
      return subscribePools(parachainApi);
    }
  }, [parachainApi]);

  return pipe(
    isPoolConfigLoaded,
    option.fromPredicate((a) => a),
    option.fold(
      () => <PoolSelectSkeleton />,
      () => {
        return pipe(
          getPoolById(poolId as string),
          option.fold(
            () => <PoolSelectSkeleton />,
            (a) => (
              <Default breadcrumbs={breadcrumbs}>
                <Container maxWidth="lg">
                  <Box
                    display="flex"
                    flexDirection="column"
                    alignItems="center"
                    mb={8}
                  >
                    <PageTitle
                      title={`${a.config.assets[0].getSymbol()}/${a.config.assets[1].getSymbol()} Pool`}
                      subtitle="Earn tokens while adding liquidity."
                    />
                  </Box>
                  {isConnected(extensionStatus) ? (
                    <PoolDetails poolId={poolId as string} />
                  ) : (
                    <ConnectWalletFeaturedBox />
                  )}
                </Container>
              </Default>
            )
          )
        );
      }
    )
  );
};

export default PoolDetailsPage;
