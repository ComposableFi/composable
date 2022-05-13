import { BaseAsset, Label } from "@/components/Atoms";
import { Link } from "@/components/Molecules";
import { FormTitle } from "@/components/Organisms/FormTitle";
import { Box, Button, useTheme, alpha, BoxProps, Typography, IconButton, Divider } from "@mui/material";
import { useState } from "react";
import BigNumber from "bignumber.js";
import { useAppSelector } from "@/hooks/store";
import { useDispatch } from "react-redux";
import { getTokenIdsFromPool, getTokenIdsFromSupply, setCurrentPool, setCurrentStep, setCurrentSupply } from "@/stores/defi/pool";
import FormWrapper from "../../FormWrapper";
import { getToken } from "@/defi/Tokens";
import { closeConfirmingModal, openConfirmingModal, setMessage } from "@/stores/ui/uiSlice";
import EditIcon from '@mui/icons-material/Edit';
import { ConfirmingPoolModal } from "./ConfirmingPoolModal";
import { AccessTimeRounded, OpenInNewRounded } from "@mui/icons-material";
import moment from "moment-timezone";
import { useRouter } from "next/router";

const labelProps = (
  label: string | undefined,
  balance?: string,
  fontWeight?: string | number,
) => ({
  label: label,
  mb: 0,
  TypographyProps: {
    variant: "body1",
    fontWeight: fontWeight,
  },
  BalanceProps: {
    balance: balance,
    BalanceTypographyProps: {
      variant: "body1",
      fontWeight: fontWeight,
    },
  },
} as const);

const ConfirmPoolStep: React.FC<BoxProps> = ({
  ...boxProps
}) => {

  const theme = useTheme();
  const dispatch = useDispatch();
  const router = useRouter();

  const currentStep = useAppSelector((state) => state.pool.currentStep);

  const { tokenId1, tokenId2 } = useAppSelector(getTokenIdsFromSupply);
  const {
    pooledAmount1,
    pooledAmount2,
    approvedToken1,
    approvedToken2,
  } = useAppSelector((state) => state.pool.currentSupply);

  const { tokenId1: poolTokenId1, tokenId2: poolTokenId2 } = useAppSelector(getTokenIdsFromPool);
  const {
    tokenWeight1,
    tokenWeight2,
    type,
    initialSwapFee,
    createdAt,
  } = useAppSelector((state) => state.pool.currentPool);

  const [tokenToUSD1] = useState<BigNumber>(new BigNumber(1.6));
  const [tokenToUSD2] = useState<BigNumber>(new BigNumber(1.3));
  const [isFunding, setIsFunding] = useState<boolean>(false);
  const [isConfirmed, setIsConfirmed] = useState<boolean>(false);

  const usdAmount1 = pooledAmount1.multipliedBy(tokenToUSD1);
  const usdAmount2 = pooledAmount2.multipliedBy(tokenToUSD2);

  const poolName = `${tokenWeight1}${getToken(poolTokenId1).symbol}-${tokenWeight2}${getToken(poolTokenId2).symbol}`;

  const buttonText = () => {
    if (!approvedToken1) {
      return `Approve ${getToken(tokenId1).symbol} for investing`
    };

    if (!approvedToken2) {
      return `Approve ${getToken(tokenId2).symbol} for investing`
    };

    if (isConfirmed) {
      return "View pool";
    };

    if (isFunding) {
      return "Fund pool";
    };

    return "Create pool";

  };

  const goChooseTokensStep = () => {
    dispatch(setCurrentStep(1));
  };

  const goSetFeesStep = () => {
    dispatch(setCurrentStep(2));
  };

  const onButtonClickHandler = () => {
    if (!approvedToken1) {
      dispatch(setCurrentSupply({approvedToken1: true}));
      return;
    };

    if (!approvedToken2) {
      dispatch(setCurrentSupply({approvedToken2: true}));
      return;
    };

    if (!isConfirmed) {
      dispatch(openConfirmingModal());
      setTimeout(() => {
        dispatch(closeConfirmingModal());
        if (isFunding) {
          setIsConfirmed(true);
          dispatch(setCurrentPool({createdAt: new Date().getTime()}));
        }else{
          setIsFunding(true);
          dispatch(setMessage(
            {
              title: `Created Pool confirmed`,
              text: `Create pool`,
              link: "/",
              severity: "success",
            }
          ));
        }
      }, 2000);
      return;
    };

    router.push("/pool-select");

  };

  const onBackHandler = () => {
    dispatch(setCurrentStep(currentStep - 1));
  };

  return (
    <FormWrapper {...boxProps}>
      <FormTitle
        title="Confirm new pool"
        onBackHandler={onBackHandler}
      />

      <Box mt={6}>
        <Typography variant="subtitle1">
          Tokens and initial seed liquidity
        </Typography>

        <Label
          {...labelProps(
                undefined,
                `${pooledAmount1}`,
                600
          )}
          mt={3}
        >
          <BaseAsset icon={getToken(tokenId1).icon} label={getToken(tokenId1).symbol} />
        </Label>

        <Typography variant="body2" color="text.secondary" textAlign="right" mt={0.5}>
          {`≈$${usdAmount1.toFixed(2)}`}
        </Typography>

        <Label
          {...labelProps(
                undefined,
                `${pooledAmount2}`,
                600
          )}
          mt={2}
        >
          <BaseAsset icon={getToken(tokenId2).icon} label={getToken(tokenId2).symbol} />
        </Label>

        <Typography variant="body2" color="text.secondary" textAlign="right" mt={0.5}>
          {`≈$${usdAmount2.toFixed(2)}`}
        </Typography>

        <Label
          {...labelProps("Total", `$${usdAmount1.plus(usdAmount2)}`, 600)}
          mt={2}
        />
      </Box>

      <Box mt={4}>
        <Divider
          sx={{
            borderColor: alpha(theme.palette.common.white, theme.custom.opacity.main),
          }} />
      </Box>

      <Box mt={4}>
        <Typography variant="subtitle1">
          Summary
        </Typography>
        <Box display="flex" gap={1} alignItems="center" mt={2}>
          <Label
            {...labelProps("Pool name", `${poolName}`)}
            width="100%"
          />
          <IconButton onClick={goChooseTokensStep}>
            <EditIcon color="primary" />
          </IconButton>
        </Box>

        <Label
          {...labelProps("Pool type", `${type}`)}
          mt={1}
        />

        <Box display="flex" gap={1} alignItems="center" mt={1}>
          <Label
            {...labelProps("Swap fee", `${initialSwapFee.toFixed(2)}%`)}
            width="100%"
          />
          <IconButton onClick={goSetFeesStep}>
            <EditIcon color="primary" />
          </IconButton>
        </Box>

        {isConfirmed && (
          <Box display="flex" justifyContent="space-between" alignItems="center" mt={4}>
            <Box display="flex" alignItems="center" gap={1.75}>
              <AccessTimeRounded sx={{color: "text.secondary"}} />
              <Typography variant="body2" color="text.secondary">
                {moment(createdAt).utc().format("ddd, DD MMM YYYY, hh:mm:ss [GMT]")}
              </Typography>
            </Box>
            <Box display="flex" alignItems="center" gap={1.75}>
              <Typography variant="body2" color="text.secondary">
                Etherscan
              </Typography>
              <Link href="/" target="_blank">
                <OpenInNewRounded color="primary" />
              </Link>
            </Box>
          </Box>
        )}

      </Box>

      <Box mt={isConfirmed ? 1.5 : 4}>
        <Button
          variant={isConfirmed ? "outlined" : "contained"}
          fullWidth
          size="large"
          onClick={onButtonClickHandler}
        >
          {buttonText()}
        </Button>
      </Box>

      <ConfirmingPoolModal poolName={poolName} isFunding={isFunding} />
    </FormWrapper>

  );
};

export default ConfirmPoolStep;
