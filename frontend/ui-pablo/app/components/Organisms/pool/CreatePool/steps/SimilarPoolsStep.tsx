import { PairAsset } from "@/components/Atoms";
import { NotificationBox } from "@/components/Molecules";
import { FormTitle } from "@/components/Organisms/FormTitle";
import {
  Box,
  Button,
  useTheme,
  BoxProps,
  Grid,
  Typography,
  Theme,
} from "@mui/material";
import { useState } from "react";
import BigNumber from "bignumber.js";
import { useDispatch } from "react-redux";
import FormWrapper from "../FormWrapper";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import { TransactionSettings } from "@/components/Organisms/TransactionSettings";
import { openTransactionSettingsModal } from "@/stores/ui/uiSlice";
import useStore from "@/store/useStore";
import { getAsset } from "@/defi/polkadot/Assets";

const itemBoxProps = (theme: Theme) =>
  ({
    sx: {
      background: theme.palette.gradient.secondary,
      borderRadius: 0.66,
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
  const dispatch = useDispatch();

  const {
    pools: {
      createPool: {
        baseAsset,
        quoteAsset,
        currentStep,
        similarPool,
        setSelectable,
      },
    },
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
    dispatch(openTransactionSettingsModal());
  };

  const onSettingHandler = () => {
    setIsSettingOnFlow(false);
    dispatch(openTransactionSettingsModal());
  };

  const onSettingCallback = () => {
    if (isSettingOnFlow) {
      onCloseHandler();
      setSelectable({currentStep: currentStep + 1 })
    }
  };

  const token1 = baseAsset !== "none" ? getAsset(baseAsset) : null;
  const token2 = quoteAsset !== "none" ? getAsset(quoteAsset) : null;

  return (
    <FormWrapper {...boxProps}>
      <FormTitle
        title="Similar pools exist"
        onBackHandler={onCloseHandler}
        onSettingHandler={onSettingHandler}
      />

      <Box mt={6} display="flex" justifyContent="center">
        {token1 && token2 && (
          <PairAsset
            assets={[
              { icon: token1.icon, label: token1.symbol },
              { icon: token2.icon, label: token2.symbol },
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
