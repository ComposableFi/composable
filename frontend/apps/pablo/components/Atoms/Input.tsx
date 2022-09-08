import React, { ReactElement } from "react";
import {
  Button,
  ButtonProps as MuiButtonProps,
  InputAdornment,
  TextField,
  TextFieldProps,
  Typography,
  useTheme,
  Box,
  IconButton,
} from "@mui/material";
import { BaseAsset } from "./BaseAsset";
import { Label, LabelProps } from "./Label";
import ClearIcon from "@mui/icons-material/Clear";
import { Asset } from "../types";
import { PairAsset, PairAssetProps } from "./PairAsset";

export type InputProps = {
  noBackground?: boolean;
  LabelProps?: LabelProps;
  alert?: boolean;
  StartAdornmentAssetProps?: PairAssetProps;
  EndAdornmentAssetProps?: PairAssetProps;
  tokenDescription?: boolean;
  buttonLabel?: string;
  ButtonProps?: MuiButtonProps;
  referenceText?: string;
  setValue?: React.Dispatch<React.SetStateAction<any>>;
  handleOnBlur?: React.Dispatch<React.SetStateAction<any>>;
  handleOnFocus?: React.Dispatch<React.SetStateAction<any>>;
  noBorder?: boolean;
  clearable?: boolean;
  customEndAdornment?: ReactElement;
} & Omit<TextFieldProps, "label">;

export const Input: React.FC<InputProps> = ({
  noBackground = false,
  LabelProps,
  alert,
  StartAdornmentAssetProps,
  EndAdornmentAssetProps,
  buttonLabel,
  ButtonProps,
  referenceText,
  setValue,
  handleOnBlur,
  handleOnFocus,
  children,
  noBorder = true,
  clearable,
  InputProps,
  customEndAdornment,
  ...rest
}) => {
  const theme = useTheme();
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue && setValue(event.target.value);
  };
  const clear = () => {
    setValue && setValue(null);
  };
  return (
    <Box>
      {LabelProps && <Label {...LabelProps} />}
      <TextField
        fullWidth
        onChange={handleChange}
        onFocus={handleOnFocus}
        onBlur={handleOnBlur}
        InputProps={{
          ...InputProps,
          startAdornment: StartAdornmentAssetProps && (
            <InputAdornment position="start">
              <PairAsset {...StartAdornmentAssetProps} />
            </InputAdornment>
          ),
          endAdornment: (buttonLabel ||
            referenceText ||
            clearable ||
            EndAdornmentAssetProps ||
            customEndAdornment) && (
            <InputAdornment position="end">
              <Box display="flex" gap={1} pr={4}>
                <>
                  {referenceText && (
                    <Typography
                      variant="body2"
                      color={rest.disabled ? "text.disabled" : "text.secondary"}
                      whiteSpace="nowrap"
                    >
                      {referenceText}
                    </Typography>
                  )}
                  {buttonLabel && (
                    <Button
                      size="small"
                      disabled={rest.disabled}
                      {...ButtonProps}
                    >
                      {buttonLabel}
                    </Button>
                  )}

                  {clearable && rest.value && (
                    <IconButton
                      size="small"
                      disabled={rest.disabled}
                      onClick={() => clear()}
                      {...ButtonProps}
                    >
                      <ClearIcon color="primary" />
                    </IconButton>
                  )}
                  {EndAdornmentAssetProps && (
                    <PairAsset
                      {...EndAdornmentAssetProps}
                      sx={{
                        opacity: rest.disabled
                          ? theme.custom.opacity.main
                          : undefined,
                      }}
                    />
                  )}
                </>
              </Box>
              {customEndAdornment && customEndAdornment}
            </InputAdornment>
          ),
        }}
        sx={{
          "& .MuiOutlinedInput-root": {
            background: noBackground ? "none" : undefined,
            color: alert ? theme.palette.warning.main : undefined,
            "& .MuiOutlinedInput-notchedOutline": {
              borderWidth: noBorder ? 0 : undefined,
              borderColor: alert ? `${theme.palette.warning.main}` : undefined,
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
