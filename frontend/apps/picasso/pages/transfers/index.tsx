import { NextPage } from "next";
import { Button, Grid, Typography } from "@mui/material";
import Default from "@/components/Templates/Default";
import {
  gridContainerStyle,
  gridItemStyle,
} from "@/components/Organisms/Transfer/transfer-styles";
import { Header } from "@/components/Organisms/Transfer/Header";
import { TransferNetworkSelector } from "@/components/Organisms/Transfer/TransferNetworkSelector";
import { AmountTokenDropdown } from "@/components/Organisms/Transfer/AmountTokenDropdown";
import { TransferRecipientDropdown } from "@/components/Organisms/Transfer/TransferRecipientDropdown";
import { TransferKeepAliveSwitch } from "@/components/Organisms/Transfer/TransferKeepAliveSwitch";
import { TransferExistentialDeposit } from "@/components/Organisms/Transfer/TransferExistentialDeposit";
import { useTransfer } from "@/defi/polkadot/hooks/useTransfer";
import { TransferFeeDisplay } from "@/components/Organisms/Transfer/TransferFeeDisplay";
import { getDestChainFee } from "@/defi/polkadot/pallets/Transfer";
import { useStore } from "@/stores/root";

const Transfers: NextPage = () => {
  const { transfer, amount, from, balance } = useTransfer();
  // For now all transactions are done with Picasso target
  // TODO: change this to get the chainApi from target (to) in store
  const fee = useStore((state) => state.transfers.fee);
  const minValue = getDestChainFee(from, "picasso").fee.plus(fee.partialFee);
  const feeTokenId = useStore((state) => state.transfers.getFeeToken(from));

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
          <TransferNetworkSelector />
        </Grid>
        <Grid item {...gridItemStyle()}>
          <AmountTokenDropdown />
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
          <TransferExistentialDeposit network={from} />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <Button
            variant="contained"
            color="primary"
            disabled={
              amount.lte(0) || amount.gt(balance) || amount.lte(minValue)
            }
            fullWidth
            onClick={transfer}
          >
            <Typography variant="button">Transfer</Typography>
          </Button>
          {amount.lte(minValue) && (
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
