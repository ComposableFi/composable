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
import { useEffect, useMemo, useState } from "react";
import BigNumber from "bignumber.js";
import { useAppSelector } from "@/hooks/store";
import { useDispatch } from "react-redux";
import {
  openConfirmSupplyModal,
  openTransactionSettingsModal,
} from "@/stores/ui/uiSlice";
import { ConfirmSupplyModal } from "./ConfirmSupplyModal";
import { PreviewSupplyModal } from "./PreviewSupplyModal";
import { ConfirmingSupplyModal } from "./ConfirmingSupplyModal";
import { TransactionSettings } from "../../TransactionSettings";
import { Assets, AssetsValidForNow, getAsset } from "@/defi/polkadot/Assets";
import { AssetId } from "@/defi/polkadot/types";
import useStore from "@/store/useStore";
import { YourPosition } from "../YourPosition";
import { PoolShare } from "./PoolShare";

export const AddLiquidityForm: React.FC<BoxProps> = ({ ...rest }) => {
  const isMobile = useMobile();
  const theme = useTheme();
  const router = useRouter();
  const dispatch = useDispatch();

  const [valid, setValid] = useState<boolean>(false);

  const {
    assets,
    addLiquidity: { pool, form, setFormField },
  } = useStore();

  const { baseAssetSelected, quoteAssetSelected, baseAmount, quoteAmount } =
    form;

  const baseAmountBn = useMemo(() => new BigNumber(baseAmount), [baseAmount]);
  const quoteAmountBn = useMemo(
    () => new BigNumber(quoteAmount),
    [quoteAmount]
  );

  const assetList1 = useMemo(() => {
    return Object.values(Assets)
      .filter((i) => {
        return (
          AssetsValidForNow.includes(i.assetId) &&
          i.assetId !== form.baseAssetSelected
        );
      })
      .map((asset) => ({
        value: asset.assetId,
        label: asset.name,
        shortLabel: asset.symbol,
        icon: asset.icon,
      }));
  }, [baseAssetSelected]);

  const assetList2 = useMemo(() => {
    return Object.values(Assets)
      .filter((i) => {
        return (
          AssetsValidForNow.includes(i.assetId) &&
          i.assetId !== form.quoteAssetSelected
        );
      })
      .map((asset) => ({
        value: asset.assetId,
        label: asset.name,
        shortLabel: asset.symbol,
        icon: asset.icon,
      }));
  }, [quoteAssetSelected]);

  const balanceQuote = useMemo(() => {
    if (form.quoteAssetSelected !== "none") {
      return new BigNumber(
        assets[form.quoteAssetSelected as AssetId].balance.picasso
      );
    } else {
      return new BigNumber(0);
    }
  }, [form.quoteAssetSelected]);

  const balanceBase = useMemo(() => {
    if (form.baseAssetSelected !== "none") {
      return new BigNumber(
        assets[form.baseAssetSelected as AssetId].balance.picasso
      );
    } else {
      return new BigNumber(0);
    }
  }, [form.baseAssetSelected]);

  const isConfirmSupplyModalOpen = useAppSelector(
    (state) => state.ui.isConfirmSupplyModalOpen
  );
  const isPreviewSupplyModalOpen = useAppSelector(
    (state) => state.ui.isPreviewSupplyModalOpen
  );
  const isConfirmingSupplyModalOpen = useAppSelector(
    (state) => state.ui.isConfirmingSupplyModalOpen
  );

  const setQuoteAmount = (v: BigNumber) => {
    setFormField({ quoteAmount: v.toString() });
  };

  const setBaseAmount = (v: BigNumber) => {
    setFormField({ baseAmount: v.toString() });
  };

  const setToken1 = (v: AssetId) => {
    setFormField({ quoteAssetSelected: v });
  };

  const setToken2 = (v: AssetId) => {
    setFormField({ baseAssetSelected: v });
  };

  const isValidToken1 = form.quoteAssetSelected != "none";
  const isValidToken2 = form.baseAssetSelected != "none";

  const needToSelectToken = () => {
    return !isValidToken1 && !isValidToken2;
  };

  const invalidTokenPair = () => {
    return (
      (!isValidToken1 && isValidToken2) || (isValidToken1 && !isValidToken2)
    );
  };

  const canSupply = () => {
    return balanceBase.gt(baseAmountBn) && balanceQuote.gt(quoteAmountBn);
  };

  const onBackHandler = () => {
    router.push("/pool");
  };

  const onSettingHandler = () => {
    dispatch(openTransactionSettingsModal());
  };

  useEffect(() => {
    setValid(true);
    form.baseAssetSelected == "none" && setValid(false);
    form.quoteAssetSelected == "none" && setValid(false);

    new BigNumber(0).eq(quoteAmount) && setValid(false);
    new BigNumber(0).eq(baseAmount) && setValid(false);

    balanceQuote.lt(quoteAmount) && setValid(false);
    balanceBase.lt(baseAmount) && setValid(false);
  }, [form, quoteAmount, baseAmount, balanceBase, balanceQuote]);

  const quoteAsset = useMemo(() => {
    return quoteAssetSelected === "none" ? null : getAsset(quoteAssetSelected);
  }, [form.quoteAssetSelected]);

  const baseAsset = useMemo(() => {
    return baseAssetSelected === "none" ? null : getAsset(baseAssetSelected);
  }, [form.baseAssetSelected]);

  const share = useMemo(() => {
    let netAum = new BigNumber(pool.balance.base).plus(pool.balance.quote);
    let netUser = new BigNumber(form.baseAmount).plus(form.quoteAmount);

    if (netAum.eq(0)) {
      return new BigNumber(100);
    } else {
      return new BigNumber(netUser)
        .div(new BigNumber(netAum).plus(netUser))
        .times(100);
    }
  }, [pool, form.baseAmount, form.quoteAmount]);

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
          maxValue={balanceQuote}
          setValid={setValid}
          noBorder
          value={quoteAmountBn}
          setValue={setQuoteAmount}
          InputProps={{
            disabled: !isValidToken1,
          }}
          buttonLabel={isValidToken1 ? "Max" : undefined}
          ButtonProps={{
            onClick: () => setQuoteAmount(balanceQuote),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          CombinedSelectProps={{
            value: quoteAssetSelected,
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
                  balance: `${balanceQuote}`,
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
          maxValue={balanceBase}
          setValid={setValid}
          noBorder
          value={baseAmountBn}
          setValue={setBaseAmount}
          InputProps={{
            disabled: !isValidToken2,
          }}
          buttonLabel={isValidToken2 ? "Max" : undefined}
          ButtonProps={{
            onClick: () => setBaseAmount(balanceBase),
            sx: {
              padding: theme.spacing(1),
            },
          }}
          CombinedSelectProps={{
            value: baseAssetSelected,
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
                  balance: `${balanceBase}`,
                }
              : undefined,
          }}
        />
      </Box>

      {valid && !invalidTokenPair() && canSupply() && (
        <PoolShare
          baseAsset={quoteAssetSelected as AssetId}
          quoteAsset={baseAssetSelected as AssetId}
          price={quoteAmountBn.div(baseAmountBn)}
          revertPrice={baseAmountBn.div(quoteAmountBn)}
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
            onClick={() => dispatch(openConfirmSupplyModal())}
          >
            Supply
          </Button>
        )}
      </Box>

      {valid && !invalidTokenPair() && canSupply() && (
        <YourPosition
          noTitle={false}
          tokenId1={quoteAssetSelected as AssetId}
          tokenId2={baseAssetSelected as AssetId}
          pooledAmount1={quoteAmountBn}
          pooledAmount2={baseAmountBn}
          amount={new BigNumber(0)}
          share={share}
          mt={4}
        />
      )}

      <ConfirmSupplyModal
        lpReceiveAmount={new BigNumber(0)}
        priceBaseInQuote={baseAmountBn.div(quoteAmountBn)}
        priceQuoteInBase={quoteAmountBn.div(baseAmountBn)}
        baseAmount={baseAmountBn}
        quoteAmount={quoteAmountBn}
        baseAsset={baseAsset}
        quoteAsset={quoteAsset}
        share={share}
        open={isConfirmSupplyModalOpen}
      />

      <PreviewSupplyModal
        open={isPreviewSupplyModalOpen}
        lpReceiveAmount={new BigNumber(0)}
        priceBaseInQuote={baseAmountBn.div(quoteAmountBn)}
        priceQuoteInBase={quoteAmountBn.div(baseAmountBn)}
        baseAmount={baseAmountBn}
        quoteAmount={quoteAmountBn}
        baseAsset={baseAsset}
        quoteAsset={quoteAsset}
        share={share}
      />

      <ConfirmingSupplyModal
        open={isConfirmingSupplyModalOpen}
        lpReceiveAmount={new BigNumber(0)}
        priceBaseInQuote={baseAmountBn.div(quoteAmountBn)}
        priceQuoteInBase={quoteAmountBn.div(baseAmountBn)}
        baseAmount={baseAmountBn}
        quoteAmount={quoteAmountBn}
        baseAsset={baseAsset}
        quoteAsset={quoteAsset}
        share={share}
      />

      <TransactionSettings />
    </Box>
  );
};
