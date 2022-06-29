import { Box } from "@mui/material";
import { ApolloTable } from "@/components/Molecules";
import { useAppSelector } from "@/hooks/store";

export const StatsApolloTab: React.FC<any> = () => {
  const apolloData = useAppSelector((state) => state.statsApollo.assets);

  return (
    <Box
      sx={{
        padding: 6,
        backgroundColor: "rgba(255, 255, 255, 0.02)",
        borderRadius: 1,
      }}
    >
      <ApolloTable assets={apolloData} />
    </Box>
  );
};
