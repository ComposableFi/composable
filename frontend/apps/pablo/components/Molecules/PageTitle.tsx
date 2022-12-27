import { Box, Typography, useTheme } from "@mui/material";
import { TypographyProps } from "@mui/system";
import { FC } from "react";
import Head from "next/head";
import config from "@/constants/config";

export type PageTitleProps = TypographyProps & {
  title: string;
  subtitle?: string;
  withDot?: boolean;
};
export const PageTitle: FC<PageTitleProps> = ({
  title,
  subtitle,
  withDot = false,
  ...props
}) => {
  const theme = useTheme();
  const { textAlign = "left", ...rest } = props;

  return (
    <>
      <Head>
        <title>{`${title} - ${config.appDescription}`}</title>
      </Head>
      <Typography
        textAlign={textAlign}
        variant="h4"
        {...rest}
        sx={{
          marginBottom: theme.spacing(3),
          fontWeight: "bold",
        }}
      >
        {title}
        {withDot && (
          <Box sx={{ display: "inline", color: theme.palette.primary.main }}>
            .
          </Box>
        )}
      </Typography>
      {subtitle && (
        <Typography
          variant="body1"
          textAlign={textAlign}
          color="text.secondary"
        >
          {subtitle}
        </Typography>
      )}
    </>
  );
};
