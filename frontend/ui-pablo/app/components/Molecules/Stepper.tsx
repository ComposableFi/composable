import React from "react";
import {
  Box,
  useTheme,
  Stepper as MuiStepper,
  StepperProps as MuiStepperProps,
  Step,
  StepLabel,
  IconButton,
} from "@mui/material";
import WarningAmberOutlinedIcon from '@mui/icons-material/WarningAmberOutlined';

export type StepperProps =  {
  currentStep?: number,
  steps: string[],
  isAlertStep?: boolean,
} & MuiStepperProps;

export const Stepper: React.FC<StepperProps> = ({
  currentStep = 1,
  steps,
  isAlertStep = false,
  ...stepperProps
}) => {

  const theme = useTheme();
  return (
    <Box>
      <MuiStepper alternativeLabel {...stepperProps}>
        {steps.map((label, index) => (
          <Step key={index} active={index < currentStep} sx={{position: "relative"}}>
            <StepLabel>
              {label}
            </StepLabel>
            {isAlertStep && index === currentStep && (
              <Box
                sx={{
                  position: "absolute",
                  top: 0,
                  left: 0,
                  transform: 'translateX(-50%)',
                }}
              >
                <IconButton
                  sx={{
                    width: 32,
                    height: 32,
                    background: theme.palette.warning.main,
                  }}
                >
                  <WarningAmberOutlinedIcon fontSize="small" />
                </IconButton>
              </Box>
            )}
          </Step>
        ))}
      </MuiStepper>

    </Box>
  );
};
