import {
  Box,
  Button,
  ButtonProps as MuiButtonProps,
  InputAdornment,
  TextField,
  TextFieldProps,
  Typography,
  useTheme,
} from "@mui/material";
import { TokenId } from "tokens";
import { TokenAsset } from "./TokenAsset";
import { Label, LabelProps as MuiLabelProps } from "./Label";
import { BaseAsset } from "./BaseAsset";
import { ChangeEvent, Dispatch, FC, SetStateAction } from "react";

export type InputProps = {
  LabelProps?: MuiLabelProps;
  alert?: boolean;
  tokenId?: TokenId;
  icon?: string;
  tokenDescription?: boolean;
  buttonLabel?: string;
  ButtonProps?: MuiButtonProps;
  referenceText?: string;
  setValue?: Dispatch<SetStateAction<any>>;
  noBorder?: boolean;
} & Omit<TextFieldProps, "label">;

export const Input: FC<InputProps> = ({
  LabelProps,
  alert,
  tokenId,
  icon,
  tokenDescription = true,
  buttonLabel,
  ButtonProps,
  referenceText,
  setValue,
  children,
  noBorder,
  InputProps,
  ...rest
}) => {
  const theme = useTheme();
  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    setValue && setValue(event.target.value);
  };
  return (
    <Box>
      {LabelProps && <Label {...LabelProps} />}
      <TextField
        variant="outlined"
        fullWidth
        onChange={handleChange}
        InputProps={{
          startAdornment: tokenId ? (
            <InputAdornment position="start">
              <TokenAsset iconOnly={!tokenDescription} />
            </InputAdornment>
          ) : icon ? (
            <InputAdornment position="start">
              <BaseAsset icon={icon} />
            </InputAdornment>
          ) : undefined,
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
          ...InputProps,
        }}
        sx={{
          "& .MuiOutlinedInput-root": {
            color: alert ? theme.palette.warning.main : undefined,
            "& .MuiOutlinedInput-notchedOutline": {
              borderWidth: noBorder ? 0 : 1,
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
