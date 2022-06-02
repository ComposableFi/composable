import React from "react";
import {
  alpha,
  Box,
  BoxProps,
  Button,
  ButtonProps,
  Typography,
  TypographyProps,
  useMediaQuery,
  useTheme,
} from "@mui/material";
import Image from "next/image";

export type FeaturedBoxProps = BoxProps & {
  variant?: "text" | "outlined" | "contained";
  title?: string;
  TitleProps?: TypographyProps;
  image?: React.ReactNode;
  textAbove?: string | React.ReactNode;
  textBelow?: string;
  TextAboveProps?: TypographyProps;
  TextBelowProps?: TypographyProps;
  token?: {
    icon: string;
    symbol: string;
  };
  horizontalAligned?: boolean;
  ButtonProps?: ButtonProps & {
    label: string;
  };
};

export const FeaturedBox: React.FC<FeaturedBoxProps> = ({
  variant,
  title,
  TitleProps,
  textAbove,
  textBelow,
  token,
  image,
  horizontalAligned,
  TextAboveProps,
  TextBelowProps,
  ButtonProps,
  ...rest
}) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down("sm"));
  const { label: buttonLabel, ...restButtonProps } = ButtonProps || {};

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
          ? alpha(theme.palette.common.white, theme.custom.opacity.lightest)
          : undefined
      }
      border={`1px solid ${alpha(
        theme.palette.common.white,
        theme.custom.opacity.light
      )}`}
      borderRadius={isMobile ? undefined : 1}
      {...rest}
    >
      <Box>
        {textAbove && typeof textAbove !== "string" && <>{textAbove}</>}
        {textAbove && typeof textAbove === "string" && (
          <Typography
            {...{
              variant: "subtitle1",
              color: "text.secondary",
              mb: 2,
              component: "div",
              ...TextAboveProps,
            }}
          >
            {textAbove}
          </Typography>
        )}
        {image ? image : null}
        {title && (
          <Typography variant="h6" color="text.primary" mb={2} {...TitleProps}>
            {title}
          </Typography>
        )}

        {token && (
          <Box
            display="flex"
            justifyContent={horizontalAligned && !isMobile ? "left" : "center"}
            alignItems="center"
          >
            <Image src={token.icon} width={24} height={24} alt={token.symbol} />
            <Typography
              variant="body2"
              color="text.secondary"
              component="div"
              ml={2}
            >
              {token.symbol}
            </Typography>
          </Box>
        )}

        {textBelow && (
          <Typography
            {...{
              component: "div",
              mt: 2,
              color: "text.secondary",
              variant: "body2",
              ...TextBelowProps,
            }}
          >
            {textBelow}
          </Typography>
        )}
      </Box>

      {buttonLabel && (
        <Box
          mt={horizontalAligned && !isMobile ? 0 : 4}
          ml={horizontalAligned && !isMobile ? 4 : 0}
        >
          <Button
            variant={"outlined"}
            size={isMobile ? "small" : "medium"}
            fullWidth={isMobile}
            sx={{
              px: 4,
            }}
            {...restButtonProps}
          >
            <Typography variant="button">{buttonLabel}</Typography>
          </Button>
        </Box>
      )}
    </Box>
  );
};
