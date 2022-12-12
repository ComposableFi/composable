import { DropdownCombinedBigNumberInput } from "@/components/Molecules";
import { useMobile } from "@/hooks/responsive";
import { Box, BoxProps, Button, Typography, useTheme } from "@mui/material";
import AccountBalanceWalletIcon from "@mui/icons-material/AccountBalanceWallet";
import { useRouter } from "next/router";
import { FormTitle } from "../../FormTitle";
import { ConfirmSupplyModal } from "./ConfirmSupplyModal";
import { ConfirmingSupplyModal } from "./ConfirmingSupplyModal";
import { TransactionSettings } from "../../TransactionSettings";
import { YourPosition } from "../YourPosition";
import { PoolShare } from "./PoolShare";
import { useAddLiquidityForm } from "@/defi/hooks";
import { useSnackbar } from "notistack";
import BigNumber from "bignumber.js";
import { useState } from "react";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { setUiState, useUiSlice } from "@/store/ui/ui.slice";

export const AddLiquidityForm: React.FC<BoxProps> = ({ ...rest }) => {
  const isMobile = useMobile();
  const theme = useTheme();
  const router = useRouter();
  const { enqueueSnackbar } = useSnackbar();

  const {
    assetList1,
    assetList2,
    setAmount,
    setToken,
    share,
    assetOneAmount,
    assetTwoAmount,
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
    spotPrice,
    pool,
  } = useAddLiquidityForm();

  const {

      isConfirmSupplyModalOpen,
      isConfirmingSupplyModalOpen
    
  } = useUiSlice();

  const [manualUpdateMode, setManualUpdateMode] = useState<1 | 2>(1);

  const onBackHandler = () => {
    router.push("/pool");
  };

  const onSettingHandler = () => {
    setUiState({ isTransactionSettingsModalOpen: true })
  };

  return (
    <HighlightBox
      margin="auto"
      sx={{
        width: 550,
        [theme.breakpoints.down("sm")]: {
          width: "100%",
        },
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
          onMouseDown={() => setManualUpdateMode(1)}
          maxValue={balanceOne}
          setValid={setValid}
          noBorder
          value={assetOneAmount}
          setValue={
            manualUpdateMode === 1 ? setAmount("assetOneAmount") : undefined
          }
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
            value: assetOne?.getPicassoAssetId() as string || "",
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
          borderRadius="50%"
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
          onMouseDown={() => setManualUpdateMode(2)}
          maxValue={balanceTwo}
          setValid={setValid}
          noBorder
          value={assetTwoAmount}
          setValue={
            manualUpdateMode === 2 ? setAmount("assetTwoAmount") : undefined
          }
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
            value: assetTwo?.getPicassoAssetId() as string || "",
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
          price={spotPrice}
          revertPrice={new BigNumber(1).div(spotPrice)}
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

        {!needToSelectToken() && canSupply() && !invalidTokenPair() && (
          <Button
            variant="contained"
            size="large"
            fullWidth
            disabled={!valid}
            onClick={() => {
              if (!pool) {
                return enqueueSnackbar(
                  "Liquidity pool for the selected token pair does not exist.",
                  {
                    variant: "error",
                  }
                );
              }

              setUiState({ isConfirmSupplyModalOpen: true })
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
          pooledAmount1={assetOneAmount}
          pooledAmount2={assetTwoAmount}
          amount={lpReceiveAmount}
          share={share}
          mt={4}
        />
      )}

      <ConfirmSupplyModal
        pool={pool}
        lpReceiveAmount={lpReceiveAmount}
        priceOneInTwo={spotPrice}
        priceTwoInOne={new BigNumber(1).div(spotPrice)}
        assetOneAmount={assetOneAmount}
        assetTwoAmount={assetTwoAmount}
        assetOne={assetOne}
        assetTwo={assetTwo}
        share={share}
        open={isConfirmSupplyModalOpen}
      />

      <ConfirmingSupplyModal
        pool={pool}
        open={isConfirmingSupplyModalOpen}
        lpReceiveAmount={lpReceiveAmount}
        priceOneInTwo={spotPrice}
        priceTwoInOne={new BigNumber(1).div(spotPrice)}
        assetOneAmount={assetOneAmount}
        assetTwoAmount={assetTwoAmount}
        assetOne={assetOne}
        assetTwo={assetTwo}
        share={share}
      />

      <TransactionSettings showSlippageSelection={false} />
    </HighlightBox>
  );
};
