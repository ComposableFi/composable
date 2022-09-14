import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
  BoxProps,
  Button,
} from "@mui/material";
import { BaseAsset } from "@/components/Atoms";
import { useAppSelector } from "@/hooks/store";
import React, { useMemo } from "react";
import { TableHeader } from "@/defi/types";
import { BoxWrapper } from "./BoxWrapper";
import { useStakedPositions } from "@/store/stakingRewards/stakingRewards.slice";
import { useOwnedFinancialNfts } from "@/store/financialNfts/financialNfts.slice";
import { DEFAULT_UI_FORMAT_DECIMALS, fromChainUnits, PBLO_ASSET_ID } from "@/defi/utils";
import { useAsset } from "@/defi/hooks";
import moment from "moment";

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
  const xPablos = useAppSelector((state) => state.polkadot.yourXPablos);
  const expired = (expiry: number) => expiry < new Date().getTime();

  const myStakingPositions = useStakedPositions(PBLO_ASSET_ID);
  const myFinancialNfts = useOwnedFinancialNfts();

  const _xPablos = useMemo(() => {
    if (financialNftCollectionId === "-") return [];

    return myStakingPositions.filter((x) => {
      return (
        x.fnftCollectionId === financialNftCollectionId &&
        x.fnftCollectionId in myFinancialNfts &&
        myFinancialNfts[x.fnftCollectionId].includes(x.fnftInstanceId)
      );
    });
  }, [myStakingPositions, myFinancialNfts, financialNftCollectionId]);

  const xPablo = useAsset(PBLO_ASSET_ID);

  return (
    <BoxWrapper title={title || "Your xPBLO"} {...boxProps}>
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
              (
                { 
                  fnftInstanceId,
                  endTimestamp,
                  amount,
                  }
              ) => (
                <TableRow key={fnftInstanceId}>
                  <TableCell align="left">
                    <BaseAsset
                      icon={xPablo?.icon}
                      label={xPablo?.symbol}
                    />
                  </TableCell>
                  <TableCell align="left">
                    <Typography variant="body1">
                      {fromChainUnits(amount).toFixed(DEFAULT_UI_FORMAT_DECIMALS)}
                    </Typography>
                  </TableCell>
                  <TableCell align="left">
                  <Typography
                    variant="body1"
                    color={expired(+endTimestamp) ? "error" : undefined}
                  >
                    {expired(+endTimestamp)
                      ? "Expired"
                      : moment(+endTimestamp).utc().format("DD MMM YYYY")
                    }
                  </Typography>
                </TableCell>
                  <TableCell align="left">
                    <Typography variant="body1">
                      {"0"} 
                      {/* Multiplier */}
                    </Typography>
                  </TableCell>
                  <TableCell align="left">
                    <Typography variant="body1">
                      {"0"}
                      {/* xPablo amount */}
                    </Typography>
                  </TableCell>
                </TableRow>
              )
            )}
          </TableBody>
        </Table>
      </TableContainer>
    </BoxWrapper>
  );
};
