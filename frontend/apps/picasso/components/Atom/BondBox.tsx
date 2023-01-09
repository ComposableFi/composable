import { FC } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import { alpha, useTheme } from "@mui/material/styles";

export type BondBoxProps = {
  title: string;
  description: string;
  discountColor?: number;
};

export const BondBox: FC<BondBoxProps> = ({
  title,
  description,
  discountColor = 0,
}) => {
  const theme = useTheme();

  const successOrError = discountColor > 0
    ? theme.palette.featured.lemon
    : theme.palette.error.main;

  return (
    <Box
      sx={{
        display: "grid",
        alignItems: "center",
        padding: "1.5rem",
        backgroundColor: alpha(theme.palette.common.white, 0.02),
        borderRadius: "0.75rem",
      }}
      gap={1}
    >
      <Typography
        variant="body1"
        textAlign="center"
        color={alpha(theme.palette.common.white, 0.6)}
      >
        {title}
      </Typography>
      <Typography
        variant="h6"
        textAlign="center"
        color={
          discountColor == 0
            ? theme.palette.common.white
            : successOrError
        }
      >
        {description}
      </Typography>
    </Box>
  );
};

export default BondBox;
