import CloseIcon from "@mui/icons-material/Close";
import CheckIcon from "@mui/icons-material/Check";
import {
  alpha,
  Box,
  IconButton,
  MenuItem,
  MenuList,
  Popover,
  PopoverProps,
  Typography,
  useTheme,
} from "@mui/material";
import {
  ModalProps,
  Modal,
} from "../Molecules";
import {
  SearchInput,
  BaseAsset,
} from "./index"
import React from "react";
import { Option } from "../types";
import { SelectAllOutlined } from "@mui/icons-material";

export type AssetSelectionModalProps = {
  selected?: number | string;
  options?: Option[];
  searchable?: boolean;
  centeredLabel?: boolean;
  setValue?: React.Dispatch<React.SetStateAction<any>>,
  forceWidth?: number | string,
} & PopoverProps;

export const AssetSelectionModal: React.FC<AssetSelectionModalProps> = ({
  selected,
  options,
  searchable,
  centeredLabel,
  setValue,
  forceWidth,
  onClose,
  anchorEl,
  ...rest
}) => {
  const theme = useTheme();
  const [keyword, setKeyword] = React.useState<string>("");

  const handleKeywordChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setKeyword(event.target.value);
  };

  const searchOptions = (options: Option[], keyword: string) => {
    return options.filter(
      (option) => {
        let re = new RegExp(keyword, 'i');
        return selected === option.value
                || [option?.shortLabel, option.label].some(x => x && re.test(x))
      }
    );
  };

  const handleSelection = (value: number | string) => {
    setValue && setValue(value);
    onClose?.({}, 'backdropClick');
  }

  return (
    <Popover
      anchorEl={anchorEl}
      onClose={onClose}
      BackdropProps={{
        sx: {
          background: anchorEl ? 'transparent' : undefined,
        }
      }}
      PaperProps={{
        sx: {
          ...(!anchorEl ? {
            transform: 'translate(-50%, -50%) !important',
            top: '50% !important',
            left: '50% !important',
          }: {}),
          padding: 0,
          backdropFilter: 'blur(48px)',
          borderColor: alpha(theme.palette.common.white, theme.custom.opacity.light),
          borderWidth: 1,
          borderStyle: 'solid',
        }
      }}
      anchorOrigin={{
        vertical: "bottom",
        horizontal: "center",
      }}
      transformOrigin={{
        vertical: "top",
        horizontal: "center",
      }}
      {...rest}
    >
      <Box
        sx={{
          width: anchorEl ? forceWidth : 550,
          [theme.breakpoints.down('sm')]: {
            width: '100%',
          },
          borderRadius: 1,
          paddingTop: theme.spacing(anchorEl ? 2: 3)
        }}
      >
        {(!anchorEl || searchable) && (
          <Box
            paddingX={theme.spacing(anchorEl ? 2: 4)}
          >
            {!anchorEl && (
              <Box
                display="flex"
                alignItems="center"
                justifyContent="space-between"
                mb={searchable ? 3 : undefined}
              >
                <Typography variant="h6">
                  Select a Token
                </Typography>
                <IconButton
                  onClick={() => onClose?.({}, 'backdropClick')}
                >
                  <CloseIcon />
                </IconButton>
              </Box>
            )}
            {searchable && (
              <Box>
                <SearchInput
                  fullWidth
                  value={keyword}
                  setValue={setKeyword}
                  onChange={handleKeywordChange}
                  onKeyDown={(e) => e.stopPropagation()}
                  onClick={(e) => e.stopPropagation()}
                />
              </Box>
            )}
          </Box>
        )}

        <Box
          mt={anchorEl ? 2: 3}
          height={anchorEl ? 270 : 384}
          overflow="auto"
        >
          <MenuList>
            {
              options && (
                searchOptions(options, keyword).map((option) => (
                  <MenuItem
                    key={option.value}
                    value={option.value}
                    disabled={option.disabled}
                    selected={selected === option.value}
                    onClick={() => handleSelection(option.value)}
                    sx={{
                      display: !option.hidden ? undefined : 'none',
                      height: 64,
                    }}
                  >
                    <BaseAsset
                      label={option.label || option.value}
                      icon={option.icon}
                      centeredLabel={centeredLabel}
                    />
                    {selected == option.value && (
                      <CheckIcon
                        sx={{
                          position: "absolute",
                          right: theme.spacing(3),
                        }}
                      />
                    )}
                  </MenuItem>
                ))
              )
            }
          </MenuList>
        </Box>
      </Box>
    </Popover>
  );
}
