import { useEffect, useState } from "react";
import { NextPage } from "next";
import { useRouter } from "next/router";
import { Box, Grid, useTheme } from "@mui/material";

import Default from "@/components/Templates/Default";
import { PageTitle } from "@/components/Molecules";
import PositionDetails from "@/components/Atom/PositionDetails";
import PositionDetailsRow from "@/components/Atom/PositionDetailsRow";
import { useAppSelector } from "@/hooks/store";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { Updater } from "@/stores/defi/polkadot/bonds/PolkadotBondsUpdater";
import { getROI } from "@/defi/polkadot/pallets/BondedFinance";
import { Token } from "@/defi/Tokens";
import { fetchBalanceByAssetId } from "@/defi/polkadot/pallets/Balance";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import BigNumber from "bignumber.js";
import { BondDetailSkeleton } from "@/components/Organisms/Bond/BondDetailSkeleton";
import { HighlightBoxes } from "@/components/Organisms/Bond/HighlightBoxes";
import { BondForm } from "@/components/Organisms/Bond/BondForm";
import {
  getMaxPurchasableBonds,
  getTokenString,
} from "@/components/Organisms/Bond/utils";

const standardPageSize = {
  xs: 12,
};

type BondOfferBalances = {
  [key: string]: BigNumber;
};

function useBalanceForOffer(offer: BondOffer) {
  const { parachainApi } = usePicassoProvider();
  const account = useSelectedAccount();
  const [balances, setBalances] = useState<BondOfferBalances>({});

  useEffect(() => {
    if (account && parachainApi && offer) {
      fetchBalanceByAssetId(parachainApi, account.address, offer.assetId).then(
        (result) => {
          setBalances((amount) => ({
            ...amount,
            ...{ [offer.assetId]: result },
          }));
        }
      );
    }
  }, [parachainApi, account, offer]);

  return {
    balances,
    isLoading: Object.keys(balances).length === 0,
  };
}

const Bond: NextPage = () => {
  const theme = useTheme();
  const router = useRouter();
  const { bond } = router.query;
  const bondOffer = useAppSelector<BondOffer>(
    (state) => state.bonding.bonds[Number(bond) - 1]
  );

  const { isLoading: isLoadingBalances, balances } =
    useBalanceForOffer(bondOffer);

  const maxPurchasableBond = getMaxPurchasableBonds(
    bondOffer,
    balances[bondOffer?.assetId]
  );

  if (!bondOffer || !bond) {
    return (
      <Default>
        <Updater />
        <BondDetailSkeleton />
      </Default>
    );
  }

  const token = getTokenString(bondOffer.asset);
  const toToken = getTokenString(bondOffer.reward.asset);
  const roi = getROI(bondOffer.rewardPrice, bondOffer.price);

  return (
    <Default>
      <Updater />
      <Box
        flexGrow={1}
        sx={{ mx: "auto" }}
        maxWidth={1032}
        mt={theme.spacing(9)}
      >
        <Grid container alignItems="center" gap={theme.spacing(9)}>
          <Grid item {...standardPageSize}>
            <PageTitle
              title={`${token}`}
              subtitle={`Purchase ${toToken} at a discount`}
              textAlign="center"
            />
          </Grid>
          <HighlightBoxes bondOffer={bondOffer} roi={roi} />
          <BondForm
            offerId={bond.toString()}
            standardPageSize={standardPageSize}
            maxPurchasableBonds={maxPurchasableBond}
            bondOffer={bondOffer}
            roi={roi}
            balances={balances}
            tokenSymbol={token}
            isLoadingBalances={isLoadingBalances}
          />
        </Grid>
      </Box>
    </Default>
  );
};

export default Bond;
