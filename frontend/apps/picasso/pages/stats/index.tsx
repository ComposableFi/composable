import type { NextPage } from "next";
import React from "react";
import Default from "@/components/Templates/Default";
import { Grid } from "@mui/material";
import { PageTitle, TabItem, Tabs } from "@/components";
import { StatsOverviewTab } from "@/components/Organisms/Stats/StatsOverviewTab";
import { StatsTelemetryTab } from "@/components/Organisms/Stats/StatsTelemetryTab";
import { StatsApolloTab } from "@/components/Organisms/Stats/StatsApolloTab";

const tabItems: TabItem[] = [
  {
    label: "Overview"
  },
  {
    label: "Telemetry"
  },
  {
    label: "Apollo"
  }
];

const Stats: NextPage = () => {
  const [tabIndex, setTabIndex] = React.useState(0);

  const handleChange = (_: React.SyntheticEvent, newValue: number) => {
    setTabIndex(newValue);
  };

  const standardPageSize = {
    xs: 12
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
        gap={4}
      >
        <Grid item {...standardPageSize} mt={9}>
          <PageTitle
            title="Stats"
            subtitle="You will be able to see all Picasso's global information here."
            textAlign="center"
          />
        </Grid>
        <Grid item {...standardPageSize}>
          <Tabs value={tabIndex} onChange={handleChange} items={tabItems} />
        </Grid>
        <Grid item {...standardPageSize}>
          {tabIndex === 0 && <StatsOverviewTab />}
          {tabIndex === 1 && <StatsTelemetryTab />}
          {tabIndex === 2 && <StatsApolloTab />}
        </Grid>
      </Grid>
    </Default>
  );
};

export default Stats;
