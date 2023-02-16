import {
  PortfolioItem,
  StakingPortfolio,
} from "@/stores/defi/polkadot/stakingRewards/slice";
import {
  Box,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
} from "@mui/material";
import { TokenAsset } from "@/components";
import { useMemo } from "react";
import { useExpiredPortfolio } from "@/components/Organisms/Staking/useExpiredPortfolio";
import { usePicaPriceDiscovery } from "@/defi/polkadot/hooks/usePicaPriceDiscovery";
import { getPicassoTokenById } from "@/stores/defi/polkadot/tokens/utils";

export const PortfolioRow = ({ portfolio }: { portfolio: PortfolioItem }) => {
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
            {portfolio.stake.toFormat(4)} ${rewardAsset.symbol}
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
    </TableRow>
  );
};

export const StakingPortfolioTable = ({
  stakingPortfolio,
}: {
  stakingPortfolio: StakingPortfolio;
}) => {
  return (
    <TableContainer component={Box}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>fNFTID</TableCell>
            <TableCell>Locked $PICA</TableCell>
            <TableCell>Locked until</TableCell>
            <TableCell>Multiplier</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {Array.from(stakingPortfolio.entries()).map(([key, portfolio]) => (
            <PortfolioRow key={key} portfolio={portfolio} />
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};
