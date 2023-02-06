import { FC, useMemo } from "react";
import { Paper, PaperProps, Typography, TypographyProps } from "@mui/material";
import { formatDate } from "shared";

export const FutureDatePaper: FC<{
  duration: string;
  TextProps?: TypographyProps;
  PaperProps?: PaperProps;
  previousDate?: Date;
}> = ({ duration, TextProps, PaperProps, previousDate }) => {
  const date = useMemo(() => {
    if (!duration) return null;
    if (duration === "0") return "No lock period";
    const now = previousDate || new Date();
    return formatDate(new Date(now.getTime() + Number(duration) * 1000));
  }, [duration, previousDate]);

  return (
    <Paper {...PaperProps}>
      <Typography
        variant="body2"
        textAlign="center"
        color={date ? "text.primary" : "text.secondary"}
        {...TextProps}
      >
        {date ?? "Select lock time"}
      </Typography>
    </Paper>
  );
};
