import { Box, Button, Checkbox, Stack, Typography, useTheme } from "@mui/material";
import { AlertBox, TokenAsset } from "@/components";
import { formatNumber, formatNumberWithSymbol, fromChainIdUnit } from "shared";
import { WarningAmberRounded } from "@mui/icons-material";
import { FC } from "react";
import { useStakingRewards } from "@/defi/polkadot/hooks/useStakingRewards";

export const BurnCheckboxList: FC<{
  openBurnModal: () => void;
  openRenewModal: () => void;
  onSelectUnstakeToken: (fnftCollectionId: string, fnftInstanceId: string) => void;
  unstakeTokenId: [string, string];
}> = ({
  openBurnModal,
  openRenewModal,
  onSelectUnstakeToken,
  unstakeTokenId
}) => {
  const { stakingPortfolio } = useStakingRewards();
  const theme = useTheme();
  return (
    <Stack gap={4} marginTop={9}>
      {stakingPortfolio.map((portfolioItem) => {
        const pair = `fNFT ${portfolioItem.instanceId}`;
        return (
          <div key={portfolioItem.id}>
            <Button
              variant="outlined"
              fullWidth
              onClick={() => {
                onSelectUnstakeToken(portfolioItem.collectionId, portfolioItem.instanceId);
              }}
              sx={{
                padding: theme.spacing(1.5, 2),
                height: "auto"
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
                    checked={unstakeTokenId[0] == portfolioItem.collectionId && unstakeTokenId[1] == portfolioItem.instanceId} />
                  <TokenAsset tokenId={"pica"} label={pair} />
                </Stack>
                <Stack direction="row" gap={1}>
                  <Typography variant="body2">
                    {formatNumber(fromChainIdUnit(portfolioItem.stake))} (
                    {formatNumberWithSymbol(0, "$")})
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
            md: "row"
          }
        }}
      >
        <Button
          variant="contained"
          color="primary"
          fullWidth
          onClick={openRenewModal}
        >
          Renew
        </Button>
        <Button
          variant="outlined"
          color="primary"
          fullWidth
          onClick={openBurnModal}
        >
          Burn and unstake
        </Button>
      </Box>
    </Stack>
  );
};
