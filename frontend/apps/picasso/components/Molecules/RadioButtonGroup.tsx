import { alpha, Box, BoxProps, Button, useTheme } from "@mui/material";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";

type RadioButtonGroupProps<T> = Omit<BoxProps, "onChange"> & {
  onChange?: (value: T) => void;
  options: {
    label: string;
    value: T;
  }[];
  value?: T;
  isMatch: (value?: T) => boolean;
  label: string;
  tooltip: string;
};

export const RadioButtonGroup = <T extends any>({
  onChange,
  options,
  isMatch,
  label,
  tooltip,
  value,
  ...rest
}: RadioButtonGroupProps<T>) => {
  const theme = useTheme();
  const optionColor = (item: T) =>
    !value || isMatch(item)
      ? theme.palette.secondary.main
      : theme.palette.secondary.light;
  const backgroundOptionColor = (item: T) =>
    isMatch(item) ? alpha(theme.palette.primary.main, 0.1) : "inherit";
  return (
    <Box display="flex" flexDirection="column" gap={1.5} {...rest}>
      <TextWithTooltip tooltip={tooltip}>{label}</TextWithTooltip>
      <Box
        display="flex"
        alignItems="center"
        justifyContent="space-between"
        width="100%"
        gap={2}
      >
        {options.map((option, index) => (
          <Button
            fullWidth
            variant="outlined"
            key={index}
            sx={{
              borderColor: optionColor(option.value),
              backgroundColor: backgroundOptionColor(option.value),
              "&:hover": {
                borderColor: optionColor(option.value),
                color: theme.palette.common.white,
              },
              "&:active": {
                borderColor: optionColor(option.value),
              },
              "&:focus": {
                borderColor: optionColor(option.value),
              },
            }}
            onClick={() => onChange?.(option.value)}
          >
            {option.label}
          </Button>
        ))}
      </Box>
    </Box>
  );
};
