import { alpha, Box, Button, Grid, Typography, useTheme } from "@mui/material";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { Link } from "@/components";
import { useRouter } from "next/router";
import { useAllLpTokenRewardingPools } from "@/defi/hooks";
import {
  useDotSamaContext,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useEffect } from "react";
import { subscribeOwnedLiquidity } from "@/store/pools/subscribeOwnedLiquidity";
import useStore from "@/store/useStore";
import { YourLiquidityTable } from "@/components/Organisms/pool/YourLiquidityTable";

export const YourLiquidity = () => {
  const handleClick = () => {
    router.push("/pool/add-liquidity");
  };

  const handleCreatePair = () => {
    router.push("/pool/create-pool");
  };
  const theme = useTheme();
  const router = useRouter();
  const allLpRewardingPools = useAllLpTokenRewardingPools();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { extensionStatus } = useDotSamaContext();
  const userOwnedLiquidity = useStore((store) => store.ownedLiquidity.tokens);

  useEffect(() => {
    let unsub: any = undefined;
    if (parachainApi && selectedAccount && extensionStatus === "connected") {
      unsub = subscribeOwnedLiquidity(parachainApi, selectedAccount.address);
    }

    return () => {
      unsub?.();
    };
  }, [extensionStatus, selectedAccount, parachainApi]);

  return (
    <Grid>
      <Grid item xs={12}>
        <HighlightBox>
          <Box
            display="flex"
            mb={3}
            justifyContent="space-between"
            alignItems="center"
          >
            <Typography variant="h6">Your Liquidity</Typography>
            <Box>
              <Button
                disabled
                sx={{ marginRight: 2 }}
                onClick={handleCreatePair}
                variant="outlined"
              >
                Create a pair
              </Button>
              <Button
                disabled={allLpRewardingPools.length === 0}
                onClick={handleClick}
                variant="contained"
              >
                Add Liquidity
              </Button>
            </Box>
          </Box>
          <YourLiquidityTable tokens={userOwnedLiquidity} />
        </HighlightBox>
        <Box mt={4} display="none" gap={1} justifyContent="center">
          <Typography
            textAlign="center"
            variant="body2"
            color={alpha(
              theme.palette.common.white,
              theme.custom.opacity.darker
            )}
          >
            {`Don't see a pool you joined?`}
          </Typography>
          <Link href="/pool/import" key="import">
            <Typography
              textAlign="center"
              variant="body2"
              color="primary"
              sx={{ cursor: "pointer" }}
            >
              Import it.
            </Typography>
          </Link>
        </Box>
      </Grid>
    </Grid>
  );
};
