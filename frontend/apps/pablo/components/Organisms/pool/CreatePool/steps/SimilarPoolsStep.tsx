import { PairAsset } from "@/components/Atoms";
import { NotificationBox } from "@/components/Molecules";
import { FormTitle } from "@/components/Organisms/FormTitle";
import {
  Box,
  BoxProps,
  Button,
  Grid,
  Theme,
  Typography,
  useTheme,
} from "@mui/material";
import { useState } from "react";
import { TransactionSettings } from "@/components/Organisms/TransactionSettings";
import { useAsset } from "@/defi/hooks/assets/useAsset";
import BigNumber from "bignumber.js";
import FormWrapper from "../FormWrapper";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import useStore from "@/store/useStore";
import { setUiState } from "@/store/ui/ui.slice";

const itemBoxProps = (theme: Theme) =>
  ({
    sx: {
      background: theme.palette.gradient.secondary,
      borderRadius: 1,
      p: 2,
    },
    textAlign: "center",
  } as const);

type SimilarPoolsStepProps = {
  onCloseHandler: () => any;
} & BoxProps;

const SimilarPoolsStep: React.FC<SimilarPoolsStepProps> = ({
  onCloseHandler,
  ...boxProps
}) => {
  const theme = useTheme();

  const {
    createPool: {
      baseAsset,
      quoteAsset,
      currentStep,
      similarPool,
      setSelectable,
    }
  } = useStore();

  const [isSettingOnFlow, setIsSettingOnFlow] = useState<boolean>(false);

  const [_similarPool] = useState({
    tokenId1: baseAsset,
    tokenId2: quoteAsset,
    value: new BigNumber(similarPool.value),
    volume_24h: new BigNumber(similarPool.volume),
    initialSwapFee: new BigNumber(similarPool.fee),
  });

  const onNextClickHandler = () => {
    setIsSettingOnFlow(true);
    setUiState({ isTransactionSettingsModalOpen: true });
  };

  const onSettingHandler = () => {
    setIsSettingOnFlow(false);
    setUiState({ isTransactionSettingsModalOpen: true });
  };

  const onSettingCallback = () => {
    if (isSettingOnFlow) {
      onCloseHandler();
      setSelectable({ currentStep: currentStep + 1 });
    }
  };

  const _baseAsset = useAsset(baseAsset);
  const _quoteAsset = useAsset(quoteAsset);

  return (
    <FormWrapper {...boxProps}>
      <FormTitle
        title="Similar pools exist"
        onBackHandler={onCloseHandler}
        onSettingHandler={onSettingHandler}
      />

      <Box mt={6} display="flex" justifyContent="center">
        {_baseAsset && _quoteAsset && (
          <PairAsset
            assets={[
              { icon: _baseAsset.getIconUrl(), label: _baseAsset.getSymbol() },
              { icon: _quoteAsset.getIconUrl(), label: _quoteAsset.getSymbol() },
            ]}
            iconSize={32}
            LabelProps={{ variant: "body1" }}
            separator="/"
          />
        )}
      </Box>

      <Grid container columnSpacing={2} mt={4}>
        <Grid item xs={4}>
          <Box {...itemBoxProps(theme)}>
            <Typography variant="subtitle1">
              ${_similarPool.value.toFormat(2)}
            </Typography>
            <Typography variant="body2" color="text.secondary" mt={0.5}>
              Pool value
            </Typography>
          </Box>
        </Grid>
        <Grid item xs={4}>
          <Box {...itemBoxProps(theme)}>
            <Typography variant="subtitle1">
              ${_similarPool.volume_24h.toFormat(2)}
            </Typography>
            <Typography variant="body2" color="text.secondary" mt={0.5}>
              Vol(24h)
            </Typography>
          </Box>
        </Grid>
        <Grid item xs={4}>
          <Box {...itemBoxProps(theme)}>
            <Typography variant="subtitle1">
              {_similarPool.initialSwapFee.toFormat(2)}%
            </Typography>
            <Typography variant="body2" color="text.secondary" mt={0.5}>
              Fees
            </Typography>
          </Box>
        </Grid>
      </Grid>

      <NotificationBox
        mt={4}
        icon={<InfoOutlinedIcon color="primary" fontSize="small" />}
        mainText="Are you sure you want to continue?"
        subText="You can create your pool anyway, but you’ll have to pay pool creating gas fees."
      />

      <Box mt={4} display="flex" justifyContent="space-between" gap={3}>
        <Button
          variant="outlined"
          fullWidth
          size="large"
          onClick={onCloseHandler}
        >
          Cancel
        </Button>
        <Button
          variant="contained"
          fullWidth
          size="large"
          onClick={onNextClickHandler}
        >
          Continue anyway
        </Button>
      </Box>
      <TransactionSettings applyCallback={onSettingCallback} />
    </FormWrapper>
  );
};

export default SimilarPoolsStep;
