import { BigNumberInput } from "@/components/Atoms";
import { FormTitle } from "@/components/Organisms/FormTitle";
import {
  Box,
  Button,
  useTheme,
  alpha,
  BoxProps,
  Grid,
  Typography,
  Theme,
} from "@mui/material";
import { useMemo, useState } from "react";
import BigNumber from "bignumber.js";
import FormWrapper from "../FormWrapper";
import { TransactionSettings } from "@/components/Organisms/TransactionSettings";
import useStore from "@/store/useStore";
import { setUiState } from "@/store/ui/ui.slice";

const availableFees = [0.1, 0.3, 1.0];

const feeButtonSx = (theme: Theme, selected: boolean = false) => ({
  px: 3,
  height: 64,
  background: selected
    ? alpha(theme.palette.primary.main, theme.custom.opacity.light)
    : alpha(theme.palette.common.white, theme.custom.opacity.lighter),
});

type SetFeesStepProps = {
  onSetSimilarPoolsHandler?: () => void;
} & BoxProps;

const SetFeesStep: React.FC<SetFeesStepProps> = ({
  onSetSimilarPoolsHandler,
  ...boxProps
}) => {
  const theme = useTheme();

  const {
    createPool
  } = useStore();

  const [bestFee] = useState<BigNumber>(new BigNumber(0.3));

  const initialSwapFee = useMemo(() => {
    return new BigNumber(createPool.swapFee);
  }, [createPool.swapFee])

  const similarPoolExist = useMemo(() => {
    if (createPool.similarPool.poolId !== -1) {
      return true;
    }
    return false;
  }, [createPool.similarPool.poolId]);

  const onNextClickHandler = () => {
    similarPoolExist
      ? onSetSimilarPoolsHandler?.()
      : createPool.setSelectable({
        currentStep: createPool.currentStep + 1
      })
  };

  const setSwapFee = (fee: BigNumber) => {
    createPool.setSelectable({ "swapFee": fee.toString() })
  }

  const onBackHandler = () => {
    createPool.setSelectable({
      currentStep: createPool.currentStep - 1
    })
  };

  const onSettingHandler = () => {
    setUiState({ isTransactionSettingsModalOpen: true });
  };

  const selected = (fee: number) => initialSwapFee.eq(fee);

  return (
    <FormWrapper {...boxProps}>
      <FormTitle
        title="Set pool fees"
        onBackHandler={onBackHandler}
        onSettingHandler={onSettingHandler}
      />

      <Box mt={6}>
        <Typography variant="subtitle1" fontWeight="600">
          Initial swap fee
        </Typography>
        <Typography variant="body1" color="text.secondary" mt={2}>
          {bestFee.toFixed(2)}% is best for most weighted pools with established
          tokens. Go higher for more exotic tokens.
        </Typography>
      </Box>

      <Grid container mt={4}>
        <Grid item sm={7}>
          <Box display="flex" justifyContent="space-between">
            {availableFees.map((fee, index) => (
              <Button
                key={index}
                variant={selected(fee) ? "outlined" : "contained"}
                sx={feeButtonSx(theme, selected(fee))}
                onClick={() => setSwapFee(new BigNumber(fee))}
              >
                {`${fee.toFixed(1)}%`}
              </Button>
            ))}
          </Box>
        </Grid>
        <Grid item sm={5} pl={2}>
          <BigNumberInput
            maxValue={new BigNumber(100)}
            value={initialSwapFee}
            setValue={setSwapFee}
            referenceText="%"
            disabled
          />
        </Grid>
      </Grid>

      <Box mt={4}>
        <Button
          variant="contained"
          fullWidth
          size="large"
          onClick={onNextClickHandler}
        >
          Next
        </Button>
      </Box>

      <TransactionSettings />
    </FormWrapper>
  );
};

export default SetFeesStep;
