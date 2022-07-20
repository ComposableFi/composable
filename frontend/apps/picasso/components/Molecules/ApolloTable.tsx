import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow
} from "@mui/material";
import { TokenAsset } from "@/components/Atom";
import { formatNumberWithSymbol, formatNumber } from "shared";

type ApolloTableProps = {
  assets: Array<AssetProps>;
};

type AssetProps = {
  symbol: string;
  binanceValue: number;
  pabloValue: number;
  aggregatedValue: number;
  apolloValue: number;
  changeValue: number;
};

const tableHeaderTitles = [
  "Asset",
  "Binance",
  "Pablo",
  "Aggregated",
  "Apollo",
  "Change (24hr)"
];

export const ApolloTable: React.FC<ApolloTableProps> = ({
  assets,
  ...rest
}) => {
  return (
    <TableContainer {...rest}>
      <Table sx={{ minWidth: 420 }} aria-label="apollo table">
        <TableHead>
          <TableRow>
            {tableHeaderTitles.map(title => (
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
                  ${formatNumber(asset.binanceValue)}
                </TableCell>
                <TableCell align="left">
                  ${formatNumber(asset.pabloValue)}
                </TableCell>
                <TableCell align="left">
                  ${formatNumber(asset.aggregatedValue)}
                </TableCell>
                <TableCell align="left">
                  ${formatNumber(asset.apolloValue)}
                </TableCell>
                <TableCell
                  align="left"
                  sx={{
                    color:
                      asset.changeValue > 0 ? "featured.lemon" : "error.main"
                  }}
                >
                  {formatNumberWithSymbol(
                    asset.changeValue,
                    asset.changeValue > 0 ? "+" : "",
                    "%"
                  )}
                </TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </TableContainer>
  );
};
