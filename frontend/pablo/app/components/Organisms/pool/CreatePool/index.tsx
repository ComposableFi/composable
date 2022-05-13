import { Link, Stepper } from "@/components/Molecules";
import { Box, useTheme, BoxProps } from "@mui/material";
import { useRouter } from "next/router";
import { useAppSelector } from "@/hooks/store";
import { useDispatch } from "react-redux";
import { setMessage } from "@/stores/ui/uiSlice";
import ChooseTokensStep from "./steps/ChooseTokensStep";
import SetFeesStep from "./steps/SetFeesStep";
import { useState } from "react";
import SimilarPoolsStep from "./steps/SimilarPoolsStep";
import SetLiquidityStep from "./steps/SetLiquidityStep";
import ConfirmPoolStep from "./steps/ConfirmPool";
import { Calculate, OpenInNewRounded } from "@mui/icons-material";

const steps = ["Choose tokens", "Set fees", "Set liquidity", "Confirm pool"];

export const CreatePool: React.FC<BoxProps> = ({ ...boxProps }) => {
  const theme = useTheme();
  const router = useRouter();
  const dispatch = useDispatch();

  const drawerWidth = theme.custom.drawerWidth.desktop;

  const currentStep = useAppSelector((state) => state.pool.currentStep);
  const [isSimilarPoolsStep, setIsSimilarPoolsStep] = useState<boolean>(false);
  const message = useAppSelector((state) => state.ui.message);

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
