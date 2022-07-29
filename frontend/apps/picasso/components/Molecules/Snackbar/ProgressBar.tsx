import { AlertColor, LinearProgress, LinearProgressProps } from "@mui/material";
import * as React from "react";
import { FC } from "react";

type Props = LinearProgressProps & {
  color: AlertColor;
  progress: number;
};

export const ProgressBar: FC<Props> = ({ progress, color, ...rest }) => (
  <LinearProgress
    {...rest}
    color={color}
    variant="determinate"
    value={progress}
  />
);
