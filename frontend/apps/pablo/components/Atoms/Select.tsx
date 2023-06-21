import React, { FC, ReactElement } from "react";
import {
  alpha,
  Box,
  ListSubheader,
  MenuItem,
  Typography,
  useMediaQuery,
  useTheme,
} from "@mui/material";
import { Input, InputProps } from "./Input";
import { AssetSelectionModal, BaseAsset, SearchInput } from "@/components";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import ExpandLessIcon from "@mui/icons-material/ExpandLess";
import CheckIcon from "@mui/icons-material/Check";
import CloseIcon from "@mui/icons-material/Close";
import { Option } from "../types";

export type SelectProps = {
  value?: any;
  options?: Option[];
  noBorder?: boolean;
  borderRight?: boolean;
  borderLeft?: boolean;
  minWidth?: number;
  mobileWidth?: number;
  searchable?: boolean;
  centeredLabel?: boolean;
  dropdownForceWidth?: number;
  dropdownOffsetX?: number | string;
  dropdownOffsetY?: number | string;
  dropdownModal?: boolean;
  forceHiddenLabel?: boolean;
  anchorEl?: HTMLElement | null;
  renderShortLabel?: boolean;
  renderValue?: (value: any) => ReactElement | null;
} & InputProps;

export const Select: FC<SelectProps> = ({
  value,
  setValue,
  options,
  noBorder,
  borderRight,
  noBackground = false,
  borderLeft,
  minWidth,
  mobileWidth,
  searchable,
  centeredLabel,
  dropdownForceWidth,
  dropdownOffsetX,
  dropdownOffsetY,
  dropdownModal = false,
  forceHiddenLabel,
  anchorEl = null,
  renderShortLabel = false,
  SelectProps,
  renderValue,
  ...inputProps
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
    !inputProps.disabled && setOpen(true);
  };

  const searchOptions = (options: Option[], keyword: string) => {
    return options.filter((option) => {
      let re = new RegExp(keyword, "i");
      return (
        value === option.value ||
        [option?.shortLabel, option.label].some((x) => x && re.test(x))
      );
    });
  };

  return (
    <>
      <Input
        select
        value={value}
        setValue={setValue}
        onClick={handleClick}
        SelectProps={{
          MenuProps: {
            open: open && !dropdownModal,
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
                width: dropdownForceWidth,
                marginLeft: dropdownOffsetX,
                marginTop: dropdownOffsetY,
                padding: 0,
                maxHeight: 360,
                backdropFilter: "blur(120px)",
                borderColor: alpha(
                  theme.palette.common.white,
                  theme.custom.opacity.light
                ),
                borderWidth: 1,
                borderStyle: "solid",
                [theme.breakpoints.down("sm")]: {
                  width: "inherit",
                  marginLeft: 0,
                  marginTop: 0,
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
            if (typeof renderValue === "function") {
              return renderValue(v);
            }

            const option = options!.find((option) => option.value === v);
            return (
              option && (
                <BaseAsset
                  label={
                    forceHiddenLabel
                      ? undefined
                      : (renderShortLabel && option.shortLabel) ||
                        option.label ||
                        option.value
                  }
                  icon={option.icon}
                  centeredLabel={centeredLabel}
                />
              )
            );
          },
          sx: {
            "& .MuiSelect-select": {
              pl: 1,
            },
          },
          ...SelectProps,
        }}
        sx={{
          borderRight: borderRight
            ? `1px solid ${alpha(
                theme.palette.common.white,
                theme.custom.opacity.main
              )}`
            : undefined,
          borderLeft: borderLeft
            ? `1px solid ${alpha(
                theme.palette.common.white,
                theme.custom.opacity.main
              )}`
            : undefined,
          "& .MuiOutlinedInput-root.MuiInputBase-root": {
            background: noBackground ? "none" : undefined,
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
        {...inputProps}
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
              sx={{
                display: !option.hidden ? undefined : "none",
              }}
            >
              <BaseAsset
                label={option.label || option.value}
                icon={option.icon}
                centeredLabel={centeredLabel}
              />
              {value === option.value && (
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
      <AssetSelectionModal
        forceWidth={dropdownForceWidth}
        onClose={() => setOpen(false)}
        open={open && dropdownModal}
        selected={value}
        setValue={setValue}
        anchorEl={anchorEl}
        options={options}
        searchable
      />
    </>
  );
};
