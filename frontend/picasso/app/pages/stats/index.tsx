import type { NextPage } from "next";
import React from "react";
import Default from "@/components/Templates/Default";
import { useTheme, Grid } from "@mui/material";
import { Chart, PageTitle } from "@/components";
import { TabItem, Tabs } from "@/components";
import { StatsOverviewTab } from "@/components/Organisms/StatsOverviewTab";
import { StatsTelemetryTab } from "@/components/Organisms/StatsTelemetryTab";
import { StatsGovernanceTab } from "@/components/Organisms/StatsGovernanceTab";

const tabItems: TabItem[] = [
  {
    label: "Overview",
  },
  {
    label: "Telemetry",
  },
  {
    label: "Governance",
  },
];

const Stats: NextPage = () => {
  const theme = useTheme();
  const [tabIndex, setTabIndex] = React.useState(0);

  const handleChange = (_: React.SyntheticEvent, newValue: number) => {
    setTabIndex(newValue);
  };

  const standardPageSize = {
    xs: 12,
  };

  return (
    <Default>
      <Grid
        container
        sx={{ mx: "auto" }}
        maxWidth={1032}
        rowSpacing={5}
        columns={10}
        direction="column"
        justifyContent="center"
      >
        <Grid item {...standardPageSize} mt={theme.spacing(9)}>
          <PageTitle
            title="Stats"
            subtitle="You will be able to check on your positons here."
            textAlign="center"
          />
        </Grid>
        <Grid item {...standardPageSize}>
          <Tabs value={tabIndex} onChange={handleChange} items={tabItems} />
        </Grid>
        <Grid item {...standardPageSize}>
          {tabIndex === 0 && <StatsOverviewTab />}
          {tabIndex === 1 && <StatsTelemetryTab />}
          {tabIndex === 2 && <StatsGovernanceTab />}
        </Grid>
      </Grid>
    </Default>
  );
};

export default Stats;
