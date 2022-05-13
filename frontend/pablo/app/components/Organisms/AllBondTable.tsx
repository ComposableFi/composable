import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Box,
  Typography,
  useTheme,
  Tooltip,
} from "@mui/material";
import { BaseAsset, PairAsset } from "../Atoms";
import { useAppDispatch, useAppSelector } from "@/hooks/store";
import { addNextDataBondPools, BondPoolRow } from "@/stores/defi/polkadot";
import React, { useEffect, useState } from "react";
import { InfoOutlined, KeyboardArrowDown } from "@mui/icons-material";
import { TableHeader } from "@/defi/types";
import { useRouter } from "next/router";
import { useDotSamaContext } from "substrate-react";

const tableHeaders: TableHeader[] = [
  {
    header: "Pools",
  },
  {
    header: "TVL",
    tooltip: "",
  },
  {
    header: "ROI",
    tooltip: "",
  },
  {
    header: "Rewards Left",
    tooltip: "",
  },
  { header: "Volume", tooltip: "" },
];

export const AllBondTable: React.FC = () => {
  let pools: BondPoolRow[];
  const dispatch = useAppDispatch();
  const theme = useTheme();
  const router = useRouter();
  const [startIndex, setStartIndex] = useState(0);
  const polkaDotState = useAppSelector((state) => state.polkadot);
  const {extensionStatus} = useDotSamaContext();

  pools = polkaDotState.allBondPools;

  const handleSeeMore = () => {
    dispatch(addNextDataBondPools({ startIndex: startIndex + 4 }));
    setStartIndex(startIndex + 4);
  };

  const handleBondClick = () => {
    if (extensionStatus === "connected") {
      //TODO: set selected bond and go to bond select page
      router.push("bond/select");
    }
  };

  useEffect(() => {
    dispatch(addNextDataBondPools({ startIndex }));
  }, []);

  return (
    <TableContainer>
      <Table sx={{ minWidth: 420 }} aria-label="simple table">
        <TableHead>
          <TableRow>
            {tableHeaders.map((th) => {
              return typeof th.tooltip === "string" ? (
                <TableCell key={th.header} align="left">
                  <Box display="flex" alignItems="center" gap={1}>
                    {th.header}
                    <Tooltip arrow title={th.tooltip}>
                      <InfoOutlined
                        sx={{
                          height: 20,
                          width: 20,
                          color: theme.palette.primary.main,
                        }}
                      />
                    </Tooltip>
                  </Box>
                </TableCell>
              ) : (
                <TableCell key={th.header} align="left">{th.header}</TableCell>
              );
            })}
          </TableRow>
        </TableHead>
        <TableBody>
          {pools.map((row) => (
            <TableRow
              key={row.token1.symbol}
              onClick={handleBondClick}
              sx={{ cursor: "pointer" }}
            >
              <TableCell align="left">
                {row.token2 ? (
                  <PairAsset
                    assets={[
                      { icon: row.token1.icon, label: row.token1.symbol },
                      { icon: row.token2.icon, label: row.token2.symbol },
                    ]}
                    separator="/"
                  />
                ) : (
                  <BaseAsset label={row.token1.symbol} icon={row.token1.icon} />
                )}
              </TableCell>
              <TableCell align="left">
                <Typography variant="body2">${row.tvl.toFormat()}</Typography>
              </TableCell>
              <TableCell align="left">
                <Typography variant="body2">{row.roi.toFormat()}%</Typography>
              </TableCell>
              <TableCell align="left">
                {row.rewardsLeft.map((item) => {
                  return (
                    <Box key={item.token.id} display="flex">
                      <PairAsset
                        assets={[
                          {
                            icon: item.token.icon,
                            label: item.token.symbol,
                          },
                        ]}
                        label={item.value.toFormat(2)}
                      />
                    </Box>
                  );
                })}
              </TableCell>
              <TableCell align="left">
                <Typography variant="body2">
                  ${row.volume.toFormat()}
                </Typography>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      <Box
        onClick={handleSeeMore}
        mt={4}
        display="flex"
        gap={1}
        justifyContent="center"
        sx={{ cursor: "pointer" }}
      >
        <Typography textAlign="center" variant="body2">
          See more
        </Typography>
        <KeyboardArrowDown sx={{ color: theme.palette.primary.main }} />
      </Box>
    </TableContainer>
  );
};
