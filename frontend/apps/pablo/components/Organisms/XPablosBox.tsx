import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
  BoxProps,
} from "@mui/material";
import { BaseAsset } from "@/components/Atoms";
import React from "react";
import { TableHeader } from "@/defi/types";
import { BoxWrapper } from "./BoxWrapper";
import {
  DEFAULT_UI_FORMAT_DECIMALS,
  PBLO_ASSET_ID,
} from "@/defi/utils";
import { useAsset } from "@/defi/hooks";
import { NoPositionsPlaceholder } from "./overview/NoPositionsPlaceholder";
import { OVERVIEW_ERRORS } from "./overview/errors";
import { useXTokensList } from "@/defi/hooks/financialNfts";

const tableHeaders: TableHeader[] = [
  {
    header: "fNFT ID",
  },
  {
    header: "PBLO locked",
  },
  {
    header: "Expiry",
  },
  {
    header: "Multiplier",
  },
  {
    header: "xPBLO",
  },
  {
    header: "", // kept empty for action column with no header
  },
];

export type XPablosBoxProps = {
  title?: string;
  header?: TableHeader[];
  financialNftCollectionId: string;
} & BoxProps;

export const XPablosBox: React.FC<XPablosBoxProps> = ({
  title,
  header,
  financialNftCollectionId,
  ...boxProps
}) => {
  const xPablo = useAsset(PBLO_ASSET_ID);
  const _xPablos = useXTokensList({
    stakedAssetId: PBLO_ASSET_ID,
  });

  return (
    <BoxWrapper title={title || "Your xPBLO"} {...boxProps}>
      {_xPablos.length <= 0 ? (
        <NoPositionsPlaceholder text={OVERVIEW_ERRORS.NO_XTOKENS} />
      ) : null}

      {_xPablos.length > 0 ? (
        <TableContainer>
          <Table>
            <TableHead>
              <TableRow>
                {(header || tableHeaders).map((th) => (
                  <TableCell key={th.header} align="left">
                    {th.header}
                  </TableCell>
                ))}
              </TableRow>
            </TableHead>
            <TableBody>
              {_xPablos.map(
                ({
                  lockedPrincipalAsset,
                  nftId,
                  expiryDate,
                  isExpired,
                  multiplier,
                  xTokenBalance,
                }) => (
                  <TableRow key={nftId}>
                    <TableCell align="left">
                      <BaseAsset icon={xPablo?.getIconUrl()} label={`X${xPablo?.getSymbol()} ${nftId}`} />
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body1">
                        {lockedPrincipalAsset.toFixed(
                          DEFAULT_UI_FORMAT_DECIMALS
                        )}
                      </Typography>
                    </TableCell>
                    <TableCell align="left">
                      <Typography
                        variant="body1"
                        color={isExpired ? "error" : undefined}
                      >
                        {isExpired ? "Expired" : expiryDate}
                      </Typography>
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body1">{multiplier}</Typography>
                    </TableCell>
                    <TableCell align="left">
                      <Typography variant="body1">
                        {xTokenBalance.toFixed(DEFAULT_UI_FORMAT_DECIMALS)}
                      </Typography>
                    </TableCell>
                  </TableRow>
                )
              )}
            </TableBody>
          </Table>
        </TableContainer>
      ) : null}
    </BoxWrapper>
  );
};
