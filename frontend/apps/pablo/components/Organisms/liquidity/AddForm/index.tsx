import { DropdownCombinedBigNumberInput } from "@/components/Molecules";
import { useMobile } from "@/hooks/responsive";
import {
  Box,
  Button,
  Typography,
  useTheme,
  alpha,
  BoxProps,
} from "@mui/material";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { useRouter } from "next/router";
import { FormTitle } from "../../FormTitle";
import { useAppSelector } from "@/hooks/store";
import { useDispatch } from "react-redux";
import {
  openConfirmSupplyModal,
  openTransactionSettingsModal,
} from "@/stores/ui/uiSlice";
import { ConfirmSupplyModal } from "./ConfirmSupplyModal";
import { ConfirmingSupplyModal } from "./ConfirmingSupplyModal";
import { TransactionSettings } from "../../TransactionSettings";
import { YourPosition } from "../YourPosition";
import { PoolShare } from "./PoolShare";
import { useAddLiquidityForm } from "@/store/hooks/useAddLiquidityForm";
import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useSnackbar } from "notistack";

export const AddLiquidityForm: React.FC<BoxProps> = ({ ...rest }) => {
  const isMobile = useMobile();
  const theme = useTheme();
  const router = useRouter();
  const dispatch = useDispatch();
  const { enqueueSnackbar } = useSnackbar();

  const {
    assetList1,
    assetList2,
    setAmount,
    setToken,
    share,
    assetOneAmountBn,
    assetTwoAmountBn,
    assetOne,
    assetTwo,
    balanceOne,
    balanceTwo,
    valid,
    isValidToken1,
    isValidToken2,
    setValid,
    invalidTokenPair,
    canSupply,
    lpReceiveAmount,
    needToSelectToken,
    findPoolManually,
    pool
  } = useAddLiquidityForm();

  const isConfirmSupplyModalOpen = useAppSelector(
    (state) => state.ui.isConfirmSupplyModalOpen
  );
  const isConfirmingSupplyModalOpen = useAppSelector(
    (state) => state.ui.isConfirmingSupplyModalOpen
  );

  const onBackHandler = () => {
    router.push("/pool");
  };

  const onSettingHandler = () => {
    dispatch(openTransactionSettingsModal());
  };

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
          maxValue={balanceOne}
          setValid={setValid}
          noBorder
          value={assetOneAmountBn}
          setValue={setAmount("assetOneAmount")}
          InputProps={{
            disabled: !isValidToken1,
          }}
          buttonLabel={isValidToken1 ? "Max" : undefined}
          ButtonProps={{
            onClick: () => setAmount("assetOneAmount")(balanceOne),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          CombinedSelectProps={{
            disabled: !findPoolManually,
            value: assetOne?.network?.[DEFAULT_NETWORK_ID] || "",
            setValue: setToken("assetOne"),
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
              ...assetList1,
            ],
            borderLeft: false,
            minWidth: isMobile ? undefined : 150,
            searchable: true,
          }}
          LabelProps={{
            label: "Token 1",
            BalanceProps: isValidToken1
              ? {
                  title: <AccountBalanceWalletIcon color="primary" />,
                  balance: `${balanceOne}`,
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
          maxValue={balanceTwo}
          setValid={setValid}
          noBorder
          value={assetTwoAmountBn}
          setValue={setAmount("assetTwoAmount")}
          InputProps={{
            disabled: !isValidToken2,
          }}
          buttonLabel={isValidToken2 ? "Max" : undefined}
          ButtonProps={{
            onClick: () => setAmount("assetTwoAmount")(balanceTwo),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          CombinedSelectProps={{
            disabled: !findPoolManually,
            value: assetTwo?.network?.[DEFAULT_NETWORK_ID] || "",
            setValue: setToken("assetTwo"),
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
              ...assetList2,
            ],
            borderLeft: false,
            minWidth: isMobile ? undefined : 150,
            searchable: true,
          }}
          LabelProps={{
            label: "Token 2",
            BalanceProps: isValidToken2
              ? {
                  title: <AccountBalanceWalletIcon color="primary" />,
                  balance: `${balanceTwo}`,
                }
              : undefined,
          }}
        />
      </Box>

      {valid && !invalidTokenPair() && canSupply() && (
        <PoolShare
          baseAsset={assetOne}
          quoteAsset={assetTwo}
          price={assetOneAmountBn.div(assetTwoAmountBn)}
          revertPrice={assetTwoAmountBn.div(assetOneAmountBn)}
          share={share.toNumber()}
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

        {canSupply() && (
          <Button
            variant="contained"
            size="large"
            fullWidth
            disabled={!valid}
            onClick={() => {
              if (!pool) {
                return enqueueSnackbar("Liquidity pool for the selected token pair does not exist.", {
                  variant: "error",
                });
              }

              dispatch(openConfirmSupplyModal());
            }}
          >
            Supply
          </Button>
        )}
      </Box>

      {valid && !invalidTokenPair() && canSupply() && (
        <YourPosition
          noTitle={false}
          token1={assetOne}
          token2={assetTwo}
          pooledAmount1={assetOneAmountBn}
          pooledAmount2={assetTwoAmountBn}
          amount={lpReceiveAmount}
          share={share}
          mt={4}
        />
      )}

      <ConfirmSupplyModal
        pool={pool}
        lpReceiveAmount={lpReceiveAmount}
        priceOneInTwo={assetOneAmountBn.div(assetTwoAmountBn)}
        priceTwoInOne={assetTwoAmountBn.div(assetOneAmountBn)}
        assetOneAmount={assetOneAmountBn}
        assetTwoAmount={assetTwoAmountBn}
        assetOne={assetOne}
        assetTwo={assetTwo}
        share={share}
        open={isConfirmSupplyModalOpen}
      />

      {/* <PreviewSupplyModal
        open={isPreviewSupplyModalOpen}
        lpReceiveAmount={lpReceiveAmount}
        priceBaseInQuote={baseAmountBn.div(quoteAmountBn)}
        priceQuoteInBase={quoteAmountBn.div(baseAmountBn)}
        baseAmount={baseAmountBn}
        quoteAmount={quoteAmountBn}
        baseAsset={baseAsset}
        quoteAsset={quoteAsset}
        share={share}
      /> */}

      <ConfirmingSupplyModal
        pool={pool}
        open={isConfirmingSupplyModalOpen}
        lpReceiveAmount={lpReceiveAmount}
        priceOneInTwo={assetOneAmountBn.div(assetTwoAmountBn)}
        priceTwoInOne={assetTwoAmountBn.div(assetOneAmountBn)}
        assetOneAmount={assetOneAmountBn}
        assetTwoAmount={assetTwoAmountBn}
        assetOne={assetOne}
        assetTwo={assetTwo}
        share={share}
      />

      <TransactionSettings showSlippageSelection={false} />
    </Box>
  );
};
