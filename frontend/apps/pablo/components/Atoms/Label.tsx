import React from "react";
import {
  Typography,
  useTheme,
  TooltipProps as MuiTooltipProps,
  TypographyProps as MuiTypographyProps,
  Box,
  BoxProps,
  Tooltip,
} from "@mui/material";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";

export type LabelProps = {
  label?: string;
  TypographyProps?: MuiTypographyProps;
  TooltipProps?: Omit<MuiTooltipProps, "children">;
  BalanceProps?: {
    title?: string | JSX.Element;
    TitleTypographyProps?: MuiTypographyProps;
    balance?: string;
    BalanceTypographyProps?: MuiTypographyProps;
  };
} & BoxProps;

export const Label: React.FC<LabelProps> = ({
  label,
  TypographyProps,
  TooltipProps,
  BalanceProps,
  children,
  ...boxProps
}) => {
  const theme = useTheme();
  return (
    <Box
      display="flex"
      alignItems="center"
      justifyContent="space-between"
      mb={1.5}
      {...boxProps}
    >
      {
        <Box display="flex" alignItems="center" gap={1.75}>
          {
            label ? (
              <Typography variant="inputLabel" color="text.primary" {...TypographyProps}>
                {label}
              </Typography>
            ) : children
          }
          {TooltipProps && TooltipProps.title && (
            <Tooltip {...TooltipProps} arrow>
              <InfoOutlinedIcon
                sx={{
                  color: theme.palette.primary.light,
                  "&:hover": {
                    color: theme.palette.secondary.main,
                  },
                }}
              />
            </Tooltip>
          )}
        </Box>
      }
      {BalanceProps && (BalanceProps.title || BalanceProps.balance) && (
        <Box display="flex">
          {
            (typeof BalanceProps.title) === 'string' ? (
              <Typography
                variant="body2"
                color="text.secondary"
                {...BalanceProps.TitleTypographyProps}
              >
                {BalanceProps.title}
              </Typography>
            ) : (<Box display="flex">{BalanceProps.title}</Box>)
          }
          <Typography
            variant="body2"
            ml={0.5}
            {...BalanceProps.BalanceTypographyProps}
          >
            {BalanceProps.balance}
          </Typography>
        </Box>
      )}
    </Box>
  );
};
