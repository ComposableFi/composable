import { alpha, useTheme } from "@mui/material";
import { Settings } from "@mui/icons-material";
import { setUiState } from "@/store/ui/ui.slice";

export const SettingsModalHandler = () => {
  const theme = useTheme();
  const onSettingHandler = () => {
    setUiState({ isTransactionSettingsModalOpen: true });
  };
  return (
    <Settings
      sx={{
        color: alpha(theme.palette.common.white, theme.custom.opacity.darker),
        "&:hover": {
          color: theme.palette.common.white,
        },
        cursor: "pointer",
      }}
      onClick={onSettingHandler}
    />
  );
};
