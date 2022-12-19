import { NotificationBox } from "@/components/Molecules";
import { BigNumberInput, Select } from "@/components/Atoms";
import { FormTitle } from "@/components/Organisms/FormTitle";
import { Box, BoxProps, Button, Grid, useTheme } from "@mui/material";
import { useMemo, useState } from "react";
import { getAMMOptions } from "@/defi/AMMs";
import { UnverifiedPoolWarningModal } from "../UnverifiedPoolWarningModal";
import { TransactionSettings } from "@/components/Organisms/TransactionSettings";
import { useFilteredAssetListDropdownOptions } from "@/defi/hooks/assets/useFilteredAssetListDropdownOptions";
import { setUiState, useUiSlice } from "@/store/ui/ui.slice";
import { AmmId } from "@/defi/types";
import BigNumber from "bignumber.js";
import FormWrapper from "../FormWrapper";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import { TokenId } from "tokens";

const labelProps = (label: string, disabled: boolean = false) => ({
  label: label,
  TypographyProps: {
    color: disabled ? "text.secondary" : undefined,
  },
});

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

const ChooseTokensStep: React.FC<BoxProps> = ({ ...boxProps }) => {
  const theme = useTheme();
  const createPool = undefined as any;
  const { baseAsset, quoteAsset, ammId, setSelectable, currentStep } =
    createPool;

  const baseAssetList = useFilteredAssetListDropdownOptions(quoteAsset);
  const quoteAssetList = useFilteredAssetListDropdownOptions(baseAsset);

  const { isUnverifiedPoolWarningOpen } = useUiSlice();

  const setWeight = (key: string) => (weight: BigNumber) => {
    createPool.setWeights({
      [key]: weight.toString(),
    });
  };

  const _setSelectable =
    (item: "baseAsset" | "quoteAsset" | "ammId" | "swapFee") =>
    (v: TokenId | AmmId | BigNumber) => {
      setSelectable({ [item]: v });
    };

  const [initialFunds] = useState<BigNumber>(new BigNumber(20000));
  const [currentFunds] = useState<BigNumber>(new BigNumber(340));
  const [verified] = useState<boolean>(false);
  const validAMM = ammId !== "none";
  const invalidFunds = initialFunds < currentFunds;

  const baseWeight = useMemo(() => {
    return new BigNumber(createPool.weights.baseWeight);
  }, [createPool.weights.baseWeight]);

  const quoteWeight = useMemo(() => {
    return new BigNumber(createPool.weights.quoteWeight);
  }, [createPool.weights.quoteWeight]);

  const valid = useMemo(() => {
    if (baseAsset !== "none" && quoteAsset !== "none") {
      if (createPool.ammId === "balancer") {
        if (!baseWeight.lte(0) && !quoteWeight.lte(0)) {
          return true;
        }
        return false;
      }
      return true;
    }
    return false;
  }, [baseAsset, quoteAsset, createPool.ammId, baseWeight, quoteWeight]);

  const onNextClickHandler = () => {
    verified
      ? setSelectable({
          currentStep: currentStep + 1,
        })
      : setUiState({ isConfirmingModalOpen: true });
  };

  const tokenGridProps =
    ammId === "balancer"
      ? gridItem8ColumnProps
      : {
          item: true,
          xs: 12,
          pl: 0,
        };

  const onSettingHandler = () => {
    setUiState({ isTransactionSettingsModalOpen: true });
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
          setValue={_setSelectable("ammId")}
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

      <Box sx={{ opacity: validAMM ? undefined : theme.custom.opacity.darker }}>
        <Grid {...gridContainerProps}>
          <Grid {...tokenGridProps}>
            <Select
              value={baseAsset}
              setValue={_setSelectable("baseAsset")}
              options={baseAssetList}
              {...selectProps("Token 1", !validAMM)}
            />
          </Grid>
          {ammId === "balancer" ? (
            <Grid {...gridItem4ColumnProps}>
              <BigNumberInput
                maxValue={new BigNumber(100)}
                value={baseWeight}
                setValue={setWeight("baseWeight")}
                LabelProps={labelProps("Weight", !validAMM)}
                referenceText="%"
                disabled={!validAMM}
              />
            </Grid>
          ) : null}
        </Grid>

        <Grid {...gridContainerProps}>
          <Grid {...tokenGridProps}>
            <Select
              value={quoteAsset}
              setValue={_setSelectable("quoteAsset")}
              options={quoteAssetList}
              {...selectProps("Token 2", !validAMM)}
            />
          </Grid>
          {ammId === "balancer" ? (
            <Grid {...gridItem4ColumnProps}>
              <BigNumberInput
                maxValue={new BigNumber(100)}
                value={quoteWeight}
                setValue={setWeight("quoteWeight")}
                referenceText="%"
                LabelProps={labelProps("Weight", !validAMM)}
                disabled={!validAMM}
              />
            </Grid>
          ) : null}
        </Grid>
      </Box>

      {valid && invalidFunds && (
        <NotificationBox
          mt={4}
          type="warning"
          icon={<InfoOutlinedIcon color="primary" fontSize="small" />}
          mainText={`It’s recommended to provide new pools with at least $${initialFunds.toFormat()} in initial funds`}
          subText={`Based on your wallet balances for these tokens, the maximum amount you can fund this pool with is ~$${currentFunds.toFormat(
            2
          )}.`}
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

      <UnverifiedPoolWarningModal open={isUnverifiedPoolWarningOpen} />

      <TransactionSettings />
    </FormWrapper>
  );
};

export default ChooseTokensStep;
