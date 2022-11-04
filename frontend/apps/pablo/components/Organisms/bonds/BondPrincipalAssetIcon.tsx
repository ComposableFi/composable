import { PairAsset, BaseAsset } from "@/components/Atoms";
import { LiquidityProviderToken, Asset } from "shared";

const BondPrincipalAssetIcon = ({
  bondedAsset,
}: {
  bondedAsset: LiquidityProviderToken | Asset | undefined;
}) => {
  if (bondedAsset instanceof LiquidityProviderToken) {
    return (
      <PairAsset
        assets={bondedAsset.getUnderlyingAssetJSON()}
        separator="/"
      />
    );
  }

  if (bondedAsset instanceof Asset) {
    return (
      <BaseAsset
        label={bondedAsset.getSymbol()}
        icon={bondedAsset.getIconUrl()}
      />
    );
  }

  return null;
};

export default BondPrincipalAssetIcon;