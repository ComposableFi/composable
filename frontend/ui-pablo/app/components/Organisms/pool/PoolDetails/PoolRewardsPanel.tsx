import { BaseAsset } from "@/components/Atoms";
import { TOKENS } from "@/defi/Tokens";
import { useAppSelector } from "@/hooks/store";
import { getTokenIdsFromSelectedPool } from "@/stores/defi/pool";
import {
  alpha,
  Box,
  BoxProps,
  Button,
  Divider,
  Grid,
  GridProps,
  Typography,
  useTheme,
} from "@mui/material";
import { PoolDetailsProps } from ".";
import { BoxWrapper } from "../../BoxWrapper";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

type ItemProps = {
  value: string,
  intro?: string,
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
          <Typography variant="subtitle1" color="text.secondary" component="span" mr={2}>
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
  const {
    tokenId1,
    tokenId2,
  } = useAppSelector(getTokenIdsFromSelectedPool);

  const {
    poolAmount,
    poolValue,
    rewardValue,
    rewardsLeft,
  } = useAppSelector((state) => state.pool.selectedPool);

  const {
    pooledAmount1,
    pooledAmount2,
  } = useAppSelector((state) => state.pool.currentLiquidity);

  const handleClaimRewards = () => {
    // TODO: handle claim rewards
  };

  return (
    <BoxWrapper {...boxProps}>
      <Item
        value={`$${poolValue.toFormat()}`}
        intro={
          `${poolAmount.toFormat()} ${TOKENS[tokenId1].symbol}/${TOKENS[tokenId2].symbol}`
        }
      >
        <Typography variant="h6">
          Your deposits
        </Typography>
      </Item>
      <Item value={pooledAmount1.toFormat()} mt={4.375}>
        <BaseAsset icon={TOKENS[tokenId1].icon} label={TOKENS[tokenId1].symbol} />
      </Item>
      <Item value={pooledAmount2.toFormat()} mt={2}>
        <BaseAsset icon={TOKENS[tokenId2].icon} label={TOKENS[tokenId2].symbol} />
      </Item>

      <Box mt={4}>
        <Divider
          sx={{
            borderColor: alpha(theme.palette.common.white, theme.custom.opacity.light),
          }} />
      </Box>

      <Item
        mt={4}
        mb={4}
        value={`$${rewardValue.toFormat()}`}
      >
        <Typography variant="h6">
          Your rewards
        </Typography>
      </Item>
      {rewardsLeft.map(({tokenId, value}) => (
        <Item value={value.toFormat()} mt={2} key={tokenId}>
          <BaseAsset icon={TOKENS[tokenId].icon} label={TOKENS[tokenId].symbol} />
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

