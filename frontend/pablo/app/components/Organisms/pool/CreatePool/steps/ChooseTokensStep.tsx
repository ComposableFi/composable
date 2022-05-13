import { NotificationBox } from "@/components/Molecules";
import { BigNumberInput, Select } from "@/components/Atoms";
import { FormTitle } from "@/components/Organisms/FormTitle";
import { getToken, getTokenOptions, TOKEN_IDS } from "@/defi/Tokens";
import { Box, Button, useTheme, BoxProps, Grid } from "@mui/material";
import { useEffect, useState } from "react";
import { AmmId, TokenId } from "@/defi/types";
import BigNumber from "bignumber.js";
import { useAppSelector } from "@/hooks/store";
import { useDispatch } from "react-redux";
import { AMMs, getAMM, getAMMOptions } from "@/defi/AMMs";
import { setCurrentStep, setCurrentPool, setCurrentSupply } from "@/stores/defi/pool";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import { UnverifiedPoolWarningModal } from "../UnverifiedPoolWarningModal";
import { openConfirmingModal, openTransactionSettingsModal } from "@/stores/ui/uiSlice";
import FormWrapper from "../FormWrapper";
import { TransactionSettings } from "@/components/Organisms/TransactionSettings";

const labelProps = (label: string, disabled: boolean = false) => ({
  label: label,
  TypographyProps: {
    color: disabled ? "text.secondary" : undefined,
  }
})

const selectProps = (label: string, disabled: boolean = false) => ({
  noBorder: true,
  renderShortLabel: true,
  disabled: disabled,
  LabelProps: labelProps(label, disabled),
  sx: {
    "& .MuiSelect-select": {
      pl: 3,
    },
  },
});

const gridContainerProps = {
  container: true,
  mt: 4,
};

const gridItem8ColumnProps = {
  item: true,
  xs: 8,
  pr: 3,
};

const gridItem4ColumnProps = {
  item: true,
  xs: 4,
  pl: 1,
};

const ChooseTokensStep: React.FC<BoxProps> = ({
  ...boxProps
}) => {

  const theme = useTheme();
  const dispatch = useDispatch();

  const currentStep = useAppSelector((state) => state.pool.currentStep);
  const isUnverifiedPoolWarningOpen = useAppSelector((state) => state.ui.isConfirmingModalOpen);
  const {
    ammId,
    tokenId1,
    tokenId2,
    tokenWeight1,
    tokenWeight2
  } = useAppSelector((state) => state.pool.currentPool);

  const setPoolState = (property: string) => (v: AmmId | TokenId | BigNumber) => {
    dispatch(setCurrentPool({ [property]: v} ));
  };

  const [initialFunds] = useState<BigNumber>(new BigNumber(20000));
  const [currentFunds] = useState<BigNumber>(new BigNumber(340));
  const [verified] = useState<boolean>(false);
  const validAMM = ammId !== 'none';
  const valid = tokenId1 !== 'none' && tokenId2 !== 'none';
  const invalidFunds = initialFunds < currentFunds;

  useEffect(() => {
    const newTokenWeight2 = new BigNumber(100).minus(tokenWeight1);
    if (!tokenWeight2.eq(newTokenWeight2)) {
      setPoolState('tokenWeight2')(new BigNumber(100).minus(tokenWeight1));
    }
  }, [tokenWeight1]);

  useEffect(() => {
    const newTokenWeight2 = new BigNumber(100).minus(tokenWeight2);
    if (!tokenWeight2.eq(newTokenWeight2)) {
      setPoolState('tokenWeight1')(new BigNumber(100).minus(tokenWeight2));
    }
  }, [tokenWeight2]);

  const onNextClickHandler = () => {
    dispatch(setCurrentSupply({tokenId1: tokenId1, tokenId2: tokenId2}));
    dispatch( verified ? setCurrentStep(currentStep + 1) : openConfirmingModal() );
  };

  const onSettingHandler = () => {
    dispatch(openTransactionSettingsModal());
  };

  return (
    <FormWrapper {...boxProps}>
      <FormTitle
        title="Choose tokens & weights"
        onSettingHandler={onSettingHandler}
      />

      <Box mt={6}>
        <Select
          noBorder
          value={ammId}
          setValue={setPoolState("ammId")}
          options={getAMMOptions("Select")}
          LabelProps={{ label: "AMM" }}
          SelectProps={{
            sx: {
              "& .MuiSelect-select": {
                pl: 3,
              },
            },
          }}
        />
      </Box>

      <Box sx={{opacity: validAMM ? undefined : theme.custom.opacity.darker}}>
        <Grid {...gridContainerProps}>
          <Grid {...gridItem8ColumnProps}>
            <Select
              value={tokenId1}
              setValue={setPoolState("tokenId1")}
              options={getTokenOptions("Select a token")}
              {...selectProps("Token 1", !validAMM)}
            />
          </Grid>
          <Grid {...gridItem4ColumnProps}>
            <BigNumberInput
              maxValue={new BigNumber(100)}
              value={tokenWeight1}
              setValue={setPoolState("tokenWeight1")}
              LabelProps={labelProps("Weight", !validAMM)}
              referenceText="%"
              disabled={!validAMM}
            />
          </Grid>
        </Grid>

        <Grid {...gridContainerProps}>
          <Grid {...gridItem8ColumnProps}>
            <Select
              value={tokenId2}
              setValue={setPoolState("tokenId2")}
              options={getTokenOptions("Select a token")}
              {...selectProps("Token 2", !validAMM)}
            />
          </Grid>
          <Grid {...gridItem4ColumnProps}>
            <BigNumberInput
              maxValue={new BigNumber(100)}
              value={tokenWeight2}
              setValue={setPoolState("tokenWeight2")}
              referenceText="%"
              LabelProps={labelProps("Weight", !validAMM)}
              disabled={!validAMM}
            />
          </Grid>
        </Grid>
      </Box>

      {valid && invalidFunds && (
        <NotificationBox
          mt={4}
          type="warning"
          icon={<InfoOutlinedIcon color="primary" fontSize="small" />}
          mainText={
            `It’s recommended to provide new pools with at least $${initialFunds.toFormat()} in initial funds`
          }
          subText={
            `Based on your wallet balances for these tokens, the maximum amount you can fund this pool with is ~$${currentFunds.toFormat(2)}.`
          }
        />
      )}

      <Box mt={4}>
        <Button
          variant="contained"
          fullWidth
          size="large"
          disabled={!valid}
          onClick={onNextClickHandler}
        >
          Next
        </Button>
      </Box>

      <UnverifiedPoolWarningModal
        open={isUnverifiedPoolWarningOpen}
      />

      <TransactionSettings />
    </FormWrapper>

  );
};

export default ChooseTokensStep;
