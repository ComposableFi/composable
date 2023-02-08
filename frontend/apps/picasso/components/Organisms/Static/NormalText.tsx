import { Typography, TypographyProps } from "@mui/material";

export const NormalText = ({ children, sx, ...props }: TypographyProps) => {
  return (
    <Typography
      paragraph
      variant="body3"
      sx={{
        mt: 1,
        ...sx,
      }}
      {...props}
    >
      {children}
    </Typography>
  );
};
