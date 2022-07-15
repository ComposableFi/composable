import React from "react";
import {
  Button,
  InputAdornment,
  TextField,
  Box,
  useTheme,
  Typography,
} from "@mui/material";
import { Select } from "../Atom/Select";
import { SelectProps } from "../Atom/Select";
import { Label } from "../Atom/Label";
import { InputProps } from "../Atom/Input";

export type DropdownCombinedInputProps = {
  CombinedSelectProps?: SelectProps;
} & InputProps;

export const DropdownCombinedInput: React.FC<DropdownCombinedInputProps> = ({
  LabelProps,
  alert,
  CombinedSelectProps,
  buttonLabel,
  ButtonProps,
  referenceText,
  setValue,
  children,
  InputProps,
  ...rest
}) => {
  const theme = useTheme();
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue && setValue(event.target.value);
  };
  return (
    <Box>
      {LabelProps && <Label {...LabelProps} />}
      <TextField
        fullWidth
        onChange={handleChange}
        InputProps={{
          ...InputProps,
          startAdornment: (
            <InputAdornment position="start" sx={{ marginRight: 0 }}>
              <Select
                noBorder
                borderRight
                minWidth={220}
                mobileWidth={140}
                {...CombinedSelectProps}
              />
            </InputAdornment>
          ),
          endAdornment: buttonLabel ? (
            <Button size="small" disabled={rest.disabled} {...ButtonProps}>
              {buttonLabel}
            </Button>
          ) : (
            referenceText && (
              <Typography
                variant="body2"
                color={rest.disabled ? "secondary.light" : "text.secondary"}
                whiteSpace="nowrap"
              >
                {referenceText}
              </Typography>
            )
          ),
        }}
        sx={{
          "& .MuiOutlinedInput-root": {
            color: alert ? theme.palette.warning.main : undefined,
            "& .MuiOutlinedInput-notchedOutline": {
              border: alert
                ? `1px solid ${theme.palette.warning.main}`
                : undefined,
            },
            "&.MuiInputBase-adornedStart": {
              paddingLeft: 0,
            },
          },
        }}
        {...rest}
      >
        {children}
      </TextField>
    </Box>
  );
};
