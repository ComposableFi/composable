import { Box, useTheme } from "@mui/material";
import { Chart, FeaturedBox } from "@/components/Molecules";
import { useStore } from "@/stores/root";
import {
  formatNumber,
  formatNumberWithSymbol,
  formatNumberCompact,
  formatNumberCompactWithToken,
  formatNumberCompactWithSymbol,
  callIf,
  unwrapNumberOrHex,
  humanBalance,
} from "shared";
import { OverviewDataProps } from "@/stores/defi/stats/overview";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";
import { u128 } from "@polkadot/types-codec";
import { ComposableTraitsOraclePrice } from "defi-interfaces";
import { Assets } from "@/defi/polkadot/Assets";
import { fromChainIdUnit } from "@/defi/polkadot/pallets/BondedFinance";

const useCirculatingSupply = () => {
  const { parachainApi } = usePicassoProvider();
  const [circulatingSupply, setCirculatingSupply] = useState<BigNumber>(
    new BigNumber(0)
  );

  useEffect(() => {
    callIf(parachainApi, (api) => {
      api.query.balances.totalIssuance((totalIssuance: u128) =>
        setCirculatingSupply(
          fromChainIdUnit(unwrapNumberOrHex(totalIssuance.toHex()))
        )
      );
    });
  }, [parachainApi]);

  return circulatingSupply;
};

const useMarketCap = () => {
  const circulatingSupply = useCirculatingSupply();
  const [picaPrice, setPicaPrice] = useState<BigNumber>(new BigNumber(0));
  const { parachainApi } = usePicassoProvider();
  useEffect(() => {
    callIf(parachainApi, (api) => {
      api.query.oracle.prices(
        Assets.pica.supportedNetwork.picasso,
        (result: ComposableTraitsOraclePrice) => {
          if (!result.isEmpty) {
            const { price, block } = result.toJSON() as any;
            setPicaPrice(fromChainIdUnit(unwrapNumberOrHex(price)));
          }
        }
      );
    });
  }, [parachainApi]);

  return circulatingSupply.multipliedBy(picaPrice);
};

export const StatsOverviewTab: React.FC<{}> = ({}) => {
  const circulatingSupply = useCirculatingSupply();
  const marketCap = useMarketCap();
  const theme = useTheme();

  return (
    <Box
      display="grid"
      sx={{
        gridTemplateColumns: {
          xs: "1fr 1fr",
          lg: "1fr 1fr 1fr",
        },
      }}
      mb={5}
      gap={4}
    >
      <FeaturedBox
        TextAboveProps={{
          color: theme.palette.common.darkWhite,
        }}
        textAbove="Picasso market cap"
        title={`$${marketCap.toFormat(2)}`}
      />
      <FeaturedBox
        TextAboveProps={{
          color: theme.palette.common.darkWhite,
        }}
        textAbove="Picasso circulating supply"
        title={`${circulatingSupply.toFormat(0)} PICA`}
      />
    </Box>
  );
};
