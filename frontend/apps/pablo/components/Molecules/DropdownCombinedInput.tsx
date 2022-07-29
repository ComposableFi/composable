import React, { useEffect, useRef } from "react";
import {
  Button,
  InputAdornment,
  TextField,
  Box,
  useTheme,
  Typography,
  TypographyProps,
} from "@mui/material";
import { Select } from "../Atoms/Select";
import { SelectProps } from "../Atoms/Select";
import { Label } from "../Atoms/Label";
import { InputProps } from "../Atoms/Input";

export type DropdownCombinedInputProps = {
  CombinedSelectProps?: SelectProps;
  isAnchorable?: boolean;
  selectPosition?: "start" | "end";
  ReferenceTextProps?: TypographyProps;
} & InputProps;

export const DropdownCombinedInput: React.FC<DropdownCombinedInputProps> = ({
  LabelProps,
  alert,
  CombinedSelectProps,
  isAnchorable,
  buttonLabel,
  ButtonProps,
  referenceText,
  ReferenceTextProps,
  setValue,
  selectPosition = "end",
  children,
  InputProps,
  noBorder,
  ...rest
}) => {
  const theme = useTheme();
  const dropdownRef = useRef(null);

  const [anchorEl, setAnchorEl] = React.useState<HTMLElement | null>(
    dropdownRef.current
  );
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue && setValue(event.target.value);
  };

  useEffect(() => {
    isAnchorable && setAnchorEl(dropdownRef.current);
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <Box>
      {LabelProps && <Label {...LabelProps} />}
      <TextField
        ref={dropdownRef}
        fullWidth
        onChange={handleChange}
        InputProps={{
          ...InputProps,
          startAdornment: (
            selectPosition == 'start' && (
              <InputAdornment
                position="start"
                sx={{marginRight: 0}}
              >
                <Select
                  noBorder
                  borderRight
                  noBackground={true}
                  minWidth={220}
                  mobileWidth={CombinedSelectProps?.forceHiddenLabel ? undefined : 140}
                  {...CombinedSelectProps}
                />
              </InputAdornment>
            )
          ),
          endAdornment: (
            <InputAdornment position="end">
              {referenceText && (
                <Typography
                  variant="body1"
                  color={rest.disabled ? "text.disabled" : "text.secondary"}
                  whiteSpace="nowrap"
                  pt={0.25}
                  {...ReferenceTextProps}
                >
                  {referenceText}
                </Typography>
              )}
              {buttonLabel && (
                <Button
                  size="small"
                  disabled={rest.disabled}
                  {...ButtonProps}
                  sx={{ padding: 1 }}
                >
                  {buttonLabel}
                </Button>
              )}

              {selectPosition == "end" && (
                <Select
                  anchorEl={anchorEl}
                  noBorder
                  borderLeft
                  noBackground={true}
                  minWidth={220}
                  mobileWidth={
                    CombinedSelectProps?.forceHiddenLabel ? undefined : 140
                  }
                  {...CombinedSelectProps}
                />
              )}
            </InputAdornment>
          ),
        }}
        sx={{
          "& .MuiOutlinedInput-root": {
            color: alert ? theme.palette.warning.main : undefined,
            "& .MuiOutlinedInput-notchedOutline": {
              borderWidth: noBorder ? 0 : undefined,
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
