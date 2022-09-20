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
import { FC, useCallback, useEffect } from "react";
import { useStakingRewards } from "@/defi/polkadot/hooks/useStakingRewards";
import { subscribeAssetPrice } from "@/defi/polkadot/pallets/Oracle";
import { useStore } from "@/stores/root";
import BigNumber from "bignumber.js";

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
  const theme = useTheme();
  const subscribePriceCallback = useCallback(
    (assetId: string) =>
      callbackGate(
        (api, id) => subscribeAssetPrice(api.createType("CurrencyId", id), api),
        parachainApi,
        assetId
      ),
    [parachainApi]
  );
  const prices = useStore((state) => state.oracle.prices);
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
  }, [stakingPortfolio]);
  return (
    <Stack gap={4} marginTop={9}>
      {stakingPortfolio.map((portfolioItem) => {
        const pair = `fNFT ${portfolioItem.instanceId}`;
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
                      unstakeTokenId[0] == portfolioItem.collectionId &&
                      unstakeTokenId[1] == portfolioItem.instanceId
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
      })}
      <AlertBox status="warning" icon={<WarningAmberRounded color="warning" />}>
        <Typography variant="body2">Slash warning</Typography>
        <Typography variant="inputLabel" color="text.secondary">
          If you withdraw now you will get rekt with less PICA.
        </Typography>
      </AlertBox>
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
