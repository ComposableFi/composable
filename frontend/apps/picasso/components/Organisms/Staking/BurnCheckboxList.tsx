import {
  alpha,
  Box,
  Button,
  Checkbox,
  Collapse,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import { AlertBox, TokenAsset } from "@/components";
import { WarningAmberRounded } from "@mui/icons-material";
import { FC, useMemo } from "react";
import { useStakingRewards } from "@/defi/polkadot/hooks/stakingRewards/useStakingRewards";
import { PortfolioItem } from "@/stores/defi/polkadot/stakingRewards/slice";
import { useExpiredPortfolio } from "@/components/Organisms/Staking/useExpiredPortfolio";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";
import { getPicassoTokenById } from "@/stores/defi/polkadot/tokens/utils";
import { StakeRemainingRelativeDate } from "@/components/Organisms/Staking/StakeRemainingRelativeDate";
import { TokenWithUSD } from "@/components/Organisms/Staking/TokenWithUSD";
import { useStore } from "@/stores/root";
import { getFnftKey } from "@/defi/polkadot/pallets/StakingRewards";

const BurnCheckboxItem = ({
  portfolio,
  selectedToken,
  onSelectUnstakeToken,
}: {
  portfolio: PortfolioItem;
  selectedToken: [string, string];
  onSelectUnstakeToken: (collectionId: string, instanceId: string) => void;
}) => {
  const theme = useTheme();
  const token = useMemo(() => {
    return getPicassoTokenById(portfolio.shareAssetId);
  }, [portfolio.shareAssetId]);
  const picaToken = useStore((store) => store.substrateTokens.tokens.pica);
  const picaPrice = usePicaPriceDiscovery();
  const stakedPrice = useMemo(() => {
    if (picaPrice.gte(0)) {
      return portfolio.stake.multipliedBy(picaPrice).toFormat(2);
    }

    return "";
  }, [picaPrice, portfolio.stake]);

  if (!token) return null;
  const isChecked =
    selectedToken[0] == portfolio.collectionId &&
    selectedToken[1] == portfolio.instanceId;
  const label = `${token?.symbol} ${portfolio.instanceId}`;
  return (
    <div key={portfolio.id}>
      <Button
        variant="outlined"
        fullWidth
        onClick={() => {
          onSelectUnstakeToken(portfolio.collectionId, portfolio.instanceId);
        }}
        sx={{
          padding: theme.spacing(1.5, 2),
          height: "auto",
          backgroundColor: isChecked
            ? alpha(theme.palette.primary.light, 0.04)
            : "inherit",
          borderColor: isChecked
            ? theme.palette.primary.main
            : alpha(theme.palette.common.white, 0.3),
        }}
      >
        <Stack
          width="100%"
          justifyContent="space-between"
          alignItems="center"
          direction="row"
        >
          <Stack direction="row" gap={1}>
            <Checkbox checked={isChecked} />
            <TokenAsset tokenId={token.id} label={label} />
          </Stack>
          <Stack direction="row" gap={2} alignItems="center">
            <TokenWithUSD
              symbol={picaToken.symbol}
              amount={portfolio.stake.toFormat()}
              price={stakedPrice}
            />
            <StakeRemainingRelativeDate portfolio={portfolio} />
          </Stack>
        </Stack>
      </Button>
    </div>
  );
};

export const BurnCheckboxList: FC<{
  openBurnModal: () => void;
  onSelectUnstakeToken: (
    fnftCollectionId: string,
    fnftInstanceId: string
  ) => void;
  unstakeTokenId: [string, string];
}> = ({ openBurnModal, onSelectUnstakeToken, unstakeTokenId }) => {
  const { stakingPortfolio } = useStakingRewards();
  const [fnftCollectionId, fnftInstanceId] = unstakeTokenId;
  const isSelected = Boolean(fnftCollectionId) && Boolean(fnftInstanceId);
  const currentPortfolio = stakingPortfolio.get(
    getFnftKey(fnftCollectionId, fnftInstanceId)
  );
  const { isExpired } = useExpiredPortfolio(currentPortfolio);
  const shouldShowSlashWarning = isSelected && !isExpired;
  const shouldDisableButton = !fnftCollectionId || !isSelected;

  const penaltyPercent = `${currentPortfolio?.unlockPenalty.toString() ?? 0}%`;

  return (
    <Stack gap={4} marginTop={9}>
      {Array.from(stakingPortfolio.entries()).map(([key, portfolioItem]) => (
        <BurnCheckboxItem
          key={key}
          portfolio={portfolioItem}
          selectedToken={unstakeTokenId}
          onSelectUnstakeToken={onSelectUnstakeToken}
        />
      ))}
      <Collapse in={shouldShowSlashWarning}>
        <AlertBox
          status="warning"
          icon={<WarningAmberRounded color="warning" />}
        >
          <Typography variant="body2">
            {penaltyPercent} withdraw fee warning
          </Typography>
          <Typography variant="inputLabel" color="text.secondary">
            If you withdraw locked fNFTs you will pay a {penaltyPercent} fee on
            your initial deposit.
          </Typography>
        </AlertBox>
      </Collapse>
      <Box
        gap={2}
        sx={{
          display: "flex",
          direction: {
            sm: "column",
            md: "row",
          },
        }}
      >
        <Button
          variant="contained"
          color="primary"
          fullWidth
          disabled={shouldDisableButton}
          onClick={openBurnModal}
        >
          Burn and unstake
        </Button>
      </Box>
    </Stack>
  );
};
