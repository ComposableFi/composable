import { Box, Typography, useTheme } from "@mui/material";
import CheckIcon from "@mui/icons-material/Check";
import type { FC } from "react";
import { Chart, FeaturedBox, VotingDetailsBox } from "@/components/Molecules";

export const StatsGovernanceTab: FC<{}> = ({}) => {
  const theme = useTheme();
  return (
    <>
      <Box mb={2}>
        <Box
          display="grid"
          sx={{
            gridTemplateColumns: {
              xs: "1fr",
              lg: "1fr 1fr 1fr",
            },
            gap: theme.spacing(4),
          }}
          mb={5}
        >
          <FeaturedBox textAbove="Proposals submitted" title="300.55K" />
          <FeaturedBox textAbove="Proposals passed" title="300.55K" />
          <FeaturedBox textAbove="Average time" title="300.55K" />
        </Box>
        <Chart
          title="Mempool &amp; fee growth"
          changeTextColor={theme.palette.featured.lemon}
          AreaChartProps={{
            data: [
              [1644550600000, 20],
              [1644560620928, 40],
              [1644570600000, 35],
              [1644580600000, 60],
              [1644590600000, 80],
            ],
            height: 200,
            shorthandLabel: "Change",
            labelFormat: (n: number) => n.toFixed(),
            color: theme.palette.primary.main,
          }}
          intervals={["1h", "24h", "1w", "1m"]}
        />
      </Box>
      <Typography variant="h5" color="text.primary" mb={2} component="div">
        Latest proposals passed
      </Typography>
      <Box mb={2}>
        <VotingDetailsBox
          id="12"
          title="Smart Contracts on Polkadot - WASM conference (Virtual)"
          status="success"
          statusText="Passed"
          timeText="19d 21h 45m remaining"
          statusIcon={<CheckIcon />}
          address="12tb....432"
          tagText="Ecosystem"
        />
      </Box>
      <Box mb={2}>
        <VotingDetailsBox
          id="12"
          title="Smart Contracts on Polkadot - WASM conference (Virtual)"
          status="success"
          statusText="Passed"
          timeText="19d 21h 45m remaining"
          statusIcon={<CheckIcon />}
          address="12tb....432"
          tagText="Ecosystem"
        />
      </Box>
    </>
  );
};
