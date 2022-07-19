import { Box, useTheme } from "@mui/material";
import { useStore } from "@/stores/root";
import { formatNumberWithSymbol, formatNumber } from "shared";
import { TreasuryDataProps } from "@/stores/defi/stats/treasury";
import { Chart, FeaturedBox } from "@/components/Molecules";

function formatTreasuryTitleValue(index: number, info: TreasuryDataProps) {
  switch (index) {
    case 0:
      return formatNumberWithSymbol(info.value[0], "$");
    case 1:
      return `${formatNumberWithSymbol(
        info.value[0],
        "$"
      )} | ${formatNumberWithSymbol(info.value[1], "$")}`;
    case 2:
      return formatNumberWithSymbol(info.value[0], "$");
    case 3:
      return formatNumberWithSymbol(info.value[0], "$");
    case 4:
      return `${formatNumber(info.value[0])}% | ${formatNumberWithSymbol(
        info.value[1],
        "",
        " months"
      )}`;
    case 5:
      return `${formatNumberWithSymbol(info.value[0], "", "%")}`;
    default:
      return undefined;
  }
}

export const StatsTreasuryTab: React.FC<{}> = ({}) => {
  const theme = useTheme();
  const { treasuryData, treasuryChartData } = useStore(
    ({ statsTreasury }) => statsTreasury
  );

  return (
    <>
      <Box
        display="grid"
        sx={{
          gridTemplateColumns: {
            xs: "1fr",
            lg: "1fr 1fr 1fr"
          }
        }}
        mb={5}
        gap={4}
      >
        {treasuryData.data.map((info, index) => (
          <FeaturedBox
            key={index}
            textAbove={info.name}
            title={formatTreasuryTitleValue(index, info)}
            TooltipProps={{ title: info.tooltip }}
          />
        ))}
      </Box>
      <Box
        display="grid"
        sx={{
          gridTemplateColumns: {
            xs: "1fr",
            lg: "1fr 1fr"
          }
        }}
        gap={4}
      >
        {treasuryChartData.data.map((info, index) => (
          <Chart
            key={index}
            title={info.data.name}
            totalText={formatNumberWithSymbol(info.data.value, "$")}
            changeText={formatNumberWithSymbol(info.data.change, "", "%")}
            changeTextColor={
              info.data.change >= 0
                ? theme.palette.featured.lemon
                : theme.palette.error.main
            }
            AreaChartProps={{
              data: info.data.data[0],
              height: 90.7,
              shorthandLabel: "Change",
              labelFormat: (n: number) => n.toFixed(),
              color: theme.palette.primary.main
            }}
          />
        ))}
      </Box>
    </>
  );
};
