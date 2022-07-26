import { Input } from "@/components/Atoms";
import { Modal, ModalProps } from "@/components/Molecules";
import { useAppDispatch, useAppSelector } from "@/hooks/store";
import { setTransactionSettings } from "@/stores/defi/settings";
import { closeTransactionSettingsModal } from "@/stores/ui/uiSlice";
import { validNumber } from "shared";
import { CloseOutlined } from "@mui/icons-material";
import {
  useTheme,
  Box,
  Typography,
  alpha,
  Button,
  FormControl,
  RadioGroup,
  FormControlLabel,
  Radio,
  Theme,
} from "@mui/material";
import BigNumber from "bignumber.js";
import React, { useMemo, useState } from "react";

const toleranceSuffix = "     %";
const toleranceOptions = [0.1, 0.5, 1];
const containerProps = (theme: Theme) => ({
  p: 4,
  borderRadius: 2,
  sx: {
    background: theme.palette.gradient.secondary,
    boxShadow: `-1px -1px ${alpha(
      theme.palette.common.white,
      theme.custom.opacity.light
    )}`,
  },
});

export type TransactionSettingsProps = {
  applyCallback?: () => void,
  closeCallback?: () => void,
} & Omit<ModalProps, "open">;

export const TransactionSettings: React.FC<TransactionSettingsProps> = ({
  applyCallback,
  closeCallback,
  ...modalProps
}) => {
  const theme = useTheme();
  const dispatch = useAppDispatch();
  const {
    transactionSettings: {tolerance, deadline},
    maxTolerance,
    minTolerance,
    maxDeadline,
    minDeadline,
  } = useAppSelector((state) => state.settings);

  const isModalOpen = useAppSelector((state) => (
    state.ui.isTransactionSettingsModalOpen
  ));

  const [isToleranceFocussed, setIsToleranceFocussed] = useState<boolean>(false);

  const [toleranceStringValue, setToleranceStringValue] = useState<string>(tolerance.toString());
  const [deadlineStringValue, setDeadlineStringValue] = useState<string>(deadline.toString());

  const toleranceSelected = (value: number) => {
    return value === tolerance
  };

  const onDeadlineChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (validNumber(e.target.value, minDeadline, maxDeadline)){
      setDeadlineStringValue(e.target.value);
    }
  };

  const onToleranceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (validNumber(e.target.value, minTolerance, maxTolerance)){
      setToleranceStringValue(e.target.value);
    }
  };

  const formattedToleranceValue = isToleranceFocussed
                                    ? toleranceStringValue
                                    : toleranceStringValue + toleranceSuffix;

  const onCloseHandler = () => {
    dispatch(closeTransactionSettingsModal());
    closeCallback?.();
  };

  const onApplySettings = () => {
    dispatch(
      setTransactionSettings({
        tolerance: new BigNumber(toleranceStringValue).toNumber(),
        deadline: Number(deadlineStringValue),
      })
    );
    dispatch(closeTransactionSettingsModal());
    applyCallback?.();
  };

  return (
    <Modal
      onClose={onCloseHandler}
      maxWidth="sm"
      open={isModalOpen}
      {...modalProps}
    >
      <Box {...containerProps(theme)}>
        <Box display="flex" alignItems="center" justifyContent="space-between">
          <Typography variant="h6">Transaction settings</Typography>
          <CloseOutlined onClick={onCloseHandler} sx={{cursor: "pointer"}} />
        </Box>
        <Box mt={6}>
          <Input
            LabelProps={{
              label: "Slippage Tolerance",
            }}
            value={formattedToleranceValue}
            onChange={onToleranceChange}
            handleOnFocus={() => setIsToleranceFocussed(true)}
            handleOnBlur={() => setIsToleranceFocussed(false)}
            customEndAdorment={
              <FormControl>
                <RadioGroup
                  row
                  sx={{ gap: 2, pr: 2, color: theme.palette.text.secondary }}
                  onChange={onToleranceChange}
                >
                  {toleranceOptions.map((value) => (
                    <FormControlLabel
                      key={value}
                      value={value.toFixed(2)}
                      control={<Radio sx={{ display: "none" }} />}
                      label={`${value.toFixed(1)} %`}
                      sx={{
                        color: toleranceSelected(value) ? theme.palette.primary.main : undefined,
                      }}
                    />
                  ))}
                </RadioGroup>
              </FormControl>
            }
          />
        </Box>
        <Box mt={4}>
          <Input
            value={deadlineStringValue}
            onChange={onDeadlineChange}
            LabelProps={{
              label: "Transaction Deadline",
            }}
            referenceText="minutes"
          />
        </Box>
        <Box mt={4}>
          <Button
            variant="contained"
            size="large"
            fullWidth
            onClick={onApplySettings}
          >
            Apply Settings
          </Button>
        </Box>
      </Box>
    </Modal>
  );
};
