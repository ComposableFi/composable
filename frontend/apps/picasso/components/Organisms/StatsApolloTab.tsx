import { Box } from "@mui/material";
import { ApolloTable } from "@/components/Molecules";
import { useStore } from "@/stores/root";

export const StatsApolloTab: React.FC<any> = () => {
  const { assets } = useStore(({ statsApollo }) => statsApollo);

  return (
    <Box
      sx={{
        padding: 6,
        backgroundColor: "rgba(255, 255, 255, 0.02)",
        borderRadius: 1,
      }}
    >
      <ApolloTable assets={assets} />
    </Box>
  );
};
