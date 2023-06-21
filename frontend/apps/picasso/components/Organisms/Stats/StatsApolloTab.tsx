import { Box } from "@mui/material";
import { ApolloTable } from "@/components/Molecules";
import { FC } from "react";

export const StatsApolloTab: FC = () => {

  return (
    <Box
      sx={{
        padding: 6,
        backgroundColor: "rgba(255, 255, 255, 0.02)",
        borderRadius: 1
      }}
    >
      <ApolloTable />
    </Box>
  );
};
