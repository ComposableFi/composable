import { useMobile } from "@/hooks/responsive";
import ArrowBackIosIcon from "@mui/icons-material/ArrowBackIos";
import SettingsIcon from '@mui/icons-material/Settings';
import { 
  Box, 
  BoxProps, 
  useTheme, 
  Button,
  ButtonProps,
} from "@mui/material";

export type ValueSelectorProps = {
  values: number[],
  unit?: string,
  onChangeHandler?: (value: number) => any,
  ButtonProps?: ButtonProps,
  disabled?: boolean,
} & BoxProps;

export const ValueSelector: React.FC<ValueSelectorProps> = ({
  unit,
  values,
  onChangeHandler,
  ButtonProps,
  disabled = false,
  ...rest
}) => {
  return (
    <Box 
      mt={4}
      display="flex"
      alignItems="center"
      justifyContent="space-between"
      {...rest}
    >
      {values.map((value) => (
        <Button
          key={value} 
          variant="outlined" 
          size="small"
          onClick={() => onChangeHandler && onChangeHandler(value)}
          fullWidth
          disabled={disabled}
          sx={{
            width: 76,
          }}
          {...ButtonProps}
        >
          {value}{unit}
        </Button>
      ))}
    </Box>
  );
}
