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
import { useLiquidityPoolDetails } from "@/defi/hooks/useLiquidityPoolDetails";
import { useXTokensList } from "@/defi/hooks/financialNfts";
import { useStakingRewardPoolCollectionId } from "@/store/stakingRewards/stakingRewards.slice";
import { useUnstake } from "@/defi/hooks/stakingRewards";
import { PairAsset } from "@/components/Atoms";
import { DEFAULT_NETWORK_ID, DEFAULT_UI_FORMAT_DECIMALS } from "@/defi/utils";
import { usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { ConfirmingModal } from "../../swap/ConfirmingModal";
import { useLpTokenPrice } from "@/defi/hooks";
import BigNumber from "bignumber.js";

const UnstakeFormPosition: React.FC<{
  financialNftId: string;
  principalAssets: Array<{ label?: string; icon: string }>;
  principalAssetValue: BigNumber;
  principalAssetStakedAmount: BigNumber;
  isExpired: boolean;
  expiryDate: string;
}> = ({
  financialNftId,
  principalAssets,
  principalAssetValue,
  principalAssetStakedAmount,
  isExpired,
  expiryDate,
}) => {
  const formattedStakedAmount = principalAssetStakedAmount.toFixed(
    DEFAULT_UI_FORMAT_DECIMALS
  );
  const formattedStakedAmountValue = principalAssetStakedAmount
    .times(principalAssetValue)
    .toFixed(DEFAULT_UI_FORMAT_DECIMALS);
  const stakedAmountInStr = `${formattedStakedAmount} (~$${formattedStakedAmountValue})`;

  return (
    <TableRow key={financialNftId}>
      <TableCell align="left">
        <PairAsset assets={principalAssets} separator="/" />
      </TableCell>
      <TableCell align="center">
        <Typography variant="body1">{stakedAmountInStr}</Typography>
      </TableCell>
      <TableCell align="right">
        <Typography variant="body1" color={isExpired ? "error" : undefined}>
          {isExpired ? "Expired" : expiryDate}
        </Typography>
      </TableCell>
    </TableRow>
  );
};

export const PoolUnstakeForm: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  const theme = useTheme();
  const { pool } = useLiquidityPoolDetails(poolId);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const lpToken = pool?.getLiquidityProviderToken() ?? null;
  const lpAssetId = lpToken?.getPicassoAssetId() as string ?? "-"
  const positions = useXTokensList({ stakedAssetId: lpAssetId });
  const collectionId = useStakingRewardPoolCollectionId(lpAssetId);

  const financialNftCollectionId = useMemo(() => {
    if (!collectionId) return undefined;
    return new BigNumber(collectionId);
  }, [collectionId]);

  const financialNftInstanceId = useMemo(() => {
    if (positions.length > 0) return new BigNumber(positions[0].nftId);
    return undefined;
  }, [positions]);

  const lpTokenPrice = useLpTokenPrice(lpToken ?? undefined);

  const pairAssets = useMemo(() => {
    if (!lpToken) return [];

    return lpToken.getUnderlyingAssetJSON();
  }, [lpToken]);

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
            You don&apos;t currently have any {lpToken?.getSymbol()} positions
            staked.
          </Typography>
        </Box>
      )}

      {positions.length > 0 ? (
        <TableContainer>
          <Table>
            <TableBody>
              {positions.map(
                ({ lockedPrincipalAsset, nftId, expiryDate, isExpired }) => {
                  return (
                    <UnstakeFormPosition
                      principalAssets={pairAssets}
                      financialNftId={nftId}
                      principalAssetValue={lpTokenPrice}
                      principalAssetStakedAmount={lockedPrincipalAsset}
                      isExpired={isExpired}
                      expiryDate={expiryDate}
                    />
                  );
                }
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
          {`Unstake ${lpToken?.getSymbol()}`}
        </Button>
      </Box>

      <ConfirmingModal open={isUnstaking} />
    </Box>
  );
};
