import { Box, useTheme } from "@mui/material";
import { FeaturedBox } from "@/components/Molecules";
import { FC } from "react";
import { useTelemetry } from "@/defi/polkadot/hooks/useTelemetry";

export const StatsTelemetryTab: FC<{}> = () => {
  const theme = useTheme();
  const { finalizedBlock, lastBlock, getBlockAverage } = useTelemetry();

  return (
    <Box
      display="grid"
      sx={{
        gridTemplateColumns: {
          xs: "1fr",
          lg: "1fr 1fr 1fr"
        }
      }}
      gap={theme.spacing(4)}
    >
      <FeaturedBox
        key="finalizedBlock"
        textAbove="Finalized Block"
        title={finalizedBlock.toFormat()}
      />
      <FeaturedBox
        key="averageTime"
        textAbove="Average Time"
        title={
          getBlockAverage()
            .div(1000)
            .toFormat(1)
            .toString() + "s"
        }
      />
      <FeaturedBox
        key="lastBlock"
        textAbove="Last Block"
        title={lastBlock.toFormat()}
      />
    </Box>
  );
};
