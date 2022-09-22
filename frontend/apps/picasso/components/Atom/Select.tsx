import React from "react";
import {
  alpha,
  ListSubheader,
  MenuItem,
  useTheme,
  useMediaQuery,
  Typography,
  Box,
} from "@mui/material";
import { InputProps, Input } from "./Input";
import { BaseAsset } from "./BaseAsset";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import ExpandLessIcon from "@mui/icons-material/ExpandLess";
import CheckIcon from "@mui/icons-material/Check";
import { SearchInput } from "./SearchInput";
import CloseIcon from "@mui/icons-material/Close";

export type Option = {
  value: any;
  label?: string;
  icon?: string;
  disabled?: boolean;
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
  ...rest
}) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down("sm"));
  const [keyword, setKeyword] = React.useState<string>("");
  const [open, setOpen] = React.useState<boolean>(false);

  const handleKeywordChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setKeyword(event.target.value);
  };

  const handleClick = (event: React.MouseEvent<HTMLInputElement>) => {
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
        MenuProps: {
          open: open,
          BackdropProps: {
            onClick: (e) => {
              e.stopPropagation();
              setOpen(false);
            },
            sx: {
              opacity: "0 !important",
            },
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
                maxWidth: "100%",
              },
            },
          },
        },
        IconComponent: open ? ExpandLessIcon : ExpandMoreIcon,
        renderValue: (v: any) => {
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
        },
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
            borderWidth: noBorder ? 0 : undefined,
          },
        },
        minWidth: {
          md: minWidth,
        },
        width: {
          xs: mobileWidth,
          md: "100%",
        },
      }}
      {...rest}
    >
      {isMobile && (
        <ListSubheader>
          <Box textAlign="right">
            <CloseIcon
              sx={{
                color: theme.palette.primary.main,
              }}
              onClick={() => setOpen(false)}
            />
          </Box>
        </ListSubheader>
      )}
      {isMobile && (
        <ListSubheader>
          <Typography variant="h6" color="text.primary" textAlign="center">
            Select option
          </Typography>
        </ListSubheader>
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
                  right: theme.spacing(3),
                }}
              />
            )}
          </MenuItem>
        ))}
    </Input>
  );
};
