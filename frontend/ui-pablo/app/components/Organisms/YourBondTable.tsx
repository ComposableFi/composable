import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Box,
  Typography,
  Tooltip,
} from "@mui/material";
import Image from "next/image";
import { BaseAsset, PairAsset } from "../Atoms";
import { useAppSelector } from "@/hooks/store";
import { useRouter } from "next/router";
import React from "react";
import { InfoOutlined } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";

const tableHeaders: TableHeader[] = [
  {
    header: "Asset",
  },
  {
    header: "Claimable",
    tooltip: "Claimable",
  },
  {
    header: "Pending",
    tooltip: "Pending",
  },
  {
    header: "Vesting time",
    tooltip: "Vesting time",
  },
];

export const YourBondTable: React.FC = () => {
  const pools = useAppSelector((state) => state.polkadot.yourBondPools);
  const router = useRouter();

  const handleRowClick = (e: React.MouseEvent) => {
    router.push("/bond/select");
  };

  if (pools.length == 0) {
    return (
      <Box textAlign="center" mt={3}>
        <Image
          src="/static/lemonade.png"
          css={{ mixBlendMode: "luminosity" }}
          width="96"
          height="96"
          alt="lemonade"
        />
        <Typography variant="body2" paddingTop={4} color="text.secondary">
          You currently do not have any active bonds.
        </Typography>
      </Box>
    );
  } else {
    return (
      <TableContainer>
        <Table>
          <TableHead>
            <TableRow>
              {tableHeaders.map((th) => (
                <TableCell align="left" key={th.header}>
                  <Box display="flex" alignItems="center" gap={1}>
                    {th.header}
                    {th.tooltip && (
                      <Tooltip arrow title={th.tooltip}>
                        <InfoOutlined color="primary" fontSize="small"/>
                      </Tooltip>
                    )}
                  </Box>
                </TableCell>
              ))}
            </TableRow>
          </TableHead>
          <TableBody>
            {pools.map((pool, index) => (
              <TableRow
                onClick={handleRowClick}
                key={index}
                sx={{cursor: "pointer"}}
              >
                <TableCell align="left">
                  {pool.token2 ? (
                    <PairAsset
                      assets={[
                        { icon: pool.token1.icon, label: pool.token1.symbol },
                        { icon: pool.token2.icon, label: pool.token2.symbol },
                      ]}
                      separator="/"
                    />
                  ) : (
                    <BaseAsset
                      label={pool.token1.symbol}
                      icon={pool.token1.icon}
                    />
                  )}
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">{pool.claimable.toFormat()} CHAOS</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">{pool.pending.toFormat()} CHAOS</Typography>
                </TableCell>
                <TableCell align="left">
                  <Typography variant="body2">
                    {pool.vesting_term} days
                  </Typography>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    );
  }
  return null;
};
