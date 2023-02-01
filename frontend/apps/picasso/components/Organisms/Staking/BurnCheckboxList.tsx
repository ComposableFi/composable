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
import { FC } from "react";
import { useStakingRewards } from "@/defi/polkadot/hooks/stakingRewards/useStakingRewards";
import { useStore } from "@/stores/root";
import BigNumber from "bignumber.js";
import { PortfolioItem } from "@/stores/defi/polkadot/stakingRewards/slice";
import { useExpiredPortfolio } from "@/components/Organisms/Staking/useExpiredPortfolio";

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
  const { stakingPortfolio } = useStakingRewards();
  // TODO: Price fetch for assets was used from oracle, needs to change to coingecko
  // const subscribePriceCallback = useCallback(
  //   (assetId: string) =>
  //     callbackGate(
  //       (api, id) => subscribeAssetPrice(api.createType("CurrencyId", id), api),
  //       parachainApi,
  //       assetId
  //     ),
  //   [parachainApi]
  // );
  //
  // useEffect(() => {
  //   const unsub = stakingPortfolio.map((portfolio) =>
  //     subscribePriceCallback(portfolio.assetId)
  //   );
  //
  //   return () => {
  //     unsub.forEach(
  //       (unsubscribeFunction) =>
  //         typeof unsubscribeFunction === "function" && unsubscribeFunction?.()
  //     );
  //   };
  //   // eslint-disable-next-line react-hooks/exhaustive-deps
  // }, [stakingPortfolio]);

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
