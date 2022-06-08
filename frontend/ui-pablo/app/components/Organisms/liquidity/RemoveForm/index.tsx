import { FormTitle, ValueSelector } from "@/components";
import { getToken } from "@/defi/Tokens";
import { TokenId } from "@/defi/types";
import { useAppSelector } from "@/hooks/store";
import {
  closeConfirmingModal,
  openConfirmingModal,
  setMessage,
} from "@/stores/ui/uiSlice";
import CheckIcon from "@mui/icons-material/Check";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import {
  alpha,
  Box,
  Button,
  Divider,
  Slider,
  Typography,
  useTheme,
} from "@mui/material";
import { BoxProps } from "@mui/system";
import BigNumber from "bignumber.js";
import { useRouter } from "next/router";
import { useSnackbar } from "notistack";
import { useEffect, useState } from "react";
import { useDispatch } from "react-redux";
import { YourPosition } from "../YourPosition";
import { ConfirmingModal } from "./ConfirmingModal";
import { PreviewDetails } from "./PreviewDetails";

export const RemoveLiquidityForm: React.FC<BoxProps> = ({ ...rest }) => {
  const theme = useTheme();
  const router = useRouter();
  const dispatch = useDispatch();

  const drawerWidth = theme.custom.drawerWidth.desktop;

  const {
    tokenId1,
    tokenId2,
    pooledAmount1,
    pooledAmount2,
    amount,
    share,
    price1,
    price2,
  } = useAppSelector((state) => state.pool.currentLiquidity);

  const isConfirmingModalOpen = useAppSelector(
    (state) => state.ui.isConfirmingModalOpen
  );

  const token1 = getToken(tokenId1 as TokenId);
  const token2 = getToken(tokenId2 as TokenId);

  const [percentage, setPercentage] = useState<number>(0);
  const [removeAmount1, setRemoveAmount1] = useState<BigNumber>(
    new BigNumber(0)
  );
  const [removeAmount2, setRemoveAmount2] = useState<BigNumber>(
    new BigNumber(0)
  );
  const [approved, setApproved] = useState<boolean>(false);
  const [confirmed, setConfirmed] = useState<boolean>(false);
  const message = useAppSelector((state) => state.ui.message);

  useEffect(() => {
    setRemoveAmount1(
      pooledAmount1.multipliedBy(new BigNumber(percentage / 100))
    );
    setRemoveAmount2(
      pooledAmount2.multipliedBy(new BigNumber(percentage / 100))
    );
  }, [percentage, pooledAmount1, pooledAmount2, confirmed]);

  const onBackHandler = () => {
    router.push("/pool");
  };

  const onSettingHandler = () => {
    console.log("onSettingHandler");
  };

  const onSliderChangeHandler = (_: Event, newValue: number | number[]) => {
    setPercentage(newValue as number);
  };

  const onRemoveHandler = () => {
    dispatch(openConfirmingModal());
  };

  useEffect(() => {
    confirmed && dispatch(closeConfirmingModal());
    !confirmed && dispatch(setMessage({}));
  }, [confirmed]);

  useEffect(() => {
    dispatch(setMessage({}));
  }, []);

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
        title="Remove Liquidity"
        onBackHandler={onBackHandler}
        onSettingHandler={onSettingHandler}
      />

      <Box
        display="flex"
        alignItems="center"
        justifyContent="space-between"
        mt={4}
      >
        <Typography variant="body1">Amount</Typography>
        <Typography variant="body1">Detailed</Typography>
      </Box>

      <Typography variant="h5" mt={4}>
        {percentage}%
      </Typography>

      <Box mt={8}>
        <Slider
          aria-label="percentage"
          value={percentage}
          valueLabelDisplay="auto"
          onChange={confirmed ? undefined : onSliderChangeHandler}
          min={0}
          max={100}
          marks={[
            { value: 0, label: "0" },
            { value: 25 },
            { value: 50 },
            { value: 75 },
            { value: 100, label: "100" },
          ]}
        />
        <ValueSelector
          values={[25, 50, 75, 100]}
          unit="%"
          onChangeHandler={confirmed ? undefined : setPercentage}
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
          <ExpandMoreIcon />
        </Box>
      </Box>

      <PreviewDetails
        mt={4}
        tokenId1={tokenId1 as TokenId}
        tokenId2={tokenId2 as TokenId}
        amount1={removeAmount1}
        amount2={removeAmount2}
        price1={price1}
        price2={price2}
      />

      {!confirmed && (
        <Box
          display="flex"
          justifyContent="space-between"
          alignItems="center"
          mt={4}
          gap={2}
        >
          <Box width="50%">
            <Button
              variant="contained"
              size="large"
              fullWidth
              onClick={() => setApproved(true)}
              disabled={approved}
              sx={{
                "&:disabled": {
                  backgroundColor: alpha(
                    theme.palette.success.main,
                    theme.custom.opacity.light
                  ),
                  color: theme.palette.featured.main,
                },
              }}
            >
              {approved ? (
                <>
                  <CheckIcon sx={{ marginRight: theme.spacing(2) }} />
                  Approved
                </>
              ) : (
                <>Approve</>
              )}
            </Button>
          </Box>

          <Box width="50%">
            <Button
              variant="outlined"
              size="large"
              fullWidth
              disabled={!percentage || !approved || confirmed}
              onClick={onRemoveHandler}
            >
              {!percentage ? "Enter Amount" : "Remove"}
            </Button>
          </Box>
        </Box>
      )}

      {!confirmed && (
        <>
          <Box mt={6}>
            <Divider
              sx={{
                borderColor: alpha(
                  theme.palette.common.white,
                  theme.custom.opacity.main
                ),
              }}
            />
          </Box>
          {/* <YourPosition
            noTitle={false}
            noDivider
            tokenId1={tokenId1 as TokenId}
            tokenId2={tokenId2 as TokenId}
            pooledAmount1={pooledAmount1}
            pooledAmount2={pooledAmount2}
            amount={amount}
            share={share}
            mt={6}
          /> */}
        </>
      )}

      {!confirmed && (
        <ConfirmingModal
          open={isConfirmingModalOpen}
          amount1={removeAmount1}
          amount2={removeAmount2}
          setConfirmed={setConfirmed}
        />
      )}
    </Box>
  );
};
