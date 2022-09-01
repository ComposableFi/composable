import { Theme } from "@mui/material";

export const gridContainerStyle = {
  mx: "auto",
};

export const gridItemStyle = (pt: string = "2rem") => ({
  xs: 12,
  sx: { pt },
});

export const networksStyle = (theme: Theme) =>
  ({
    alignItems: "flex-end",
    flexDirection: "row",
    gap: "2rem",
    [theme.breakpoints.down("sm")]: {
      flexDirection: "column",
      alignItems: "initial",
      gap: "1.5rem",
    },
    "& > *": { flex: 1 },
  } as const);

export const swapButtonStyle = (theme: Theme) => ({
  maxWidth: "4rem",
  minWidth: "4rem",
  [theme.breakpoints.down("sm")]: {
    maxWidth: "3.5rem",
    minWidth: "3.5rem",
    alignSelf: "center",
  },
});

export const amountInputStyle = {
  "& .MuiOutlinedInput-input": {
    textAlign: "center",
  },
};
