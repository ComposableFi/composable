import React from "react";
import { SwitchButton, Label } from "../Atoms";
import { Box, BoxProps, TooltipProps } from "@mui/material";

export type LabeledSwitchProps = {
  label: string;
  TooltipProps?: TooltipProps;
  textFirst?: boolean;
} & BoxProps;

export const LabeledSwitch: React.FC<LabeledSwitchProps> = ({
  label,
  TooltipProps,
  textFirst = true,
}) => {
  return (
    <Box
      display="flex"
      flexDirection={textFirst ? "row" : "row-reverse"}
      justifyContent={"space-between"}
      alignItems={"center"}
      sx={{ width: "fit-content" }}
      component="div"
      gap={2}
    >
      <Label 
        label={label}
        TooltipProps={TooltipProps}
        mb={0}
      />
      <SwitchButton />
    </Box>
  );
};
