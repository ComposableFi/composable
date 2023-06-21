import React from "react";
import {
  AlertColor,
  alpha,
  Box,
  BoxProps,
  Typography,
  TypographyProps,
  useTheme,
} from "@mui/material";

export type NotificationBoxProps = {
  type?: AlertColor;
  icon?: JSX.Element;
  mainText?: string;
  MainTextProps?: TypographyProps;
  subText?: string;
  SubTextProps?: TypographyProps;
} & BoxProps;

export const NotificationBox: React.FC<NotificationBoxProps> = ({
  type = "warning",
  icon,
  mainText,
  MainTextProps,
  subText,
  SubTextProps,
  ...boxProps
}) => {
  const theme = useTheme();

  return (
    <Box
      p={3}
      borderRadius={1}
      sx={{
        background: alpha(theme.palette[type].main, theme.custom.opacity.light),
      }}
      {...boxProps}
    >
      <Box display="flex" gap={2.5}>
        {icon && icon}
        <Box>
          {mainText && (
            <Typography
              variant="body1"
              fontWeight="600"
              color={`${type}.main`}
              {...MainTextProps}
            >
              {mainText}
            </Typography>
          )}
          {subText && (
            <Typography
              variant="body2"
              color={`${type}.main`}
              mt={1}
              {...SubTextProps}
            >
              {subText}
            </Typography>
          )}
        </Box>
      </Box>
    </Box>
  );
};
