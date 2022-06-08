import { FC } from "react";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";
import { useTheme } from "@mui/material/styles";

type PositionDetailsRowProps = {
  label: string;
  description: string;
  descriptionColor?: number;
  soldOut?: boolean;
};

const PositionDetailsRow: FC<PositionDetailsRowProps> = ({
  label,
  description,
  descriptionColor = 0,
  soldOut = false,
}) => {
  const theme = useTheme();

  return (
    <Box
      height="52px"
      display="flex"
      alignItems="center"
      justifyContent="space-between"
    >
      <Typography
        sx={{ float: "left" }}
        variant="subtitle2"
        color={soldOut ? "text.secondary" : theme.palette.common.white}
      >
        {label}
      </Typography>
      <Typography
        sx={{ float: "right" }}
        variant="subtitle2"
        color={
          soldOut
            ? "text.secondary"
            : descriptionColor == 0
            ? theme.palette.common.white
            : descriptionColor > 0
            ? theme.palette.featured.lemon
            : theme.palette.error.main
        }
      >
        {description}
      </Typography>
    </Box>
  );
};

export default PositionDetailsRow;
