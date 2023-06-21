import { Stepper } from "@/components/Molecules";
import { Box, BoxProps, useTheme } from "@mui/material";
import ChooseTokensStep from "./steps/ChooseTokensStep";
import SetFeesStep from "./steps/SetFeesStep";
import { useState } from "react";
import SimilarPoolsStep from "./steps/SimilarPoolsStep";
import SetLiquidityStep from "./steps/SetLiquidityStep";
import ConfirmPoolStep from "./steps/ConfirmPool";

const steps = ["Choose tokens", "Set fees", "Set liquidity", "Confirm pool"];

export const CreatePool: React.FC<BoxProps> = ({ ...boxProps }) => {
  const theme = useTheme();
  const currentStep = 1 as number; // TODO implement steps once pool creation is required
  const [isSimilarPoolsStep, setIsSimilarPoolsStep] = useState<boolean>(false);

  const isChooseTokensStep = currentStep === 1;
  const isSetFeesStep = currentStep === 2 && !isSimilarPoolsStep;
  const isSetLiquidityStep = currentStep === 3;
  const isConfirmPoolStep = currentStep === 4;

  return (
    <Box margin="auto" width={{ md: 550 }} mb={25} {...boxProps}>
      <Box mb={7}>
        <Stepper
          currentStep={currentStep}
          steps={steps}
          isAlertStep={isSimilarPoolsStep}
        />
      </Box>

      {isChooseTokensStep && <ChooseTokensStep />}

      {isSetFeesStep && (
        <SetFeesStep
          onSetSimilarPoolsHandler={() => setIsSimilarPoolsStep(true)}
        />
      )}

      {isSimilarPoolsStep && (
        <SimilarPoolsStep onCloseHandler={() => setIsSimilarPoolsStep(false)} />
      )}

      {isSetLiquidityStep && <SetLiquidityStep />}

      {isConfirmPoolStep && <ConfirmPoolStep />}
    </Box>
  );
};
