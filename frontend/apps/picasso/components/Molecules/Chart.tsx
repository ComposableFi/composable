import {
  Box,
  BoxProps,
  Typography,
  useTheme,
  alpha,
  TypographyProps,
  Button,
  CircularProgress,
} from "@mui/material";
import { AreaChart, AreaChartProps } from "../Atom";

export type ChartProps = {
  title?: string;
  TitleTypographyProps?: TypographyProps;
  totalText?: string;
  TotalTextTypographyProps?: TypographyProps;
  changeText?: string;
  changeTextColor?: string;
  ChangeTextTypographyProps?: TypographyProps;
  AreaChartProps: AreaChartProps;
  currentInterval?: string;
  onIntervalChange?: Function;
  isLoading?: boolean;
  intervals?: string[];
} & BoxProps;

export const Chart: React.FC<ChartProps> = ({
  title,
  TitleTypographyProps,
  totalText,
  TotalTextTypographyProps,
  changeText,
  changeTextColor,
  ChangeTextTypographyProps,
  AreaChartProps,
  currentInterval,
  onIntervalChange,
  isLoading,
  intervals,
  ...boxProps
}) => {
  const theme = useTheme();

  return (
    <Box
      borderRadius={1}
      padding={6}
      sx={{
        background: theme.palette.background.paper,
      }}
      {...boxProps}
    >
      <Box display="flex" alignItems="center" justifyContent="space-between">
        <Typography
          variant="body2"
          color="text.secondary"
          {...TitleTypographyProps}
        >
          {title}
        </Typography>
        <Box display="flex" alignItems="center" justifyContent="right">
          {intervals &&
            intervals.map((interval) => (
              <Button
                key={interval}
                size="small"
                variant="text"
                onClick={() => onIntervalChange?.(interval.toLowerCase())}
                disabled={isLoading}
                sx={{
                  color:
                    interval.toLowerCase() === currentInterval
                      ? theme.palette.text.primary
                      : theme.palette.text.secondary,
                  minWidth: 32,
                }}
              >
                {isLoading && interval.toLowerCase() === currentInterval ? (
                  <CircularProgress color="inherit" size={20} />
                ) : (
                  interval
                )}
              </Button>
            ))}
        </Box>
      </Box>
      {totalText && (
        <Typography variant="h5" mt={1} {...TotalTextTypographyProps}>
          {totalText}
        </Typography>
      )}
      {changeText && (
        <Typography
          variant="body2"
          color={
            changeTextColor ||
            AreaChartProps.color ||
            theme.palette.success.main
          }
          mt={1}
          {...ChangeTextTypographyProps}
        >
          {changeText}
        </Typography>
      )}
      {!totalText && !changeText && <Box mb={9} />}
      <AreaChart {...AreaChartProps} />
    </Box>
  );
};
