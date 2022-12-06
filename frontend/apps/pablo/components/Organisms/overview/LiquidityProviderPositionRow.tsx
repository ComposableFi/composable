import {
  TableCell,
  TableRow,
  Typography,
} from "@mui/material";
import { PairAsset } from "@/components/Atoms";
import { useLpTokenPrice, useLpTokenUserBalance } from "@/defi/hooks";
import { PabloConstantProductPool } from "shared";
import BigNumber from "bignumber.js";

const LiquidityProviderPositionRow = ({
  pool,
}: {
  pool: PabloConstantProductPool
}) => {
  const lpTokenUserBalance = useLpTokenUserBalance(pool);
  const lpTokenPrice = useLpTokenPrice(pool.getLiquidityProviderToken());
  const apr = new BigNumber(0);

  return (
    <TableRow key={`${pool.getLiquidityProviderToken().getSymbol()}`}>
      <TableCell align="left">
        <PairAsset
          assets={pool.getLiquidityProviderToken().getUnderlyingAssetJSON()}
          separator="/"
        />
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">
          ${lpTokenPrice.toFormat(2)}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">{lpTokenUserBalance.toFormat(2)}</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">
          ${lpTokenPrice.times(lpTokenUserBalance).toFormat(2)}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">{apr.toFormat(2)}%</Typography>
      </TableCell>
    </TableRow>
  );
};

export default LiquidityProviderPositionRow;
