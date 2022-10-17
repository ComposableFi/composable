import {
  Box,
  Button,
  Checkbox,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import { AlertBox, TokenAsset } from "@/components";
import { callbackGate, humanBalance } from "shared";
import { WarningAmberRounded } from "@mui/icons-material";
import { FC, useCallback, useEffect, useMemo } from "react";
import { useStakingRewards } from "@/defi/polkadot/hooks/useStakingRewards";
import { subscribeAssetPrice } from "@/defi/polkadot/pallets/Oracle";
import { useStore } from "@/stores/root";
import BigNumber from "bignumber.js";
import { PortfolioItem } from "@/stores/defi/polkadot/stakingRewards/slice";

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
  const pair = `fNFT ${portfolioItem.instanceId}`;
  const prices = useStore((state) => state.oracle.prices);
  const fnftPrice = new BigNumber(portfolioItem.stake).multipliedBy(
    new BigNumber(
      prices[Number(portfolioItem.assetId)]?.price.toString() ?? "0"
    )
  );

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
            <TokenAsset tokenId={"pica"} label={pair} />
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
  const { stakingPortfolio, parachainApi } = useStakingRewards();
  const subscribePriceCallback = useCallback(
    (assetId: string) =>
      callbackGate(
        (api, id) => subscribeAssetPrice(api.createType("CurrencyId", id), api),
        parachainApi,
        assetId
      ),
    [parachainApi]
  );

  useEffect(() => {
    const unsub = stakingPortfolio.map((portfolio) =>
      subscribePriceCallback(portfolio.assetId)
    );

    return () => {
      unsub.forEach(
        (unsubscribeFunction) =>
          typeof unsubscribeFunction === "function" && unsubscribeFunction?.()
      );
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [stakingPortfolio]);

  const isSelected =
    unstakeTokenId[0].length > 0 && unstakeTokenId[1].length > 0;

  const isExpired = useMemo(() => {
    const portfolioItem = stakingPortfolio.find(
      (portfolio) =>
        portfolio.collectionId === unstakeTokenId[0] &&
        portfolio.instanceId === unstakeTokenId[1]
    );
    if (portfolioItem && isSelected) {
      const endDate = new Date(Number(portfolioItem.endTimestamp.toString()));
      const now = new Date();

      return endDate.getTime() - now.getTime() < 0;
    }

    return false;
  }, [stakingPortfolio, isSelected, unstakeTokenId]);

  const shouldShowSlashWarning = isSelected && !isExpired;

  return (
    <Stack gap={4} marginTop={9}>
      {stakingPortfolio.map((portfolioItem) => (
        <BurnCheckboxItem
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
