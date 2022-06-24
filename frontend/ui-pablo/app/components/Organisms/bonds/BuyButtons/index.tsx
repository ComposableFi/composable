import { BaseAsset, PairAsset } from "@/components/Atoms";
import { SelectedBondOffer } from "@/defi/hooks/bonds/useBondOffer";
import { Button, Grid, GridProps } from "@mui/material";
import { useRouter } from "next/router";

const threeColumnPageSize = {
  xs: 12,
  sm: 12,
  md: 4,
};

const buttonProps = (onClick: () => void) =>
  ({
    variant: "outlined",
    fullWidth: true,
    onClick: onClick,
  } as const);

const restAssetProps = (label: string, iconSize: number) =>
  ({
    label: label,
    LabelProps: {
      variant: "body1",
      fontWeight: "normal",
    },
    iconSize: iconSize,
  } as const);

type TokenType = "token1" | "token2" | "lp";

export type BuyButtonsProps = {
  bond: SelectedBondOffer;
  iconSize?: number;
} & GridProps;
export const BuyButtons: React.FC<BuyButtonsProps> = ({
  bond,
  iconSize = 24,
  ...gridProps
}) => {
  const router = useRouter();

  const onBuyHandler = (token: TokenType) => () => {
    if (token === "lp") {
      router.push("/pool");
    } else {
      router.push("/swap");
    }
  };

  const isLpBond =
    bond.principalAsset &&
    (bond.principalAsset as any).baseAsset &&
    (bond.principalAsset as any).quoteAsset;
  if (!isLpBond) return null;

  return (
    <Grid container columnSpacing={3} {...gridProps}>
      <Grid item {...threeColumnPageSize}>
        <Button {...buttonProps(onBuyHandler("token1"))}>
          <BaseAsset
            icon={(bond.principalAsset as any).baseAsset.icon}
            {...restAssetProps(
              (bond.principalAsset as any).baseAsset.symbol,
              iconSize
            )}
          />
        </Button>
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Button {...buttonProps(onBuyHandler("token2"))}>
          <BaseAsset
            icon={(bond.principalAsset as any).quoteAsset.icon}
            {...restAssetProps(
              (bond.principalAsset as any).quoteAsset.symbol,
              iconSize
            )}
          />
        </Button>
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Button {...buttonProps(onBuyHandler("lp"))}>
          <PairAsset
            assets={[
              {
                icon: (bond.principalAsset as any).baseAsset.icon,
                label: (bond.principalAsset as any).baseAsset.symbol,
              },
              {
                icon: (bond.principalAsset as any).quoteAsset.icon,
                label: (bond.principalAsset as any).quoteAsset.symbol,
              },
            ]}
            {...restAssetProps("Create LP", iconSize)}
          />
        </Button>
      </Grid>
    </Grid>
  );
};
