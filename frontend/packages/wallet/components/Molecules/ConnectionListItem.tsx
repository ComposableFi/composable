import { Box, Button, Typography, useTheme } from "@mui/material";
import { NetworkId } from "../../types";
import { ConnectorType } from "bi-lib";
import { SupportedWalletId } from "substrate-react";
import Image from "next/image";

export const ConnectionListItem = ({
  icon,
  name,
  onClick,
  id,
}: {
  id: SupportedWalletId | ConnectorType | NetworkId;
  icon: string;
  name: string;
  onClick?: Function;
}) => {
  const theme = useTheme();
  return (
    <Button
      sx={{
        mt: "2rem",
        justifyContent: "flex-start",
      }}
      variant="outlined"
      color="primary"
      size="large"
      fullWidth
      onClick={() => {
        onClick?.(id);
      }}
    >
      <Box
        sx={{ marginLeft: theme.spacing(1.75), marginTop: theme.spacing(0.5) }}
      >
        <Image src={icon} width="24" height="24" alt={name} />
      </Box>
      <Box sx={{ justifyContent: "center", flexGrow: 1 }}>
        <Typography variant="button">{name}</Typography>
      </Box>
    </Button>
  );
};
