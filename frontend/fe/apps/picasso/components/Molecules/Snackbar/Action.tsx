import React from "react";
import { Close, OpenInNew } from "@mui/icons-material";
import {
  AlertColor,
  Box,
  IconButton,
  Link as MuiLink,
  useTheme,
} from "@mui/material";
import Link from "next/link";

type Props = {
  variant: AlertColor;
  onClose: () => void;
  url?: string;
  isClosable?: boolean;
};
export const MessageAction = ({ url, onClose, isClosable, variant }: Props) => {
  const theme = useTheme();
  return (
    <Box
      sx={{
        display: "flex",
        alignItems: "center",
      }}
    >
      {url && (
        <Link href={url} passHref>
          <MuiLink
            sx={{
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              padding: theme.spacing(1),
            }}
            color={`text.${variant}`}
            underline="none"
            target="_blank"
            rel="noopener noreferrer"
          >
            <OpenInNew color={variant} />
          </MuiLink>
        </Link>
      )}
      {isClosable && (
        <IconButton disableRipple color={variant} onClick={onClose}>
          <Close color={variant} />
        </IconButton>
      )}
    </Box>
  );
};
