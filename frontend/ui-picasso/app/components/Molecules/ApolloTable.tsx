import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from "@mui/material";
import { TokenAsset } from "@/components/Atom";
import { formatNumberWithSymbol, formatNumber } from "@/utils/formatters";

type ApolloTableProps = {
  assets: Array<AssetProps>;
};

type AssetProps = {
  symbol: string;
  binanceValue: number | undefined;
  apolloValue: number | undefined;
  changeValue: number | undefined;
};

const tableHeaderTitles = ["Asset", "Binance", "Apollo", "Change (24hr)"];

export const ApolloTable: React.FC<ApolloTableProps> = ({
  assets,
  ...rest
}) => {
  return (
    <TableContainer {...rest}>
      <Table sx={{ minWidth: 420 }} aria-label="apollo table">
        <TableHead>
          <TableRow>
            {tableHeaderTitles.map((title) => (
              <TableCell key={title} align="left">
                {title}
              </TableCell>
            ))}
          </TableRow>
        </TableHead>
        <TableBody>
          {assets.map((asset: AssetProps) => {
            return (
              <TableRow key={asset.symbol}>
                <TableCell align="left">
                  <TokenAsset tokenId={asset.symbol.toLowerCase()} />
                </TableCell>
                <TableCell align="left">
                  {asset.binanceValue
                    ? "$" +
                      formatNumber(parseFloat(asset.binanceValue).toFixed(2))
                    : "-"}
                </TableCell>
                <TableCell align="left">
                  {asset.apolloValue
                    ? "$" + formatNumber(asset.apolloValue.toFixed(2))
                    : "-"}
                </TableCell>
                <TableCell
                  align="left"
                  sx={{
                    color:
                      asset.changeValue && asset.changeValue > 0
                        ? "featured.lemon"
                        : "error.main",
                  }}
                >
                  {asset.changeValue
                    ? formatNumberWithSymbol(
                        asset.changeValue,
                        asset.changeValue > 0 ? "+" : "",
                        "%"
                      )
                    : "-"}
                </TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </TableContainer>
  );
};
