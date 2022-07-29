import { useMemo } from "react";
import { BondPrincipalAsset } from "./useBondOffers";

export function usePrincipalAssetSymbol(
  principalAsset: BondPrincipalAsset
): string {
  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset;
  const { baseAsset, quoteAsset } = lpPrincipalAsset;

  let principalSymbol = useMemo(() => {
    if (baseAsset && quoteAsset) {
      return `${baseAsset.symbol}/${quoteAsset.symbol}`;
    }
    if (simplePrincipalAsset) {
      return simplePrincipalAsset.symbol;
    }
    return "-";
  }, [simplePrincipalAsset, baseAsset, quoteAsset]);

  return principalSymbol;
}
