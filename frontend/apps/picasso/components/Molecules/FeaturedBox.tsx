import React from "react";
import {
  alpha,
  Box,
  BoxProps,
  Button,
  ButtonProps,
  Theme,
  Tooltip,
  TooltipProps,
  Typography,
  TypographyProps,
  useMediaQuery,
  useTheme
} from "@mui/material";
import Image from "next/image";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";

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
  TooltipProps?: Omit<TooltipProps, "children">;
};

const infoIconStyle = (theme: Theme) => ({
  color: theme.palette.primary.light,
  "&:hover": {
    color: theme.palette.secondary.main
  }
});

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
  TooltipProps,
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
        <Box
          display="flex"
          flexDirection="row"
          justifyContent="center"
          alignItems="center"
          component="div"
        >
          {React.isValidElement(textAbove) ? (
            textAbove
          ) : (
            <>
              <Typography
                variant="body2"
                color={titleColor || theme.palette.text.primary}
                mb={2}
                {...TextAboveProps}
              >
                {textAbove}
              </Typography>
              {TooltipProps?.title && (
                <Tooltip {...TooltipProps} arrow>
                  <Box display="flex" alignItems="center" ml={2} mb={2}>
                    <InfoOutlinedIcon sx={infoIconStyle} />
                  </Box>
                </Tooltip>
              )}
            </>
          )}
        </Box>
        {image && <>{image}</>}
        {title && (
          <Typography
            variant="h5"
            color={titleColor ?? "text.primary"}
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
              ...TextBelowProps
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
            variant="outlined"
            size={isMobile ? "small" : "medium"}
            fullWidth={isMobile}
            sx={{
              px: 4
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
