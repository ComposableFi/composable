import { AmountTokenDropdown } from "@/components/Organisms/Transfer/AmountTokenDropdown";
import { Header } from "@/components/Organisms/Transfer/Header";
import {
  gridContainerStyle,
  gridItemStyle,
} from "@/components/Organisms/Transfer/transfer-styles";
import { TransferExistentialDeposit } from "@/components/Organisms/Transfer/TransferExistentialDeposit";
import { TransferFeeDisplay } from "@/components/Organisms/Transfer/TransferFeeDisplay";
import { TransferKeepAliveSwitch } from "@/components/Organisms/Transfer/TransferKeepAliveSwitch";
import { TransferNetworkSelector } from "@/components/Organisms/Transfer/TransferNetworkSelector";
import { TransferRecipientDropdown } from "@/components/Organisms/Transfer/TransferRecipientDropdown";
import Default from "@/components/Templates/Default";
import { useTransfer } from "@/defi/polkadot/hooks/useTransfer";
import { getDestChainFee } from "@/defi/polkadot/pallets/Transfer";
import { useStore } from "@/stores/root";
import { Alert, Button, Grid, Typography } from "@mui/material";
import { NextPage } from "next";
import { useEffect, useMemo } from "react";
import {
  subscribeDestinationMultiLocation,
  subscribeMultiAsset,
  subscribeTransferApiCall,
} from "@/stores/defi/polkadot/transfers/subscribers";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import BigNumber from "bignumber.js";
import {
  DESTINATION_FEE_MULTIPLIER,
  FEE_MULTIPLIER,
} from "shared/defi/constants";
import { usePendingExtrinsic } from "substrate-react";
import { InfoOutlined } from "@mui/icons-material";
import { pipe } from "fp-ts/function";
import { option } from "fp-ts";
import { TransferKSMAlert } from "@/components/Molecules";
import { PICASSO_STATEMINE_KSM_TRANSFER_FEE } from "@/defi/config";
import { fromChainIdUnit } from "shared";

const Transfers: NextPage = () => {
  const { setAmount, from, balance, transfer, to, isDirty } = useTransfer();
  const amount = useStore((state) => state.transfers.amount);
  const allProviders = useAllParachainProviders();

  const tokens = useStore((state) => state.substrateTokens.tokens);
  const isLoaded = useStore((state) => state.substrateTokens.isLoaded);
  const fee = useStore((state) => state.transfers.fee);
  const selectedToken = useStore((state) => state.transfers.selectedToken);
  const destFee = getDestChainFee(from, to, tokens, selectedToken);
  const ksmBalance = useStore(
    (state) => state.substrateBalances.balances.picasso.ksm.free
  );
  const isKSMAlertVisible =
    from === "picasso" &&
    to === "statemine" &&
    ksmBalance.lt(
      fromChainIdUnit(
        PICASSO_STATEMINE_KSM_TRANSFER_FEE,
        tokens.ksm.decimals.picasso
      )
    ) &&
    selectedToken === "usdt";

  // TODO this value can be moved to its own store subscriber
  const minValue = useMemo(() => {
    const ed = tokens[selectedToken].existentialDeposit[to];
    return pipe(
      destFee?.fee?.multipliedBy(DESTINATION_FEE_MULTIPLIER),
      option.fromNullable,
      option.chain((fee) =>
        pipe(
          ed,
          option.fromNullable,
          option.map((v) => fee.plus(v as BigNumber.Instance))
        )
      ),
      option.getOrElse(() => new BigNumber(0))
    );
  }, [tokens, to, destFee?.fee, selectedToken]);
  const feeTokenId = useStore((state) => state.transfers.getFeeToken(from));
  const selectedAccount = useSelectedAccount();
  const hasPendingXcmTransfer = usePendingExtrinsic(
    "reserveTransferAssets",
    "xcmPallet",
    selectedAccount ? selectedAccount.address : "-"
  );
  const hasPendingXTokensTransfer = usePendingExtrinsic(
    "transfer",
    "xTokens",
    selectedAccount ? selectedAccount.address : "-"
  );

  const hasPendingLimitedXcmTransfer = usePendingExtrinsic(
    "limitedReserveTransferAssets",
    "polkadotXcm",
    selectedAccount ? selectedAccount.address : "-"
  );

  const hasPendingTransfer =
    hasPendingXcmTransfer ||
    hasPendingXTokensTransfer ||
    hasPendingLimitedXcmTransfer;

  const getBalance = useStore((state) => state.substrateBalances.getBalance);
  const hasFormError = useStore((state) => state.transfers.hasFormError);

  useEffect(() => {
    if (
      allProviders[from].parachainApi &&
      selectedAccount &&
      allProviders[from].apiStatus === "connected" &&
      isLoaded
    ) {
      let subscriptions: Array<Promise<() => void>> = [];
      subscriptions.push(
        subscribeDestinationMultiLocation(allProviders, selectedAccount.address)
      );
      subscriptions.push(subscribeMultiAsset(allProviders));

      subscriptions.push(subscribeTransferApiCall(allProviders));

      return () => {
        subscriptions.forEach((sub) => sub.then((call) => call()));
      };
    }
  }, [allProviders, from, selectedAccount, isLoaded]);

  const hasEnoughGasFee = useMemo(() => {
    const feeBalance = getBalance(feeTokenId.id, from);
    return feeBalance.free.gte(fee.partialFee.multipliedBy(FEE_MULTIPLIER));
  }, [fee.partialFee, feeTokenId.id, from, getBalance]);

  useEffect(() => {
    return () => {
      // Clear form and reset everything on page change
      setAmount(new BigNumber(0));
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <Default>
      <Grid
        container
        sx={gridContainerStyle}
        maxWidth={1032}
        columns={10}
        direction="column"
        justifyContent="center"
      >
        <Grid item {...gridItemStyle("6rem")}>
          <Header />
        </Grid>
        <Grid item {...gridItemStyle()}>
          <TransferNetworkSelector disabled={hasPendingTransfer} />
        </Grid>
        <Grid item {...gridItemStyle()}>
          <AmountTokenDropdown
            disabled={hasPendingTransfer}
            isDirty={isDirty}
          />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <TransferRecipientDropdown />
        </Grid>
        {isKSMAlertVisible ? (
          <Grid item {...gridItemStyle("1.5rem")}>
            <TransferKSMAlert />
          </Grid>
        ) : null}
        <Grid item {...gridItemStyle("1.5rem")}>
          <TransferFeeDisplay />
        </Grid>
        {!hasEnoughGasFee ? (
          <Grid item {...gridItemStyle("1.5rem")}>
            <Alert
              variant="filled"
              severity="error"
              iconMapping={{
                error: <InfoOutlined color="error" />,
              }}
            >
              You do not have enough gas for this transfer, switch token or top
              up to complete transfer.
            </Alert>
          </Grid>
        ) : null}
        <Grid item {...gridItemStyle()}>
          <TransferKeepAliveSwitch />
        </Grid>
        <Grid item {...gridItemStyle()}>
          <TransferExistentialDeposit />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <Button
            variant="contained"
            color="primary"
            disabled={
              amount.lte(0) ||
              amount.gt(balance) ||
              amount.lte(minValue ?? 0) ||
              !hasEnoughGasFee ||
              hasPendingTransfer ||
              hasFormError
            }
            fullWidth
            onClick={transfer}
          >
            <Typography variant="button">Transfer</Typography>
          </Button>
          {!amount.eq(0) && amount.lte(minValue ?? 0) && (
            <Typography variant="caption" color="error.main">
              {`
              At least ${minValue?.toFormat(12) ?? "0"} ${
                tokens[selectedToken].symbol
              } should be sent over. Transferred amount should be bigger than target network's 
              existential deposit.
              `}
            </Typography>
          )}
        </Grid>
      </Grid>
    </Default>
  );
};

export default Transfers;
