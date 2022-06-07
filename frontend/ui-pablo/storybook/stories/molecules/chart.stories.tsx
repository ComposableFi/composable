import { 
  ChartProps,
  Chart 
} from "@/components";
import { 
  Box, 
  SxProps,
  useTheme, 
} from "@mui/material";
import { Story } from "@storybook/react";

const chartProps = {
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
  color: "#FF8500",
};

const ChartStories = (props: ChartProps) => {
  const theme = useTheme();
  const boxStyle: Partial<SxProps> = {
    display: "flex",
    flexDirection: "column",
    gap: 2,
    resize: "both",
    overflow: "auto",
  };

  return (
    <Box sx={boxStyle}>
      <Chart {...props} 
            AreaChartProps={{
              ...chartProps,
            }}
      />

      <Chart {...props} 
            AreaChartProps={{
              ...chartProps,
              color: theme.palette['featured'].main,
            }}
      />

      <Chart {...props} 
            AreaChartProps={{
              ...chartProps,
              color: theme.palette['error'].main,
            }}
      />
    </Box>
  );
};
export default {
  title: "molecules/Chart",
  component: Chart,
};

const Template: Story<typeof ChartStories> = (args) => (
  <ChartStories {...args} />
);

export const DefaultAreaChart = Template.bind({});
DefaultAreaChart.args = {
  title: "My portofolio",
  TitleTypographyProps: {},
  totalText: "$24,587,298",
  TotalTextTypographyProps: {},
  changeText: "+34%",
  changeTextColor: "#33C500",
  ChangeTextTypographyProps: {},
  AreaChartProps: chartProps,
  currentInterval: '1h',
  onIntervalChange: () => {},
  isLoading: false,
  intervals: ['1h', '24h', '1w', '1m', '1y'],
};