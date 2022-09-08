import { BondPrincipalAsset } from "@/defi/types";
import { useMemo } from "react";

export default function usePrincipalAssetSymbol(
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