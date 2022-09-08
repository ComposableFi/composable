import { useMemo } from "react";
import { NextPage } from "next";
import { Button, Grid, Typography } from "@mui/material";
import { useStore } from "@/stores/root";
import Default from "@/components/Templates/Default";
import {
  gridContainerStyle,
  gridItemStyle,
} from "@/components/Organisms/Transfer/transfer-styles";
import { Header } from "@/components/Organisms/Transfer/Header";
import { TransferNetworkSelector } from "@/components/Organisms/Transfer/TransferNetworkSelector";
import { AmountTokenDropdown } from "@/components/Organisms/Transfer/AmountTokenDropdown";
import { TransferRecipientDropdown } from "@/components/Organisms/Transfer/TransferRecipientDropdown";
import { TransferFeeDisplay } from "@/components/Organisms/Transfer/TransferFeeDisplay";
import { TransferKeepAliveSwitch } from "@/components/Organisms/Transfer/TransferKeepAliveSwitch";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import { toChainIdUnit } from "shared";
import { useExecutor } from "substrate-react";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { useSnackbar } from "notistack";
import {
  getTransferToken,
  TransferHandlerArgs,
  transferKaruraPicasso,
  transferKusamaPicasso,
  transferPicassoKarura,
  transferPicassoKusama,
} from "@/components/Organisms/Transfer/xcmp";
import { TransferExistentialDeposit } from "@/components/Organisms/Transfer/TransferExistentialDeposit";
import { TransferFeeItem } from "@/components/Organisms/Transfer/TransferFeeItem";
import { AssetId } from "@/defi/polkadot/types";

const Transfers: NextPage = () => {
  const { enqueueSnackbar } = useSnackbar();
  const tokenId = useStore((state) => state.transfers.tokenId);
  const amount = useStore((state) => state.transfers.amount);
  const selectedRecipient = useStore(
    (state) => state.transfers.recipients.selected
  );

  const from = useStore((state) => state.transfers.networks.from);
  const to = useStore((state) => state.transfers.networks.to);
  const assets = useStore(
    ({ substrateBalances }) => substrateBalances[from].assets
  );
  const { hasFeeItem, feeItem } = useStore(({ transfers }) => transfers);
  const native = useStore(
    ({ substrateBalances }) => substrateBalances[from].native
  );
  const keepAlive = useStore((state) => state.transfers.keepAlive);
  const existentialDeposit = useStore(
    ({ substrateBalances }) => substrateBalances[from].native.existentialDeposit
  );

  const isNativeToNetwork = useMemo(() => {
    return assets[getTransferToken(from, to)].meta.supportedNetwork[from] === 1;
  }, [from, to, assets]);
  const balance = isNativeToNetwork ? native.balance : assets[tokenId].balance;

  const account = useSelectedAccount();
  const providers = useAllParachainProviders();
  const executor = useExecutor();

  const prepareAndCall = async (
    transferHandler: (args: TransferHandlerArgs) => Promise<void>
  ) => {
    const api = providers[from].parachainApi;

    if (!api || !executor || !account || (hasFeeItem && feeItem.length === 0)) {
      console.error("No API or Executor or account", {
        api,
        executor,
        account,
      });
      return;
    }
    const TARGET_ACCOUNT_ADDRESS = selectedRecipient.length
      ? selectedRecipient
      : account.address;

    const TARGET_PARACHAIN_ID = SUBSTRATE_NETWORKS[to].parachainId;
    // Set amount to transfer
    const amountToTransfer = api.createType(
      "u128",
      toChainIdUnit(
        keepAlive ? amount.minus(existentialDeposit) : amount
      ).toString()
    );

    const feeItemId =
      hasFeeItem && feeItem.length > 0
        ? assets[feeItem as AssetId].meta.supportedNetwork[from]
        : null;

    const signerAddress = account.address;

    await transferHandler({
      api,
      targetChain: TARGET_PARACHAIN_ID,
      targetAccount: TARGET_ACCOUNT_ADDRESS,
      amount: amountToTransfer,
      executor,
      enqueueSnackbar,
      signerAddress,
      hasFeeItem,
      feeItemId,
    });
  };

  const handleTransfer = async () => {
    let networkSpecificHandler = (_args: TransferHandlerArgs) =>
      Promise.resolve();
    switch (`${from}-${to}`) {
      case "kusama-picasso":
        networkSpecificHandler = transferKusamaPicasso;
        break;
      case "picasso-kusama":
        networkSpecificHandler = transferPicassoKusama;
        break;
      case "karura-picasso":
        networkSpecificHandler = transferKaruraPicasso;
        break;
      case "picasso-karura":
        networkSpecificHandler = transferPicassoKarura;
        break;
    }

    return prepareAndCall(networkSpecificHandler);
  };

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
        <Grid item {...gridItemStyle()}>
          <TransferFeeItem />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <TransferRecipientDropdown />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          {providers[from]?.parachainApi && <TransferFeeDisplay />}
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
            disabled={amount.lte(0) || amount.gt(balance)}
            fullWidth
            onClick={handleTransfer}
          >
            <Typography variant="button">Transfer</Typography>
          </Button>
        </Grid>
      </Grid>
    </Default>
  );
};

export default Transfers;
