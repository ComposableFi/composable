import { FC, useMemo } from "react";
import { Paper, Typography } from "@mui/material";

export const FutureDatePaper: FC<{
  duration: "2w" | "2m" | "1y" | "2y" | undefined;
}> = ({ duration }) => {
  const date = useMemo(() => {
    if (!duration) return null;
    const now = new Date();
    const target = (() => {
      switch (duration) {
        case "2w":
          return new Date(now.setDate(now.getDate() + 14));
        case "2m":
          return new Date(now.setMonth(now.getMonth() + 2));
        case "1y":
          return new Date(now.setFullYear(now.getFullYear() + 1));
        case "2y":
          return new Date(now.setFullYear(now.getFullYear() + 2));
      }
    })();
    return (
      target.getDate().toString().padStart(2, "0") +
      "." +
      (target.getMonth() + 1).toString().padStart(2, "0") +
      "." +
      target.getFullYear()
    );
  }, [duration]);

  return (
    <Paper>
      <Typography
        variant="body2"
        textAlign="center"
        color={date ? "text.primary" : "text.secondary"}
      >
        {date ?? "Select lock time"}
      </Typography>
    </Paper>
  );
};
