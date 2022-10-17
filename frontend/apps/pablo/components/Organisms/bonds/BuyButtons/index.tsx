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

  const { baseAsset, quoteAsset } = bond.principalAsset.lpPrincipalAsset;
  const isLpBond = baseAsset && quoteAsset;
  if (!isLpBond) return null;

  return (
    <Grid container columnSpacing={3} {...gridProps}>
      <Grid item {...threeColumnPageSize}>
        <Button {...buttonProps(onBuyHandler("token1"))}>
          {baseAsset && (
            <BaseAsset
              icon={baseAsset.icon}
              {...restAssetProps(baseAsset.symbol, iconSize)}
            />
          )}
        </Button>
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Button {...buttonProps(onBuyHandler("token2"))}>
          {quoteAsset && (
            <BaseAsset
              icon={quoteAsset.icon}
              {...restAssetProps(quoteAsset.symbol, iconSize)}
            />
          )}
        </Button>
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Button {...buttonProps(onBuyHandler("lp"))}>
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
            {...restAssetProps("Create LP", iconSize)}
          />
        </Button>
      </Grid>
    </Grid>
  );
};
