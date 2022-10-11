import {
  Box,
  BoxProps,
  Button,
  CircularProgress,
  Typography,
  TypographyProps,
  useTheme,
} from "@mui/material";
import { ReactElement } from "react";
import { AreaChart, AreaChartProps } from "../Atoms";

export type ChartProps = {
  title?: string;
  titleComponent?: ReactElement;
  TitleTypographyProps?: TypographyProps;
  totalText?: string;
  TotalTextTypographyProps?: TypographyProps;
  changeText?: string;
  changeTextColor?: string;
  ChangeTextTypographyProps?: TypographyProps;
  changeIntroText?: string;
  AreaChartProps: AreaChartProps;
  currentInterval?: string;
  onIntervalChange?: Function;
  isLoading?: boolean;
  intervals?: string[];
  timeSlots?: string[];
  marginTop?: number;
} & BoxProps;

export const Chart: React.FC<ChartProps> = ({
  title,
  titleComponent,
  TitleTypographyProps,
  totalText,
  TotalTextTypographyProps,
  changeText,
  changeTextColor,
  ChangeTextTypographyProps,
  changeIntroText,
  AreaChartProps,
  currentInterval,
  onIntervalChange,
  isLoading,
  intervals,
  timeSlots,
  marginTop,
  ...boxProps
}) => {
  const theme = useTheme();

  return (
    <>
      <Box
        display="flex"
        alignItems={typeof title === "string" ? "center" : "flex-start"}
        justifyContent="space-between"
        sx={{
          [theme.breakpoints.down("sm")]: {
            flexDirection: "column",
          },
        }}
      >
        {typeof title === "string" && (
          <Typography
            variant="h5"
            color="text.primary"
            {...TitleTypographyProps}
          >
            {title}
          </Typography>
        )}
        {titleComponent && titleComponent}
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
                  minWidth: 24,
                  padding: 1.25,
                  fontSize: "1.125rem",
                  borderRadius: "50%",
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
        <Box display="flex" alignItems="center" gap={1.5} mt={1} mb={3}>
          <Typography
            variant="body2"
            color={
              changeTextColor ||
              AreaChartProps.color ||
              theme.palette.success.main
            }
            {...ChangeTextTypographyProps}
          >
            {changeText}
          </Typography>
          {changeIntroText && (
            <Typography variant="body2" color="text.primary">
              {changeIntroText}
            </Typography>
          )}
        </Box>
      )}
      {!totalText && !changeText && <Box mb={9} />}
      <AreaChart marginTop={marginTop} {...AreaChartProps} />
      {timeSlots && (
        <Box
          mt={1.5}
          display="flex"
          alignItems="center"
          justifyContent="space-between"
        >
          {timeSlots.map((slot, index) => (
            <Typography
              key={index}
              variant="caption"
              sx={{ whiteSpace: "nowrap" }}
            >
              {slot}
            </Typography>
          ))}
        </Box>
      )}
    </>
  );
};
