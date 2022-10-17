import { PairAsset, BaseAsset } from "@/components/Atoms";
import { BondPrincipalAsset } from "@/defi/types";

const BondPrincipalAssetIcon = ({
  principalAsset,
}: {
  principalAsset: BondPrincipalAsset;
}) => {
  const { lpPrincipalAsset, simplePrincipalAsset } = principalAsset;
  const { baseAsset, quoteAsset } = lpPrincipalAsset;

  if (baseAsset && quoteAsset) {
    return (
      <PairAsset
        assets={[
          {
            icon: baseAsset.icon,
            label: baseAsset.symbol,
          },
          {
            icon: quoteAsset.icon,
            label: quoteAsset.symbol,
          },
        ]}
        separator="/"
      />
    );
  }

  if (simplePrincipalAsset) {
    return (
      <BaseAsset
        label={simplePrincipalAsset.symbol}
        icon={simplePrincipalAsset.icon}
      />
    );
  }

  return null;
};

export default BondPrincipalAssetIcon;