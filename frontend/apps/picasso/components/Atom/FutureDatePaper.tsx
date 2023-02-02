import { FC, useMemo } from "react";
import { Paper, Typography } from "@mui/material";

export const FutureDatePaper: FC<{
  duration: string;
}> = ({ duration }) => {
  const date = useMemo(() => {
    if (!duration) return null;
    if (duration === "0") return "No lock period";
    const now = new Date();
    const target = (() =>
      new Date(now.setSeconds(now.getSeconds() + parseInt(duration))))();
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
