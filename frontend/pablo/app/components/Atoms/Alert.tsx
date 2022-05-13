import { FC } from 'react';
import {
  Alert as MuiAlert, 
  AlertProps as MuiAlertProps, 
} from "@mui/material";
import Typography from '@mui/material/Typography';
import { useTheme } from "@mui/material/styles";
import { AlertTitle, Box } from '@mui/material';
import CheckCircleOutlinedIcon from '@mui/icons-material/CheckCircleOutlined';

export type AlertProps = {
  alertTitle?: string;
  alertText: string;
  underlined?: boolean;
  centered?: boolean;
} & MuiAlertProps;

export const Alert: FC<AlertProps> = ({
  severity = 'info',
  alertTitle,
  alertText,
  underlined,
  centered = false,
  action,
  ...rest
}) => {
  const theme = useTheme();

  return (
    <Box
      width="100%"
      position="relative"
    >
      <MuiAlert
        variant="filled"
        severity={severity}
        {...rest}
        sx={{
          width: "100%",
          "& .MuiSvgIcon-root": {
            fill: theme.palette[severity].main,
          },
        }}
        iconMapping={{
          success: <CheckCircleOutlinedIcon />,
        }}
      >
        <Box
          display="flex"
          alignItems="center"
          justifyContent="space-between"
          width="100%"
          pr={3}
        >
          <Box width="100%" textAlign={centered ? "center" : undefined}>
            {alertTitle && <AlertTitle>{alertTitle}</AlertTitle>}
            <Typography
              sx={{
                color: theme.palette.common.white,
                fontFamily: theme.custom.fontFamily.primary,
              }}
              variant="body2"
            >
              {alertText}
            </Typography>
          </Box>
          {action && action} 
        </Box>
      </MuiAlert>
      {underlined && (
        <Box
          borderBottom={`2px solid ${theme.palette[severity].main}`}
          marginX={theme.spacing(4)}
        ></Box>)}
    </Box>
  );
};