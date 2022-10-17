import { Box, Typography, useTheme } from "@mui/material";
import { TypographyProps } from "@mui/system";

export type PageTitleProps = TypographyProps & {
  title: string;
  subtitle?: string;
  withDot?: boolean;
};
export const PageTitle: React.FC<PageTitleProps> = ({
  title,
  subtitle,
  withDot = false,
  ...props
}) => {
  const theme = useTheme();
  const { textAlign = "left", ...rest } = props;

  return (
    <>
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
