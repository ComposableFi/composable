import React, { useMemo, useState } from "react";
import { Box, Grid, Link, Typography, useTheme } from "@mui/material";
import { TabPanel } from "../Atoms/TabPanel";
import { WalletViewTabs } from "../WalletViewModal";
import moment from "moment";

export type TransactionsPanelProps = {
  activePanel: WalletViewTabs;
  transactions: Array<{ title: string; timestamp: number }>;
};

export const TransactionsPanel = ({
  activePanel,
  transactions,
}: TransactionsPanelProps) => {
  const theme = useTheme();

  const [lastClearedTimestamp, setLastClearedTimestamp] = useState(0);
  const filtered = useMemo(() => {
    return transactions.filter(tx => {
      return tx.timestamp > lastClearedTimestamp
    });
  }, [lastClearedTimestamp, transactions])

  return (
    <TabPanel value={activePanel} index={WalletViewTabs.Transactions}>
      <Grid container>
        <Grid item xs={12} display="flex" justifyContent={"space-between"}>
          <Typography variant="inputLabel">Recent Transactions</Typography>
          <Typography variant="inputLabel">
            <Link onClick={(evt) => {
              setLastClearedTimestamp(Date.now())
            }}>Clear All</Link>
          </Typography>
        </Grid>

        <Grid item xs={12} marginTop={theme.spacing(2)}>
          <Box
            sx={{
              height: "172px",
              px: 0,
              overflowY: "scroll",
            }}
          >
            {filtered.map((tx) => {
              return (
                <Box sx={{display: "block"}}>
                  <Typography variant="caption">{tx.title}</Typography>{" "}
                  <Typography variant="caption">
                    {moment.utc(tx.timestamp).format("DD/MM/yyyy")}
                  </Typography>
                </Box>
              );
            })}
          </Box>
        </Grid>
      </Grid>
    </TabPanel>
  );
};
