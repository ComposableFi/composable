import React, { ReactNode } from "react";
import {
  Typography,
  useTheme,
  TooltipProps,
  TypographyProps,
  Box,
  BoxProps,
  Tooltip,
} from "@mui/material";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";

export type LabelProps = {
  mainLabelProps?: {
    label?: string;
    TypographyProps?: TypographyProps;
    TooltipProps?: TooltipProps;
  };
  balanceLabelProps?: {
    label?: string;
    LabelTypographyProps?: TypographyProps;
    balanceText?: string | ReactNode;
    BalanceTypographyProps?: TypographyProps;
  };
} & BoxProps;

export const Label: React.FC<LabelProps> = ({
  mainLabelProps,
  balanceLabelProps,
  ...boxProps
}) => {
  const theme = useTheme();
  return mainLabelProps || balanceLabelProps ? (
    <Box
      display="flex"
      alignItems="center"
      justifyContent="space-between"
      {...boxProps}
    >
      {mainLabelProps && mainLabelProps.label && (
        <Box display="flex" alignItems="center" gap={1.75} marginBottom={1.5}>
          <Typography
            variant="inputLabel"
            color="text.primary"
            {...mainLabelProps.TypographyProps}
          >
            {mainLabelProps.label}
          </Typography>
          {mainLabelProps.TooltipProps && mainLabelProps.TooltipProps.title && (
            <Tooltip {...mainLabelProps.TooltipProps} arrow>
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
      )}
      {balanceLabelProps &&
        (balanceLabelProps.label || balanceLabelProps.balanceText) && (
          <Box display="flex">
            <Typography
              variant="inputLabel"
              color="text.secondary"
              {...balanceLabelProps.LabelTypographyProps}
            >
              {balanceLabelProps.label}
            </Typography>
            {typeof balanceLabelProps.balanceText === "string" ? (
              <Typography
                variant="inputLabel"
                ml={0.5}
                {...balanceLabelProps.BalanceTypographyProps}
              >
                {balanceLabelProps.balanceText}
              </Typography>
            ) : (
              balanceLabelProps.balanceText
            )}
          </Box>
        )}
    </Box>
  ) : (
    <></>
  );
};
