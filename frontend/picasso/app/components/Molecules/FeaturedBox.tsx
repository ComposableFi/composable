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
  image?: React.ReactNode;
  textAbove?: string | React.ReactNode;
  textBelow?: string | React.ReactNode;
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
  titleColor?: string;
};

export const FeaturedBox: React.FC<FeaturedBoxProps> = ({
  variant,
  title,
  textAbove,
  textBelow,
  token,
  image,
  horizontalAligned,
  titleColor,
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
          ? alpha(theme.palette.common.white, 0.02)
          : undefined
      }
      border={
        variant == "outlined"
          ? `1px solid ${alpha(
              theme.palette.common.white,
              theme.custom.opacity.light
            )}`
          : undefined
      }
      borderRadius={isMobile ? undefined : 1}
      {...rest}
    >
      <Box>
        {React.isValidElement(textAbove) ? (
          textAbove
        ) : (
          <Typography
            variant="body1"
            color={titleColor || theme.palette.text.primary}
            {...TextAboveProps}
          >
            {textAbove}
          </Typography>
        )}
        {image && <>{image}</>}
        {title && (
          <Typography
            variant="h6"
            color={titleColor ?? "text.primary"}
            mb={2}
            component="div"
          >
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
        {React.isValidElement(textBelow) ? (
          textBelow
        ) : (
          <Typography
            {...{
              variant: "body2",
              color: "text.secondary",
              mb: 2,
              component: "div",
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
