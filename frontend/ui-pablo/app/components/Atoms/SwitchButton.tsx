import React from "react";
import { Switch, SwitchProps } from "@mui/material";

export type SwitchButtonProps = {
  disable?: boolean;
  checked?: boolean;
} & SwitchProps;

export const SwitchButton: React.FC<SwitchButtonProps> = ({
  disabled,
  checked,
  ...rest
}) => {
  return <Switch disabled={disabled} checked={checked} {...rest} />;
};
