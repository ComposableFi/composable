import { useMemo } from "react";
import { BondPrincipalAsset } from "./useBondOffers";

export function usePrincipalAssetSymbol(
  principalAsset: BondPrincipalAsset
): string {
  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset;
  const { baseAsset, quoteAsset } = lpPrincipalAsset;

  let principalSymbol = useMemo(() => {
    return baseAsset && quoteAsset
      ? baseAsset.symbol + "/" + quoteAsset.symbol
      : simplePrincipalAsset
      ? simplePrincipalAsset.symbol
      : "-";
  }, [simplePrincipalAsset, baseAsset, quoteAsset]);

  return principalSymbol;
}
