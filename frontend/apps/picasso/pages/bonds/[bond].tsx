import { NextPage } from "next";
import { useRouter } from "next/router";
import { Box, Grid, useTheme } from "@mui/material";

import Default from "@/components/Templates/Default";
import { PageTitle } from "@/components/Molecules";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { Updater } from "@/stores/defi/polkadot/bonds/PolkadotBondsUpdater";
import { getROI } from "@/defi/polkadot/pallets/BondedFinance";
import { BondDetailSkeleton } from "@/components/Organisms/Bond/BondDetailSkeleton";
import { HighlightBoxes } from "@/components/Organisms/Bond/HighlightBoxes";
import { BondForm } from "@/components/Organisms/Bond/BondForm";
import {
  getMaxPurchasableBonds,
  getTokenString,
} from "@/components/Organisms/Bond/utils";
import { useActiveBonds } from "@/defi/polkadot/hooks/useActiveBonds";
import { ClaimForm } from "@/components/Organisms/Bond/ClaimForm";
import { useBalanceForOffer } from "@/stores/defi/polkadot/bonds/useBalanceForOffer";
import { useStore } from "@/stores/root";

const standardPageSize = {
  xs: 12,
};

const Bond: NextPage = () => {
  const theme = useTheme();
  const router = useRouter();
  const { bond } = router.query;
  const bondOffer = useStore<BondOffer>(
    (state) => state.bonds.bonds[Number(bond) - 1]
  );
  const { isLoading: isLoadingBalances, balances } =
    useBalanceForOffer(bondOffer);
  const { activeBonds, loading } = useActiveBonds();
  const maxPurchasableBond = getMaxPurchasableBonds(
    bondOffer,
    balances[bondOffer?.assetId]
  );

  if (!bondOffer || !bond || loading) {
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
        </Grid>
        <Box display="flex" gap={2} mt={4}>
          <BondForm
            hasClaim={activeBonds.length > 0}
            offerId={bond.toString()}
            standardPageSize={standardPageSize}
            maxPurchasableBonds={maxPurchasableBond}
            bondOffer={bondOffer}
            roi={roi}
            balances={balances}
            tokenSymbol={token}
            isLoadingBalances={isLoadingBalances}
          />
          <ClaimForm />
        </Box>
      </Box>
    </Default>
  );
};

export default Bond;
