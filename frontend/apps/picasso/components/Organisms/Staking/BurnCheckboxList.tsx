import {
  alpha,
  Box,
  Button,
  Checkbox,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import { AlertBox, TokenAsset } from "@/components";
import { WarningAmberRounded } from "@mui/icons-material";
import { FC, useMemo, useState } from "react";
import { useStakingRewards } from "@/defi/polkadot/hooks/stakingRewards/useStakingRewards";
import { PortfolioItem } from "@/stores/defi/polkadot/stakingRewards/slice";
import { useExpiredPortfolio } from "@/components/Organisms/Staking/useExpiredPortfolio";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";
import { getPicassoTokenById } from "@/stores/defi/polkadot/tokens/utils";
import { StakeRemainingRelativeDate } from "@/components/Organisms/Staking/StakeRemainingRelativeDate";
import { TokenWithUSD } from "@/components/Organisms/Staking/TokenWithUSD";

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
    return getPicassoTokenById(portfolio.collectionId);
  }, [portfolio.collectionId]);
  const picaPrice = usePicaPriceDiscovery();
  const stakedPrice = useMemo(() => {
    if (picaPrice.gte(0)) {
      const stakedPrice = portfolio.stake.multipliedBy(picaPrice);

      return `(~$${stakedPrice.toFormat(2)})`;
    }

    return "";
  }, [picaPrice, portfolio.stake]);

  if (!token) return null;
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
        }}
      >
        <Stack
          width="100%"
          justifyContent="space-between"
          alignItems="center"
          direction="row"
        >
          <Stack direction="row" gap={1}>
            <Checkbox
              checked={
                selectedToken[0] == portfolio.collectionId &&
                selectedToken[1] == portfolio.instanceId
              }
            />
            <TokenAsset tokenId={token.id} label={label} />
          </Stack>
          <Stack direction="row" gap={2} alignItems="center">
            <TokenWithUSD
              symbol={"$PICA"}
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
  const theme = useTheme();
  const { stakingPortfolio } = useStakingRewards();
  const isSelected =
    unstakeTokenId[0].length > 0 && unstakeTokenId[1].length > 0;
  const [fnftCollectionId, fnftInstanceId] = unstakeTokenId;
  const currentPortfolio = Object.values(stakingPortfolio).find(
    (portfolio) =>
      portfolio.collectionId === fnftCollectionId &&
      portfolio.instanceId === fnftInstanceId
  );
  const { isExpired } = useExpiredPortfolio(currentPortfolio);
  const [agreed, setAgreed] = useState(false);
  const shouldShowSlashWarning = isSelected && !isExpired;
  const shouldDisableButton =
    !fnftCollectionId ||
    !isSelected ||
    (shouldShowSlashWarning ? !agreed : false);

  return (
    <Stack gap={4} marginTop={9}>
      {stakingPortfolio.map((portfolioItem) => (
        <BurnCheckboxItem
          key={portfolioItem.id}
          portfolio={portfolioItem}
          selectedToken={unstakeTokenId}
          onSelectUnstakeToken={onSelectUnstakeToken}
        />
      ))}
      {shouldShowSlashWarning && (
        <>
          <AlertBox
            status="warning"
            icon={<WarningAmberRounded color="warning" />}
          >
            <Typography variant="body2">50% withdraw fee warning</Typography>
            <Typography variant="inputLabel" color="text.secondary">
              If you withdraw locked fNFTs you will pay a 50% fee on your
              initial deposit.
            </Typography>
          </AlertBox>
          <Stack
            direction="row"
            alignItems="center"
            sx={{
              p: theme.spacing(3),
              borderRadius: `${theme.shape.borderRadius}px `,
              backgroundColor: alpha(theme.palette.common.white, 0.02),
            }}
            gap={1}
          >
            <Checkbox checked={agreed} onChange={(_, v) => setAgreed(v)} />
            <Typography variant="body2">
              I understand I will pay a 50% withdraw fee
            </Typography>
          </Stack>
        </>
      )}
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
