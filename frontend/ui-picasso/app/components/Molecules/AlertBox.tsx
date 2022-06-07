import { Box, BoxProps, alpha, useTheme } from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";

type AlertBoxProps = {
  status?: "primary" | "secondary" | "success" | "error" | "warning" | "info";
  icon?: JSX.Element;
  underlined?: boolean;
  link?: JSX.Element;
  dismissible?: boolean;
  onClose?: () => any;
} & BoxProps;

export const AlertBox: React.FC<AlertBoxProps> = ({
  status = "success",
  icon,
  underlined,
  link,
  dismissible,
  onClose,
  children,
  ...rest
}) => {
  const theme = useTheme();
  return (
    <Box {...rest}>
      <Box
        sx={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          padding: theme.spacing(2.25, 3),
          background: alpha(
            theme.palette[status || "success"].main,
            theme.custom.opacity.light
          ),
          backdropFilter: "blur(32px)",
          borderRadius: 1,
          position: "relative",
        }}
      >
        <Box display="flex" alignItems="center">
          {icon && (
            <Box mr={2.25} display="flex">
              {icon}
            </Box>
          )}
          <Box>{children}</Box>
        </Box>
        <Box display="flex">
          {link && (
            <Box
              display="flex"
              alignItems="center"
              pt={0.875}
              sx={{
                "& a": {
                  color: theme.palette[status || "success"].main,
                },
              }}
            >
              {link}
            </Box>
          )}
          {dismissible && (
            <Box display="flex" alignItems="center" ml={3}>
              <CloseIcon
                onClick={onClose}
                sx={{
                  color: theme.palette[status || "success"].main,
                  cursor: "pointer",
                }}
              />
            </Box>
          )}
        </Box>
      </Box>
      {underlined && (
        <Box
          sx={{
            borderBottom: `2px solid ${
              theme.palette[status || "success"].main
            }`,
          }}
        />
      )}
    </Box>
  );
};
