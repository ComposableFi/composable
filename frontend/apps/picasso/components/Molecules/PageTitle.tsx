import { Box, Typography, useTheme } from "@mui/material";
import { TypographyProps } from "@mui/system";
import Head from "next/head";
import { FC } from "react";

type PageTitleProps = TypographyProps & {
  title: string;
  subtitle?: string;
};
export const PageTitle: FC<PageTitleProps> = ({
  title,
  subtitle,
  ...props
}) => {
  const theme = useTheme();
  const { textAlign, ...rest } = props;

  return (
    <>
      <Head>
        <title>{`${title} - Composable Finance Picasso Parachain`}</title>
      </Head>
      <Typography
        textAlign={textAlign}
        variant="h4"
        {...rest}
        sx={{
          marginBottom: theme.spacing(2.625),
        }}
      >
        {title}
        <Box sx={{ display: "inline", color: theme.palette.primary.main }}>
          .
        </Box>
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
