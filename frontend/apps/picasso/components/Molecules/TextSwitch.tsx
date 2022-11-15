import React from "react";
import {
  TypographyProps,
  Box,
  BoxProps,
  Typography,
  TooltipProps,
  Tooltip,
  Switch,
  useTheme,
  alpha,
} from "@mui/material";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import { Theme } from "@mui/material";

const infoIconStyle = (theme: Theme) => ({
  color: theme.palette.primary.light,
  "&:hover": {
    color: theme.palette.secondary.main,
  },
});

export type TextSwitchProps = {
  label: string;
  TypographyProps?: TypographyProps;
  TooltipProps?: Omit<TooltipProps, "children">;
  textFirst?: boolean;
  checked: boolean;
  onChange: (event: React.ChangeEvent<HTMLInputElement>) => void;
} & BoxProps;

export const TextSwitch: React.FC<TextSwitchProps> = ({
  label,
  TypographyProps,
  TooltipProps,
  textFirst = true,
  checked,
  onChange,
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
          mr={2}
          ml={textFirst ? 0 : 2}
          {...TypographyProps}
        >
          {label}
        </Typography>
        {TooltipProps?.title && (
          <Tooltip {...TooltipProps} arrow>
            <InfoOutlinedIcon sx={infoIconStyle} />
          </Tooltip>
        )}
      </Box>
      <Switch checked={checked} onChange={onChange} />
    </Box>
  );
};
