import Image from "next/image";
import { Box, Typography, Button, useTheme } from "@mui/material";

type ConnectButtonProps = {
  onClick: () => void;
  imageSrc: string;
  imageAlt: string;
};

export const ConnectButton: React.FC<ConnectButtonProps> = ({
  children,
  onClick,
  imageSrc,
  imageAlt,
}) => {
  const theme = useTheme();

  return (
    <Button
      variant="outlined"
      color="primary"
      onClick={onClick}
      sx={{
        cursor: "pointer",
      }}
    >
      <Box
        sx={{
          display: "grid",
          gridTemplateColumns: "24px auto 24px",
          minWidth: theme.spacing(31.25),
        }}
      >
        <Box sx={{ height: 24 }}>
          <Image src={imageSrc} width="24" height="24" alt={imageAlt} />
        </Box>
        <Box>
          <Typography variant="button">{children}</Typography>
        </Box>
      </Box>
    </Button>
  );
};
