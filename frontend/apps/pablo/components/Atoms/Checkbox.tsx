import {
  alpha,
  Checkbox as MuiCheckbox,
  CheckboxProps,
  styled,
  Theme,
  useTheme,
} from "@mui/material";
import { FC } from "react";
import { Check } from "@mui/icons-material";

const AlternativeCheckbox = styled("span")(({ theme }: { theme: Theme }) => ({
  width: theme.typography.pxToRem(24),
  height: theme.typography.pxToRem(24),
  flexGrow: 0,
  display: "flex",
  flexDirection: "row",
  justifyContent: "center",
  alignItems: "center",
  padding: theme.spacing(0.5),
  borderRadius: 1,
  border: `solid 1px ${alpha(theme.palette.common.white, 0.6)}`,
}));

const AlternativeCheckedCheckbox = () => {
  const theme = useTheme();

  return (
    <AlternativeCheckbox
      sx={{
        backgroundColor: theme.palette.primary.main,
        border: `solid 1px ${theme.palette.primary.main}`,
        color: theme.palette.common.white,
      }}
    >
      <Check sx={{ fontSize: "1rem" }} />
    </AlternativeCheckbox>
  );
};

export const Checkbox: FC<CheckboxProps> = (props) => {
  return (
    <MuiCheckbox
      {...props}
      icon={<AlternativeCheckbox />}
      checkedIcon={<AlternativeCheckedCheckbox />}
    />
  );
};
