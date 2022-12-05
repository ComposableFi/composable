import {
  alpha,
  Box,
  BoxProps,
  Typography,
  useTheme,
} from "@mui/material";
import { BaseAsset } from "@/components";
import KeyboardArrowUpIcon from '@mui/icons-material/KeyboardArrowUp';
import KeyboardArrowDownIcon from '@mui/icons-material/KeyboardArrowDown';
import ChevronRightIcon from '@mui/icons-material/ChevronRight';
import { useState } from "react";
import { Asset } from "shared";

export type SwapRouteProps = {
  quoteAsset: Asset | undefined,
  baseAsset: Asset | undefined,
  visibleOnMount?: boolean,
} & BoxProps;

export const SwapRoute: React.FC<SwapRouteProps> = ({
  quoteAsset,
  baseAsset,
  visibleOnMount = true,
  ...boxProps
}) => {

  const [open, setOpen] = useState<boolean>(visibleOnMount);
  const theme = useTheme();

  const validTokens = !!baseAsset && !!quoteAsset;

  if (!validTokens) {
    return <></>;
  }

  return (
    <Box {...boxProps}>
      <Box display="flex" alignItems="center" justifyContent="center">
        <Box
          display="flex"
          alignItems="center"
          justifyContent="center"
          gap={1}
          py={1.375}
          px={3}
          sx={{cursor: "pointer"}}
          onClick={() => setOpen(!open)}
        >
          <Typography variant="body1" color="primary.main">
            View swap route
          </Typography>
          {
            open ? (
              <KeyboardArrowUpIcon color="primary" />
            ) : (
              <KeyboardArrowDownIcon color="primary" />
            )
          }
        </Box>
      </Box>
      {open && (
        <Box
          mt={1.25}
          display="flex"
          alignItems="center"
          justifyContent="space-between"
          gap={1}
          py={2.75}
          px={3}
          borderRadius="50%"
          sx={{
            background: alpha(
              theme.palette.common.white,
              theme.custom.opacity.lighter
            ),
          }}
        >
          <BaseAsset icon={quoteAsset?.getIconUrl()} label={quoteAsset?.getSymbol()} width="auto" />
          <ChevronRightIcon />
          <BaseAsset icon={'/tokens/pablo.svg'} label={`PABLO`} width="auto" />
          <ChevronRightIcon />
          <BaseAsset icon={baseAsset?.getIconUrl()} label={baseAsset?.getSymbol()} width="auto" />
        </Box>
      )}
    </Box>
  );
}
