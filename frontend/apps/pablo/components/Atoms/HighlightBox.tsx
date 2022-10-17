import { Box, BoxProps, useMediaQuery, useTheme } from "@mui/material";
import { FC } from "react";

export const HighlightBox: FC<
  BoxProps & {
    variant?: "text" | "outlined" | "contained";
    horizontalAligned?: boolean;
  }
> = ({ variant = "contained", horizontalAligned, children, ...rest }) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down("sm"));
  return (
    <Box
      padding={isMobile ? [3, 2] : 4}
      textAlign={horizontalAligned && !isMobile ? undefined : "center"}
      display={horizontalAligned && !isMobile ? "flex" : undefined}
      alignItems={horizontalAligned && !isMobile ? "center" : undefined}
      justifyContent={
        horizontalAligned && !isMobile ? "space-between" : undefined
      }
      bgcolor={
        !variant || variant == "contained"
          ? theme.palette.background.transparentCharcoal
          : undefined
      }
      borderRadius={1}
      {...rest}
    >
      {children}
    </Box>
  );
};
