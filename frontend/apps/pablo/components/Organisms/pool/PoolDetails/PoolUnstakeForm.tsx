import {
  Box,
  useTheme,
  Button,
  Typography,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableRow,
} from "@mui/material";
import { useMemo } from "react";
import { PoolDetailsProps } from "./index";
import { useLiquidityPoolDetails } from "@/store/hooks/useLiquidityPoolDetails";
import { useXTokensList } from "@/defi/hooks/financialNfts";
import { useStakingRewardPoolCollectionId } from "@/store/stakingRewards/stakingRewards.slice";
import { useUnstake } from "@/defi/hooks/stakingRewards";
import { PairAsset } from "@/components/Atoms";
import { DEFAULT_NETWORK_ID, DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { ConfirmingModal } from "../../swap/ConfirmingModal";
import BigNumber from "bignumber.js";

export const PoolUnstakeForm: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  const theme = useTheme();
  
  const { baseAsset, quoteAsset, pool } = useLiquidityPoolDetails(poolId);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const positions = useXTokensList({ stakedAssetId: pool?.lpToken });
  const collectionId = useStakingRewardPoolCollectionId(pool?.lpToken ?? "-");

  const financialNftCollectionId = useMemo(() => {
    if (!collectionId) return undefined;
    return new BigNumber(collectionId);
  }, [collectionId]);

  const financialNftInstanceId = useMemo(() => {
    if (positions.length > 0) return new BigNumber(positions[0].nftId);
    return undefined;
  }, [positions]);

  const principalAssetSymbol = useMemo(() => {
    if (!baseAsset || !quoteAsset) return undefined;
    return `${baseAsset.symbol}/${quoteAsset.symbol}`;
  }, [baseAsset, quoteAsset]);

  const hasStakedPositions = useMemo(() => {
    return positions.length > 0;
  }, [positions]);

  const handleUnStake = useUnstake({
    financialNftCollectionId,
    financialNftInstanceId,
  });

  const isUnstaking = usePendingExtrinsic(
    "unstake",
    "stakingRewards",
    selectedAccount ? selectedAccount.address : "-"
  );

  return (
    <Box {...boxProps}>
      {!hasStakedPositions && (
        <Box>
          <Typography
            variant="subtitle1"
            color="text.primary"
            textAlign={"center"}
          >
            No LP Staked.
          </Typography>
          <Typography
            mt={2}
            variant="body2"
            color="text.secondary"
            textAlign={"center"}
          >
            You don&apos;t currently have any ${principalAssetSymbol} positions
            staked.
          </Typography>
        </Box>
      )}

      {positions.length > 0 ? (
        <TableContainer>
          <Table>
            <TableBody>
              {positions.map(
                ({
                  lockedPrincipalAsset,
                  nftId,
                  expiryDate,
                  isExpired,
                }) => (
                  <TableRow key={nftId}>
                    <TableCell align="left">
                      <PairAsset
                        assets={
                          baseAsset && quoteAsset
                            ? [
                                {
                                  label: baseAsset.symbol,
                                  icon: baseAsset.icon,
                                },
                                {
                                  label: quoteAsset.symbol,
                                  icon: quoteAsset.icon,
                                },
                              ]
                            : []
                        }
                        separator="/"
                      />
                    </TableCell>
                    <TableCell align="center">
                      <Typography variant="body1">
                        {lockedPrincipalAsset.toFixed(
                          DEFAULT_UI_FORMAT_DECIMALS
                        )}
                      </Typography>
                    </TableCell>
                    <TableCell align="right">
                      <Typography
                        variant="body1"
                        color={isExpired ? "error" : undefined}
                      >
                        {isExpired ? "Expired" : expiryDate}
                      </Typography>
                    </TableCell>
                  </TableRow>
                )
              )}
            </TableBody>
          </Table>
        </TableContainer>
      ) : null}

      <Box mt={4}>
        <Button
          variant="contained"
          size="large"
          fullWidth
          onClick={handleUnStake}
          disabled={!hasStakedPositions}
        >
          {`Unstake ${principalAssetSymbol}`}
        </Button>
      </Box>

      <ConfirmingModal open={isUnstaking} />
    </Box>
  );
};
