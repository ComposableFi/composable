import CheckIcon from "@mui/icons-material/Check";
import CloseIcon from "@mui/icons-material/Close";
import ExpandLessIcon from "@mui/icons-material/ExpandLess";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import { alpha, Box, ListSubheader, MenuItem, Typography, useMediaQuery, useTheme } from "@mui/material";
import React, { ReactNode } from "react";
import { BaseAsset } from "./BaseAsset";
import { Input, InputProps } from "./Input";
import { SearchInput } from "./SearchInput";

export type RequiredOption = {
  value: any;
  label?: string;
  icon?: string;
  disabled?: boolean;
};

export type Option = RequiredOption & {
  [key: string]: any;
};

export type SelectProps = {
  value?: any;
  options?: Option[];
  noBorder?: boolean;
  borderRight?: boolean;
  minWidth?: number;
  mobileWidth?: number;
  searchable?: boolean;
  centeredLabel?: boolean;
  renderValue?: (v: any) => ReactNode | undefined;
  displayEmpty?: boolean;
} & InputProps;

export const Select: React.FC<SelectProps> = ({
  value,
  options,
  noBorder,
  borderRight,
  minWidth,
  mobileWidth,
  searchable,
  centeredLabel,
  renderValue,
  displayEmpty,
  ...rest
}) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down("sm"));
  const [keyword, setKeyword] = React.useState<string>("");
  const [open, setOpen] = React.useState<boolean>(false);

  const handleKeywordChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setKeyword(event.target.value);
  };

  const handleClick = () => {
    setKeyword("");
    setOpen(true);
  };

  const searchOptions = (options: Option[], keyword: string) => {
    return options.filter(
      (option) =>
        value == option.value ||
        (option.label || option.value)
          .toLowerCase()
          .indexOf(keyword.toLowerCase()) != -1
    );
  };

  return (
    <Input
      select
      value={value}
      onClick={handleClick}
      SelectProps={{
        displayEmpty,
        MenuProps: {
          open: open,
          BackdropProps: {
            onClick: (e) => {
              e.stopPropagation();
              setOpen(false);
            },
            sx: {
              opacity: "0 !important"
            }
          },
          PaperProps: {
            onClick: (e) => e.stopPropagation(),
            sx: {
              padding: 0,
              backgroundColor: theme.palette.secondary.dark,
              [theme.breakpoints.down("sm")]: {
                top: "0 !important",
                left: "0 !important",
                bottom: 0,
                right: 0,
                maxWidth: "100%"
              }
            }
          }
        },
        IconComponent: open ? ExpandLessIcon : ExpandMoreIcon,
        renderValue: (v: any) => {
          if (typeof renderValue === "function") {
            return renderValue(v);
          }
          const option = options!.find((option) => option.value == v);
          return (
            option && (
              <BaseAsset
                label={option.label || option.value}
                icon={option.icon}
                centeredLabel={centeredLabel}
              />
            )
          );
        }
      }}
      sx={{
        borderRight: borderRight
          ? `1px solid ${alpha(
            theme.palette.common.white,
            theme.custom.opacity.main
          )}`
          : undefined,
        "& .MuiOutlinedInput-root.MuiInputBase-root": {
          borderWidth: noBorder ? 0 : undefined,
          "& .MuiOutlinedInput-notchedOutline": {
            borderWidth: noBorder ? 0 : undefined
          }
        },
        minWidth: {
          md: minWidth
        },
        width: {
          xs: mobileWidth,
          md: "100%"
        }
      }}
      {...rest}
    >
      {isMobile && (
        [
          <ListSubheader key="close">
            <Box textAlign="right">
              <CloseIcon
                sx={{
                  color: theme.palette.primary.main
                }}
                onClick={() => setOpen(false)}
              />
            </Box>
          </ListSubheader>,
          <ListSubheader key="select-option">
            <Typography variant="h6" color="text.primary" textAlign="center">
              Select option
            </Typography>
          </ListSubheader>
        ]
      )}
      {searchable && (
        <ListSubheader>
          <SearchInput
            fullWidth
            value={keyword}
            setValue={setKeyword}
            onChange={handleKeywordChange}
            onKeyDown={(e) => e.stopPropagation()}
            onClick={(e) => e.stopPropagation()}
          />
        </ListSubheader>
      )}
      {options &&
        searchOptions(options, keyword).map((option) => (
          <MenuItem
            key={option.value}
            value={option.value}
            disabled={option.disabled}
            onClick={() => setOpen(false)}
          >
            <BaseAsset
              label={option.label || option.value}
              icon={option.icon}
              centeredLabel={centeredLabel}
            />
            {value == option.value && (
              <CheckIcon
                sx={{
                  position: "absolute",
                  right: theme.spacing(3)
                }}
              />
            )}
          </MenuItem>
        ))}
    </Input>
  );
};
