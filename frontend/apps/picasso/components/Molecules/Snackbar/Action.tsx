import React from "react";
import { Close, OpenInNew } from "@mui/icons-material";
import { AlertColor, Box, IconButton } from "@mui/material";

type Props = {
  variant: AlertColor;
  onClose: () => void;
  url?: string;
  isClosable?: boolean;
};
export const MessageAction = ({ url, onClose, isClosable, variant }: Props) => (
  <Box
    sx={{
      display: "flex",
      alignItems: "center",
    }}
  >
    {url && (
      <IconButton
        disableRipple
        color={variant}
        onClick={() => {
          window.open(url, "_blank", "noopener");
        }}
      >
        <OpenInNew color={variant} />
      </IconButton>
    )}
    {isClosable && (
      <IconButton disableRipple color={variant} onClick={onClose}>
        <Close color={variant} />
      </IconButton>
    )}
  </Box>
);
