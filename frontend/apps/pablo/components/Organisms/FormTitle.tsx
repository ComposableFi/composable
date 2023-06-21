import { useMobile } from "@/hooks/responsive";
import ArrowBackIosIcon from "@mui/icons-material/ArrowBackIos";
import SettingsIcon from "@mui/icons-material/Settings";
import {
  alpha,
  Box,
  BoxProps,
  Typography,
  TypographyProps,
  useTheme,
} from "@mui/material";

export type FormTitleProps = {
  title: string;
  onBackHandler?: () => any;
  onSettingHandler?: () => any;
  TitleProps?: TypographyProps;
} & BoxProps;

export const FormTitle: React.FC<FormTitleProps> = ({
  title,
  onBackHandler,
  onSettingHandler,
  TitleProps,
  ...rest
}) => {
  const theme = useTheme();
  return (
    <Box
      display="flex"
      justifyContent="space-between"
      alignItems="center"
      {...rest}
    >
      <Typography color="text.secondary" display="flex">
        {onBackHandler && (
          <ArrowBackIosIcon
            sx={{
              cursor: "pointer",
              "&:hover": {
                color: theme.palette.text.primary,
              },
              fontSize: "1rem",
            }}
            onClick={onBackHandler}
          />
        )}
      </Typography>

      <Typography variant="h6" {...TitleProps}>
        {title}
      </Typography>

      <Typography color="text.secondary" display="flex">
        {onSettingHandler && (
          <SettingsIcon
            sx={{
              cursor: "pointer",
              "&:hover": {
                color: theme.palette.text.primary,
              },
            }}
            onClick={onSettingHandler}
          />
        )}
      </Typography>
    </Box>
  );
};
