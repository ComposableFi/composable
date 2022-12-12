import { useMemo, useState } from "react";
import { alpha, Box, Grid, Link, Typography, useTheme } from "@mui/material";
import { TabPanel } from "../Atoms/TabPanel";
import { WalletViewTabs } from "../WalletViewModal";
import moment from "moment";
import Image from "next/image";

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
    return transactions.filter((tx) => {
      return tx.timestamp > lastClearedTimestamp;
    });
  }, [lastClearedTimestamp, transactions]);

  return (
    <TabPanel value={activePanel} index={WalletViewTabs.Transactions}>
      <Grid container>
        <Grid item xs={12} display="flex" justifyContent={"space-between"}>
          <Typography variant="body2">Recent Transactions</Typography>
          <Link
            color={
              filtered.length > 0 ? "primary" : theme.palette.secondary.light
            }
            variant="body2"
            onClick={(evt) => {
              setLastClearedTimestamp(Date.now());
            }}
            sx={{
              textDecoration: "none",
              cursor: filtered.length > 0 ? "pointer" : "not-allowed",
            }}
          >
            Clear All
          </Link>
        </Grid>

        <Grid item xs={12} marginTop={theme.spacing(2)}>
          {filtered.length > 0 ? (
            <Box
              sx={{
                height: "172px",
                px: 0,
                overflowY: "scroll",
              }}
            >
              {filtered.map((tx) => {
                return (
                  <Box sx={{ display: "block" }}>
                    <Typography variant="caption">{tx.title}</Typography>{" "}
                    <Typography variant="caption">
                      {moment.utc(tx.timestamp).format("DD/MM/yyyy")}
                    </Typography>
                  </Box>
                );
              })}
            </Box>
          ) : (
            <Box
              sx={{
                gap: "2rem",
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                justifyContent: "center",
              }}
            >
              <Image
                src="/static/lemonade.png"
                css={{ mixBlendMode: "luminosity" }}
                width="96"
                height="96"
                alt="lemonade"
              />
              <Typography
                variant="body2"
                color={alpha(theme.palette.common.white, 0.6)}
              >
                Your transactions will appear here.
              </Typography>
            </Box>
          )}
        </Grid>
      </Grid>
    </TabPanel>
  );
};
