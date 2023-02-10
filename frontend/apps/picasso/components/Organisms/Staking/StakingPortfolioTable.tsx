import { PortfolioItem } from "@/stores/defi/polkadot/stakingRewards/slice";
import {
  Box,
  Button,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
  useTheme,
} from "@mui/material";
import { TokenAsset } from "@/components";
import { Add } from "@mui/icons-material";
import { useMemo, useState } from "react";
import { RenewModal } from "@/components/Organisms/Staking/RenewModal";
import { useExpiredPortfolio } from "@/components/Organisms/Staking/useExpiredPortfolio";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";
import { getPicassoTokenById } from "@/stores/defi/polkadot/tokens/utils";

export const PortfolioRow = ({
  portfolio,
  onSelectToken,
}: {
  portfolio: PortfolioItem;
  onSelectToken: (collectionId: string, instanceId: string) => void;
}) => {
  const theme = useTheme();
  const { isExpired, expiredDate } = useExpiredPortfolio(portfolio);
  const picaPrice = usePicaPriceDiscovery();
  const stakedPrice = useMemo(() => {
    if (picaPrice.gte(0)) {
      const stakedPrice = portfolio.stake.multipliedBy(picaPrice);

      return `(~$${stakedPrice.toFormat(2)})`;
    }

    return "";
  }, [picaPrice, portfolio.stake]);
  const shareAsset = getPicassoTokenById(portfolio.shareAssetId);
  const rewardAsset = getPicassoTokenById(portfolio.assetId);

  if (!shareAsset || !rewardAsset) {
    return null;
  }

  return (
    <TableRow>
      <TableCell>
        <TokenAsset
          tokenId={shareAsset.id}
          label={`${shareAsset.symbol} ${portfolio.instanceId}`}
        />
      </TableCell>
      <TableCell size="medium">
        <Box display="flex" gap={1}>
          <Typography variant="body2" color="text.primary">
            {portfolio.stake.toFormat()} ${rewardAsset.symbol}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            {stakedPrice}
          </Typography>
        </Box>
      </TableCell>
      <TableCell>{expiredDate}</TableCell>
      <TableCell>
        <Typography
          variant="body2"
          color={isExpired ? "warning.main" : "success.main"}
        >
          {`${portfolio.multiplier.div(100).toFixed(1)}X`}
        </Typography>
      </TableCell>
      <TableCell>
        <Button
          variant="outlined"
          size="small"
          sx={{
            minWidth: theme.spacing(5),
            width: theme.spacing(5),
            height: theme.spacing(5),
            padding: 0,
          }}
          onClick={() => {
            onSelectToken(portfolio.collectionId, portfolio.instanceId);
          }}
        >
          <Add />
        </Button>
      </TableCell>
    </TableRow>
  );
};

export const StakingPortfolioTable = ({
  stakingPortfolio,
}: {
  stakingPortfolio: Array<PortfolioItem>;
}) => {
  const [selectedToken, setSelectedToken] = useState<[string, string]>([
    "",
    "",
  ]);
  const [isRenewModalOpen, setIsRenewModalOpen] = useState<boolean>(false);
  return (
    <>
      <TableContainer component={Box}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>fNFTID</TableCell>
              <TableCell>Locked $PICA</TableCell>
              <TableCell>Locked until</TableCell>
              <TableCell>Multiplier</TableCell>
              <TableCell></TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {stakingPortfolio.map((portfolio) => (
              <PortfolioRow
                key={`${portfolio.collectionId}-${portfolio.instanceId}`}
                portfolio={portfolio}
                onSelectToken={(collectionId, instanceId) => {
                  setSelectedToken([collectionId, instanceId]);
                  setIsRenewModalOpen(true);
                }}
              />
            ))}
          </TableBody>
        </Table>
      </TableContainer>
      <RenewModal
        open={isRenewModalOpen}
        selectedToken={selectedToken}
        onClose={() => setIsRenewModalOpen(false)}
      />
    </>
  );
};
