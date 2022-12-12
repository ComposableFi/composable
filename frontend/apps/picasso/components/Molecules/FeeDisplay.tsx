import React, { ReactNode } from "react";
import {
  alpha,
  Box,
  Theme,
  Tooltip,
  TooltipProps,
  Typography,
  TypographyProps,
  useTheme,
} from "@mui/material";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";

const infoIconStyle = (theme: Theme) => ({
  color: theme.palette.primary.light,
  "&:hover": {
    color: theme.palette.secondary.main,
  },
});

export type FeeDisplayProps = {
  label: string;
  feeText: string | ReactNode;
  textFirst?: boolean;
  TypographyProps?: TypographyProps;
  TooltipProps?: Omit<TooltipProps, "children">;
};

export const FeeDisplay: React.FC<FeeDisplayProps> = ({
  label,
  feeText,
  textFirst = true,
  TypographyProps,
  TooltipProps,
}) => {
  const theme = useTheme();
  return (
    <Box
      display="flex"
      flexDirection={textFirst ? "row" : "row-reverse"}
      justifyContent="space-between"
      alignItems="center"
      component="div"
    >
      <Box
        display="flex"
        flexDirection="row"
        alignItems="center"
        component="div"
      >
        <Typography
          variant="body2"
          color={alpha(theme.palette.common.white, theme.custom.opacity.darker)}
          {...TypographyProps}
        >
          {label}
        </Typography>
        {TooltipProps?.title && (
          <Tooltip {...TooltipProps} arrow>
            <Box display="flex" alignItems="center" ml={2}>
              <InfoOutlinedIcon sx={infoIconStyle} />
            </Box>
          </Tooltip>
        )}
      </Box>
      {typeof feeText === "string" ? (
        <Typography variant="body2" color="text.primary" {...TypographyProps}>
          {feeText}
        </Typography>
      ) : (
        feeText
      )}
    </Box>
  );
};
