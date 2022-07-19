import { BaseAsset } from "@/components/Atoms";
import { TOKENS } from "@/defi/Tokens";
import { useAppSelector } from "@/hooks/store";
import { useLiquidityPoolDetails } from "@/store/hooks/useLiquidityPoolDetails";
import { useUserProvidedLiquidityByPool } from "@/store/hooks/useUserProvidedLiquidityByPool";
import {
  alpha,
  Box,
  Button,
  Divider,
  Grid,
  GridProps,
  Typography,
  useTheme,
} from "@mui/material";
import BigNumber from "bignumber.js";
import { PoolDetailsProps } from "./index";
import { BoxWrapper } from "../../BoxWrapper";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

type ItemProps = {
  value: string;
  intro?: string;
} & GridProps;

const Item: React.FC<ItemProps> = ({
  value,
  intro,
  children,
  ...gridProps
}) => {
  return (
    <Grid container {...gridProps}>
      <Grid item {...twoColumnPageSize}>
        {children}
      </Grid>
      <Grid item {...twoColumnPageSize} textAlign="right">
        {intro && (
          <Typography
            variant="subtitle1"
            color="text.secondary"
            component="span"
            mr={2}
          >
            {intro}
          </Typography>
        )}
        <Typography variant="subtitle1" fontWeight={600} component="span">
          {value}
        </Typography>
      </Grid>
    </Grid>
  );
};

export const PoolRewardsPanel: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  const theme = useTheme();

  const poolDetails = useLiquidityPoolDetails(poolId);
  const userProvidedLiquidity = useUserProvidedLiquidityByPool(poolId);

  const { rewardValue, rewardsLeft } = useAppSelector(
    (state) => state.pool.selectedPool
  );

  // WIP
  const lpDeposit = new BigNumber(0);

  const handleClaimRewards = () => {
    // TODO: handle claim rewards
  };

  return (
    <BoxWrapper {...boxProps}>
      <Item
        value={`$${lpDeposit}`}
        intro={`${lpDeposit} ${poolDetails.baseAsset?.symbol}/${
          poolDetails.quoteAsset?.symbol
        }`}
      >
        <Typography variant="h6">Your deposits</Typography>
      </Item>
      <Item
        value={userProvidedLiquidity.tokenAmounts.baseAmount.toFormat()}
        mt={4.375}
      >
        {poolDetails.baseAsset && (
          <BaseAsset
            icon={poolDetails.baseAsset.icon}
            label={poolDetails.baseAsset.symbol}
          />
        )}
      </Item>
      <Item
        value={userProvidedLiquidity.tokenAmounts.quoteAmount.toFormat()}
        mt={2}
      >
        {poolDetails.quoteAsset && (
          <BaseAsset
            icon={poolDetails.quoteAsset.icon}
            label={poolDetails.quoteAsset.symbol}
          />
        )}
      </Item>

      <Box mt={4}>
        <Divider
          sx={{
            borderColor: alpha(
              theme.palette.common.white,
              theme.custom.opacity.light
            ),
          }}
        />
      </Box>

      <Item mt={4} mb={4} value={`$${rewardValue.toFormat()}`}>
        <Typography variant="h6">Your rewards</Typography>
      </Item>
      {rewardsLeft.map(({ tokenId, value }: { tokenId: keyof typeof TOKENS, value: BigNumber}) => (
        <Item value={value.toFormat()} mt={2} key={tokenId}>
          <BaseAsset
            icon={TOKENS[tokenId].icon}
            label={TOKENS[tokenId].symbol}
          />
        </Item>
      ))}

      <Box mt={4}>
        <Button
          variant="contained"
          size="large"
          fullWidth
          onClick={handleClaimRewards}
        >
          Claim rewards
        </Button>
      </Box>
    </BoxWrapper>
  );
};
