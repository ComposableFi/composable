import {
  Box,
  Button,
  Checkbox,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import { AlertBox, TokenAsset } from "@/components";
import { humanBalance } from "shared";
import { WarningAmberRounded } from "@mui/icons-material";
import { FC, useMemo } from "react";
import { useStakingRewards } from "@/defi/polkadot/hooks/stakingRewards/useStakingRewards";
import { PortfolioItem } from "@/stores/defi/polkadot/stakingRewards/slice";
import { useExpiredPortfolio } from "@/components/Organisms/Staking/useExpiredPortfolio";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";
import { getPicassoTokenById } from "@/stores/defi/polkadot/tokens/utils";

const BurnCheckboxItem = ({
  portfolioItem,
  selectedToken,
  onSelectUnstakeToken,
}: {
  portfolioItem: PortfolioItem;
  selectedToken: [string, string];
  onSelectUnstakeToken: (collectionId: string, instanceId: string) => void;
}) => {
  const theme = useTheme();
  const token = useMemo(() => {
    return getPicassoTokenById(portfolioItem.collectionId);
  }, [portfolioItem.collectionId]);
  const picaPrice = usePicaPriceDiscovery();
  const fnftPrice = useMemo(() => {
    return picaPrice.multipliedBy(portfolioItem.stake);
  }, [picaPrice, portfolioItem.stake]);

  if (!token) return null;
  const label = `${token?.symbol} ${portfolioItem.instanceId}`;
  return (
    <div key={portfolioItem.id}>
      <Button
        variant="outlined"
        fullWidth
        onClick={() => {
          onSelectUnstakeToken(
            portfolioItem.collectionId,
            portfolioItem.instanceId
          );
        }}
        sx={{
          padding: theme.spacing(1.5, 2),
          height: "auto",
        }}
      >
        <Box
          width="100%"
          display="flex"
          justifyContent="space-between"
          alignItems="center"
        >
          <Stack direction="row" gap={1}>
            <Checkbox
              checked={
                selectedToken[0] == portfolioItem.collectionId &&
                selectedToken[1] == portfolioItem.instanceId
              }
            />
            <TokenAsset tokenId={token.id} label={label} />
          </Stack>
          <Stack direction="row" gap={1}>
            <Typography variant="body2">
              {portfolioItem.stake.toFixed()} (${humanBalance(fnftPrice)})
            </Typography>
          </Stack>
        </Box>
      </Button>
    </div>
  );
};

export const BurnCheckboxList: FC<{
  openBurnModal: () => void;
  openRenewModal: () => void;
  onSelectUnstakeToken: (
    fnftCollectionId: string,
    fnftInstanceId: string
  ) => void;
  unstakeTokenId: [string, string];
}> = ({
  openBurnModal,
  openRenewModal,
  onSelectUnstakeToken,
  unstakeTokenId,
}) => {
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

  const shouldShowSlashWarning = isSelected && !isExpired;

  return (
    <Stack gap={4} marginTop={9}>
      {stakingPortfolio.map((portfolioItem) => (
        <BurnCheckboxItem
          key={portfolioItem.id}
          portfolioItem={portfolioItem}
          selectedToken={unstakeTokenId}
          onSelectUnstakeToken={onSelectUnstakeToken}
        />
      ))}
      {shouldShowSlashWarning && (
        <AlertBox
          status="warning"
          icon={<WarningAmberRounded color="warning" />}
        >
          <Typography variant="body2">Slash warning</Typography>
          <Typography variant="inputLabel" color="text.secondary">
            If you withdraw now you will get rekt with less PICA.
          </Typography>
        </AlertBox>
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
          disabled={!unstakeTokenId[0]}
          onClick={openRenewModal}
        >
          Add stake to period
        </Button>
        <Button
          variant="outlined"
          color="primary"
          fullWidth
          disabled={!unstakeTokenId[0]}
          onClick={openBurnModal}
        >
          Burn and unstake
        </Button>
      </Box>
    </Stack>
  );
};
