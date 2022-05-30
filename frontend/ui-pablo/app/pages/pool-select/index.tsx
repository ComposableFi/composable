import type { NextPage } from "next";
import {
  Container,
  Box,
  Grid,
  useTheme,
  Button,
  Typography,
  alpha,
} from "@mui/material";
import Default from "@/components/Templates/Default";
import { Chart, PageTitle } from "@/components";
import { DonutChart } from "@/components/Atoms/DonutChart";
import { useRouter } from "next/router";

const twoColumnPageSize = {
  xs: 12,
  md: 6,
};

const PoolSelect: NextPage = () => {
  const theme = useTheme();
  const router = useRouter();

  const handleClick = () => {
    router.push("/pool/add-liquidity");
  };

  const goRemoveLiquidity = () => {
    router.push("/pool/remove-liquidity");
  };

  return (
    <Default>
      <Container maxWidth="lg">
        <Box display="flex" flexDirection="column" alignItems="center">
          <PageTitle
            title="PICA/KSM Pool"
            subtitle="Earn tokens while adding liquidity."
          />
        </Box>
        <Grid mt={4} container spacing={4}>
          <Grid item {...twoColumnPageSize}>
            <Chart
              title="TVL"
              changeTextColor={theme.palette.common.white}
              changeText="Past 1 week"
              AreaChartProps={{
                data: [
                  [1644550600000, 20],
                  [1644560620928, 45],
                  [1644570600000, 40],
                  [1644590600000, 100],
                ],
                height: 300,
                shorthandLabel: "Change",
                labelFormat: (n: number) => n.toFixed(),
                color: theme.palette.common.white,
              }}
              marginTop={17}
              intervals={["1w", "1m", "1y", "All"]}
              currentInterval="1w"
            />
          </Grid>
          <Grid item {...twoColumnPageSize}>
            <Box display="flex" gap={3}>
              <Button
                onClick={handleClick}
                variant="contained"
                sx={{ width: "50%" }}
              >
                Add Liquidity
              </Button>
              <Button
                sx={{ width: "50%" }}
                onClick={goRemoveLiquidity}
                variant="outlined"
              >
                Remove Liquidity
              </Button>
            </Box>
            <Box
              mt={4}
              padding={4}
              sx={{
                background: alpha(
                  theme.palette.common.white,
                  theme.custom.opacity.lightest
                ),
                borderRadius: 1,
              }}
              border={`1px solid ${alpha(
                theme.palette.common.white,
                theme.custom.opacity.light
              )}`}
            >
              <Box display="flex" justifyContent="center" mb={2}>
                <DonutChart
                  data={[20, 80]}
                  colors={[
                    alpha(theme.palette.common.white, 0.1),
                    alpha(theme.palette.common.white, 0.2),
                  ]}
                  labels={["Your Position", "Total Value Locked"]}
                  width={"125%"}
                />
              </Box>
              <Box display="flex" justifyContent="space-between">
                <Typography variant="body2" mb={2}>
                  Your LP Tokens
                </Typography>
                <Typography variant="body2" mb={2}>
                  1.56
                </Typography>
              </Box>
              <Box display="flex" justifyContent="space-between">
                <Typography variant="body2" mb={2}>
                  Pooled PICA
                </Typography>
                <Typography variant="body2" mb={2}>
                  59.28
                </Typography>
              </Box>
              <Box display="flex" justifyContent="space-between">
                <Typography variant="body2" mb={2}>
                  Pooled KSM
                </Typography>
                <Typography variant="body2" mb={2}>
                  59.28
                </Typography>
              </Box>
              <Box display="flex" justifyContent="space-between">
                <Typography variant="body2" mb={2}>
                  Share of Pool
                </Typography>
                <Typography variant="body2" mb={2}>
                  3.3%
                </Typography>
              </Box>
            </Box>
          </Grid>
        </Grid>
      </Container>
    </Default>
  );
};

export default PoolSelect;
