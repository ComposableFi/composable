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
import { Button, Grid, Typography } from "@mui/material";
import { NextPage } from "next";
import { useEffect } from "react";
import {
  subscribeDestinationMultiLocation,
  subscribeMultiAsset,
  subscribeTransferApiCall,
} from "@/stores/defi/polkadot/transfers/subscribers";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import BigNumber from "bignumber.js";
import { usePendingExtrinsic } from "substrate-react";

const Transfers: NextPage = () => {
  const { amount, setAmount, from, balance, transfer, to } = useTransfer();
  const allProviders = useAllParachainProviders();

  const tokens = useStore((state) => state.substrateTokens.tokens);
  const isLoaded = useStore((state) => state.substrateTokens.isLoaded);
  const fee = useStore((state) => state.transfers.fee);
  const minValue = getDestChainFee(from, to, tokens).fee.plus(fee.partialFee);
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

  const hasPendingTransfer = hasPendingXcmTransfer || hasPendingXTokensTransfer;

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

  useEffect(() => {
    return () => {
      // Clear form and reset everything on page change
      setAmount(new BigNumber(0));
    };
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
          <AmountTokenDropdown disabled={hasPendingTransfer} />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <TransferRecipientDropdown />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <TransferFeeDisplay />
        </Grid>
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
              amount.lte(minValue) ||
              hasPendingTransfer
            }
            fullWidth
            onClick={transfer}
          >
            <Typography variant="button">Transfer</Typography>
          </Button>
          {!amount.eq(0) && amount.lte(minValue) && (
            <Typography variant="caption" color="error.main">
              At least {minValue.toFormat(12)} {feeTokenId.symbol.toUpperCase()}{" "}
              will be spent for gas fees.
            </Typography>
          )}
        </Grid>
      </Grid>
    </Default>
  );
};

export default Transfers;
