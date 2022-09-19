import { PortfolioItem, StakingPortfolio as TStakingPortfolio } from "@/stores/defi/polkadot/stakingRewards/slice";
import { Box, Button, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, useTheme } from "@mui/material";
import { TokenAsset } from "@/components";
import { formatDate, fromChainIdUnit, humanBalance } from "shared";
import { Add } from "@mui/icons-material";

export const PortfolioRow = ({
  portfolio
}: {
  portfolio: PortfolioItem;
}) => {
  const theme = useTheme();
  return (
    <TableRow>
      <TableCell>
        <TokenAsset tokenId={"pica"} label={`fNFT ${portfolio.instanceId}`} />
      </TableCell>
      <TableCell>{humanBalance(fromChainIdUnit(portfolio.stake))}</TableCell>
      <TableCell>
        {formatDate(new Date(Number(portfolio.endTimestamp.toString())))}
      </TableCell>
      <TableCell>{`${portfolio.multiplier.toFixed(
        2)}%`}</TableCell>
      <TableCell>{portfolio.share.toFixed()}</TableCell>
      <TableCell>
        <Button
          variant="outlined"
          size="small"
          sx={{
            minWidth: theme.spacing(6)
          }}
          onClick={() => console.log(
            "add stake ",
            portfolio.collectionId,
            portfolio.instanceId
          )}
        >
          <Add />
        </Button>
      </TableCell>
    </TableRow>
  );
};
export const StakingPortfolioTable = ({
  stakingPortfolio
}:
  {
    stakingPortfolio: TStakingPortfolio,
  }) => {
  return (
    <TableContainer component={Box}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>fNFT ID</TableCell>
            <TableCell>Locked PICA</TableCell>
            <TableCell>Expiry Date</TableCell>
            <TableCell>Multiplier</TableCell>
            <TableCell>Your xPICA</TableCell>
            <TableCell></TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {stakingPortfolio.map((portfolio, key) => (
            <PortfolioRow
              key={key}
              portfolio={portfolio}
            />
          ))}

        </TableBody>
      </Table>
    </TableContainer>
  );
};
