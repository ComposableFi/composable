import { BaseAsset, PairAsset } from "@/components/Atoms";
import { Asset, LiquidityProviderToken } from "shared";

const BondPrincipalAssetIcon = ({
  bondedAsset,
}: {
  bondedAsset: LiquidityProviderToken | Asset | undefined;
}) => {
  if (bondedAsset instanceof LiquidityProviderToken) {
    return <PairAsset assets={[]} separator="/" />;
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
