import {
  TableCell,
  TableRow,
  Typography,
} from "@mui/material";
import { PairAsset } from "@/components/Atoms";
import { useAsset } from "@/defi/hooks";
import { useUSDPriceByAssetId } from "@/store/assets/hooks";
import { ConstantProductPoolWithLpBalance, StableSwapPoolWithLpBalance } from "@/store/hooks/overview/usePoolsWithLpBalance";
import BigNumber from "bignumber.js";

const LiquidityProviderPositionRow = ({
  pool,
}: {
  pool: StableSwapPoolWithLpBalance | ConstantProductPoolWithLpBalance
}) => {
  const baseAsset = useAsset(pool.pair.base.toString());
  const quoteAsset = useAsset(pool.pair.quote.toString());
  const lpPrice = useUSDPriceByAssetId(pool.lpToken);
  const apr = new BigNumber(0);

  return (
    <TableRow key={`${baseAsset?.symbol}-${quoteAsset?.symbol}`}>
      <TableCell align="left">
        {baseAsset && quoteAsset && (
          <PairAsset
            assets={[
              { icon: baseAsset.icon, label: baseAsset.symbol },
              { icon: quoteAsset.icon, label: quoteAsset.symbol },
            ]}
            separator="/"
          />
        )}
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">
          ${lpPrice ? lpPrice.toFormat(2) : " - "}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">{pool.lpBalance.toFormat(2)}</Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">
          ${lpPrice.times(pool.lpBalance).toFormat(2)}
        </Typography>
      </TableCell>
      <TableCell align="left">
        <Typography variant="body1">{apr.toFormat(2)}%</Typography>
      </TableCell>
    </TableRow>
  );
};

export default LiquidityProviderPositionRow;
