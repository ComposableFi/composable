import CloseIcon from "@mui/icons-material/Close";
import {
  alpha,
  Box,
  Dialog,
  DialogProps,
  IconButton,
  useTheme,
} from "@mui/material";

export type ModalProps = DialogProps & {
  dismissible?: boolean;
};

export const Modal: React.FC<ModalProps> = ({
  dismissible = false,
  children,
  open,
  maxWidth,
  onClose,
  ...props
}) => {
  const theme = useTheme();
  return (
    <Dialog maxWidth="xl" fullScreen open={open} onClose={onClose} {...props}>
      {dismissible && (
        <IconButton
          sx={{
            position: "absolute",
            top: theme.spacing(9),
            right: theme.spacing(9),
            color: "primary.light",
            borderRadius: 1,
            "&:hover": {
              backgroundColor: alpha(
                theme.palette.primary.light,
                theme.custom.opacity.light
              ),
              color: "secondary.main",
            },
          }}
          onClick={() => onClose?.({}, "backdropClick")}
          aria-label="close"
        >
          <CloseIcon />
        </IconButton>
      )}
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
          width: "100%",
          height: "100%",
          padding: theme.spacing(4),
          [theme.breakpoints.down("sm")]: {
            padding: theme.spacing(2),
          },
        }}
      >
        <Box
          sx={{
            width: theme.breakpoints.values[maxWidth || "sm"],
            [theme.breakpoints.down("sm")]: {
              width: "100%",
            },
          }}
        >
          {children}
        </Box>
      </Box>
    </Dialog>
  );
};
