import { Typography, Box, useTheme, alpha, BoxProps } from "@mui/material";
import { CloseOutlined } from "@mui/icons-material";

export type MessageBoxProps = {
  title: string;
  message: string;
  onClose: React.MouseEventHandler;
} & BoxProps;

export const MessageBox: React.FC<MessageBoxProps> = ({
  title,
  message,
  onClose,
  ...rest
}) => {
  const theme = useTheme();
  return (
    <Box
      sx={{
        mt: 8,
        display: "flex",
        flexDirection: "column",
        padding: 4,
        backgroundImage:
          "linear-gradient(to bottom, #010632 -16%, #04031a 83%)",
        borderRadius: 1,
      }}
      {...rest}
    >
      <Box display="flex" justifyContent="space-between">
        <Typography variant="h6" gutterBottom>
          {title}
        </Typography>
        <CloseOutlined
          onClick={onClose}
          sx={{ color: theme.palette.primary.main }}
        />
      </Box>
      <Typography
        variant="body2"
        color={alpha(theme.palette.common.white, theme.custom.opacity.darker)}
      >
        {message}
      </Typography>
    </Box>
  );
};

MessageBox.defaultProps = {
  title: "",
  message: "",
};
