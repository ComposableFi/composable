import { DropdownCombinedBigNumberInput } from "@/components/Molecules";
import { getToken, TOKEN_IDS } from "@/defi/Tokens";
import { useMobile } from "@/hooks/responsive";
import { Box, Button, Typography, useTheme, alpha, BoxProps } from "@mui/material";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { useRouter } from "next/router";
import { FormTitle } from "../../FormTitle";
import { useEffect, useState } from "react";
import { TokenId } from "@/defi/types";
import BigNumber from "bignumber.js";
import { PoolShare } from "./PoolShare";
import { YourPosition } from "../YourPosition";
import { useAppSelector } from "@/hooks/store";
import { useDispatch } from "react-redux";
import {
  openConfirmSupplyModal,
  openTransactionSettingsModal,
} from "@/stores/ui/uiSlice";
import { setCurrentSupply } from "@/stores/defi/pool";
import { ConfirmSupplyModal } from "./ConfirmSupplyModal";
import { PreviewSupplyModal } from "./PreviewSupplyModal";
import { ConfirmingSupplyModal } from "./ConfirmingSupplyModal";
import { TransactionSettings } from "../../TransactionSettings";

export const AddLiquidityForm: React.FC<BoxProps> = ({ ...rest }) => {
  const isMobile = useMobile();
  const theme = useTheme();
  const router = useRouter();
  const dispatch = useDispatch();

  const [valid, setValid] = useState<boolean>(false);

  const isConfirmSupplyModalOpen = useAppSelector(
    (state) => state.ui.isConfirmSupplyModalOpen
  );
  const isPreviewSupplyModalOpen = useAppSelector(
    (state) => state.ui.isPreviewSupplyModalOpen
  );
  const isConfirmingSupplyModalOpen = useAppSelector(
    (state) => state.ui.isConfirmingSupplyModalOpen
  );
  
  const {
    tokenId1,
    tokenId2,
    balance1,
    balance2,
    pooledAmount1,
    pooledAmount2,
    approvedToken1,
    approvedToken2,
    confirmed,
    amount,
    share,
  } = useAppSelector((state) => state.pool.currentSupply);

  const setPooledToken1 = (v: BigNumber) => {
    dispatch(setCurrentSupply({ pooledAmount1: v }));
  };

  const setPooledToken2 = (v: BigNumber) => {
    dispatch(setCurrentSupply({ pooledAmount2: v }));
  };

  const setToken1 = (v: TokenId) => {
    dispatch(setCurrentSupply({ tokenId1: v }));
  };

  const setToken2 = (v: TokenId) => {
    dispatch(setCurrentSupply({ tokenId2: v }));
  };

  const isValidToken1 = tokenId1 != 'none';
  const isValidToken2 = tokenId2 != 'none';

  const needToSelectToken = () => {
    return !isValidToken1 && !isValidToken2;
  };

  const invalidTokenPair = () => {
    return (
      (!isValidToken1 && isValidToken2) ||
      (isValidToken1 && !isValidToken2)
    );
  };

  const needToApproveToken1 = () => {
    return !needToSelectToken() && !invalidTokenPair() && !approvedToken1;
  };

  const needToApproveToken2 = () => {
    return (
      !needToSelectToken() &&
      !invalidTokenPair() &&
      approvedToken1 &&
      !approvedToken2
    );
  };

  const canSupply = () => {
    return approvedToken1 && approvedToken2;
  };

  const onBackHandler = () => {
    router.push("/pool");
  };

  const onSettingHandler = () => {
    dispatch(openTransactionSettingsModal());
  };

  useEffect(() => {
    setValid(true);
    tokenId1 == "none" && setValid(false);
    tokenId2 == "none" && setValid(false);
    new BigNumber(0).eq(pooledAmount1) && setValid(false);
    new BigNumber(0).eq(pooledAmount2) && setValid(false);
    balance1.lt(pooledAmount1) && setValid(false);
    balance2.lt(pooledAmount2) && setValid(false);
    dispatch(setCurrentSupply({ confirmed: false }));
  }, [tokenId1, tokenId2, pooledAmount1, pooledAmount2, balance1, balance2]);

  useEffect(() => {
    dispatch(setCurrentSupply({ approvedToken1: false }));
  }, [tokenId1]);

  useEffect(() => {
    dispatch(setCurrentSupply({ approvedToken2: false }));
  }, [tokenId2]);

  return (
    <Box
      borderRadius={1.33}
      margin="auto"
      sx={{
        width: 550,
        padding: theme.spacing(4),
        [theme.breakpoints.down("sm")]: {
          width: "100%",
          padding: theme.spacing(2),
        },
        background: theme.palette.gradient.secondary,
        boxShadow: `-1px -1px ${alpha(
          theme.palette.common.white,
          theme.custom.opacity.light
        )}`,
      }}
      {...rest}
    >
      <FormTitle
        title="Add liquidity"
        onBackHandler={onBackHandler}
        onSettingHandler={onSettingHandler}
      />

      <Typography variant="subtitle1" textAlign="center" mt={4}>
        Use this tool to add tokens to the liquidity pool.
      </Typography>

      <Box mt={4}>
        <DropdownCombinedBigNumberInput
          maxValue={balance1}
          setValid={setValid}
          noBorder
          value={pooledAmount1}
          setValue={setPooledToken1}
          InputProps={{
            disabled: !isValidToken1,
          }}
          buttonLabel={isValidToken1 ? "Max" : undefined}
          ButtonProps={{
            onClick: () => setPooledToken1(balance1),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          CombinedSelectProps={{
            value: tokenId1,
            setValue: setToken1,
            dropdownModal: true,
            forceHiddenLabel: isMobile ? true : false,
            options: [
              {
                value: "none",
                label: "Select",
                icon: undefined,
                disabled: true,
                hidden: true,
              },
              ...TOKEN_IDS.map((tokenId) => ({
                value: tokenId,
                label: getToken(tokenId).symbol,
                icon: getToken(tokenId).icon,
              })),
            ],
            borderLeft: false,
            minWidth: isMobile ? undefined : 150,
            searchable: true,
          }}
          LabelProps={{
            label: "Token 1",
            BalanceProps:
              isValidToken1
                ? {
                    title: <AccountBalanceWalletIcon color="primary" />,
                    balance: `${balance1}`,
                  }
                : undefined,
          }}
        />
      </Box>

      <Box mt={4} textAlign="center">
        <Box
          width={56}
          height={56}
          borderRadius={9999}
          display="flex"
          border={`2px solid ${theme.palette.primary.main}`}
          justifyContent="center"
          alignItems="center"
          margin="auto"
        >
          <Typography variant="h5">+</Typography>
        </Box>
      </Box>

      <Box mt={4}>
        <DropdownCombinedBigNumberInput
          maxValue={balance2}
          setValid={setValid}
          noBorder
          value={pooledAmount2}
          setValue={setPooledToken2}
          InputProps={{
            disabled: !isValidToken2,
          }}
          buttonLabel={isValidToken2 ? "Max" : undefined}
          ButtonProps={{
            onClick: () => setPooledToken2(balance2),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          CombinedSelectProps={{
            value: tokenId2,
            setValue: setToken2,
            dropdownModal: true,
            forceHiddenLabel: isMobile ? true : false,
            options: [
              {
                value: "none",
                label: "Select",
                icon: undefined,
                disabled: true,
                hidden: true,
              },
              ...TOKEN_IDS.map((tokenId) => ({
                value: tokenId,
                label: getToken(tokenId).symbol,
                icon: getToken(tokenId).icon,
              })),
            ],
            borderLeft: false,
            minWidth: isMobile ? undefined : 150,
            searchable: true,
          }}
          LabelProps={{
            label: "Token 2",
            BalanceProps:
              isValidToken2
                ? {
                    title: <AccountBalanceWalletIcon color="primary" />,
                    balance: `${balance2}`,
                  }
                : undefined,
          }}
        />
      </Box>

      {isValidToken1 && isValidToken2 && (
        <PoolShare
          tokenId1={tokenId1 as TokenId}
          tokenId2={tokenId2 as TokenId}
          price={0.1}
          revertPrice={10}
          share={3.3}
        />
      )}

      <Box mt={4}>
        {needToSelectToken() && (
          <Button variant="contained" size="large" fullWidth disabled>
            Select tokens
          </Button>
        )}

        {invalidTokenPair() && (
          <Button variant="contained" size="large" fullWidth disabled>
            Invalid pair
          </Button>
        )}

        {needToApproveToken1() && (
          <Button
            variant="contained"
            size="large"
            fullWidth
            disabled={confirmed}
            onClick={() => dispatch(setCurrentSupply({ approvedToken1: true }))}
          >
            {`Approve ${getToken(tokenId1 as TokenId).symbol}`}
          </Button>
        )}

        {needToApproveToken2() && (
          <Button
            variant="contained"
            size="large"
            fullWidth
            disabled={confirmed}
            onClick={() => dispatch(setCurrentSupply({ approvedToken2: true }))}
          >
            {`Approve ${getToken(tokenId2 as TokenId).symbol}`}
          </Button>
        )}

        {canSupply() && (
          <Button
            variant="contained"
            size="large"
            fullWidth
            disabled={!valid || confirmed}
            onClick={() => dispatch(openConfirmSupplyModal())}
          >
            {confirmed ? `Enter an amount` : `Supply`}
          </Button>
        )}
      </Box>

      {valid && canSupply() && !confirmed && (
        <YourPosition
          noTitle={false}
          tokenId1={tokenId1 as TokenId}
          tokenId2={tokenId2 as TokenId}
          pooledAmount1={pooledAmount1}
          pooledAmount2={pooledAmount2}
          amount={amount}
          share={share}
          mt={4}
        />
      )}

      <ConfirmSupplyModal open={isConfirmSupplyModalOpen} />

      <PreviewSupplyModal open={isPreviewSupplyModalOpen} />

      <ConfirmingSupplyModal open={isConfirmingSupplyModalOpen} />

      <TransactionSettings />
    </Box>
  );
};
