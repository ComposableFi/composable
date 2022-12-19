import type { NextPage } from "next";
import { Box, Container, Typography } from "@mui/material";
import Default from "@/components/Templates/Default";
import { ConnectWalletFeaturedBox, Link, PageTitle } from "@/components";
import { PoolDetails } from "@/components/Organisms/pool/PoolDetails";
import { useDotSamaContext, useParachainApi } from "substrate-react";
import useStore from "@/store/useStore";
import { pipe } from "fp-ts/lib/function";
import { option } from "fp-ts";
import { useEffect } from "react";
import { subscribePools } from "@/store/pools/subscribePools";
import { PoolSelectSkeleton } from "@/components/Templates/pools/PoolSelectSkeleton";
import { usePoolDetail } from "@/defi/hooks/pools/usePoolDetail";
import { subscribePoolAmount } from "@/store/pools/subscribePoolAmount";

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
  const getPoolById = useStore((store) => store.pools.getPoolById);
  const { extensionStatus } = useDotSamaContext();
  const { parachainApi } = useParachainApi("picasso");
  const isPoolConfigLoaded = useStore((store) => store.pools.isLoaded);
  const { poolId } = usePoolDetail();

  useEffect(() => {
    if (parachainApi) {
      return subscribePools(parachainApi);
    }
  }, [parachainApi]);
  useEffect(() => {
    subscribePoolAmount(parachainApi);
  }, [parachainApi]);

  return pipe(
    isPoolConfigLoaded,
    option.fromPredicate((a) => a),
    option.fold(
      () => <PoolSelectSkeleton />,
      () => {
        return pipe(
          getPoolById(poolId),
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
                    <PoolDetails poolId={poolId} />
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
