import { BaseAsset, PairAsset } from "@/components/Atoms";
import { getToken } from "@/defi/Tokens";
import { BondDetails, TokenId } from "@/defi/types";
import { Button, Grid, GridProps } from "@mui/material";
import { useState } from "react";
import { BuyLPTokenModal } from "./BuyLPTokenModal";
import { BuySingleTokenModal } from "./BuySingleTokenModal";

const threeColumnPageSize = {
  xs: 12,
  sm: 12,
  md: 4,
};

const buttonProps = (onClick: () => void) => ({
  variant: "outlined",
  fullWidth: true,
  onClick: onClick,
} as const);

const restAssetProps = (label: string, iconSize: number) => ({
  label: label,
  LabelProps: {
    variant: "body1",
    fontWeight: "normal"
  },
  iconSize: iconSize,
} as const);

type TokenType = "token1" | "token2" | "lp";

export type BuyButtonsProps = {
  bond: BondDetails;
  iconSize?: number;
} & GridProps;
export const BuyButtons: React.FC<BuyButtonsProps> = ({
  bond,
  iconSize = 24,
  ...gridProps
}) => {
  const token1 = getToken(bond.tokenId1);
  const token2 = getToken(bond.tokenId2);

  const [activeToken, setActiveToken] = useState<TokenType>("token1");
  const [isOpenBuyModal, setIsOpenBuyModal] = useState<boolean>(false);

  const isBuyLpToken = activeToken === "lp";
  const tokenIdToBuy = activeToken === "token1" ? bond.tokenId1 : bond.tokenId2;

  const onBuyHandler = (token: TokenType) => () => {
    setActiveToken(token);
    setIsOpenBuyModal(true);
  };

  const onCloseBuyModal = () => {
    setIsOpenBuyModal(false);
  };

  return (
    <Grid container columnSpacing={3} {...gridProps}>
      <Grid item {...threeColumnPageSize}>
        <Button {...buttonProps(onBuyHandler("token1"))}>
          <BaseAsset
            icon={token1.icon}
            {...restAssetProps(token1.symbol, iconSize)}
          />
        </Button>
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Button {...buttonProps(onBuyHandler("token2"))}>
          <BaseAsset
            icon={token2.icon}
            {...restAssetProps(token2.symbol, iconSize)}
          />
        </Button>
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Button {...buttonProps(onBuyHandler("lp"))}>
          <PairAsset
            assets={[
              {icon: token1.icon, label: token1.symbol},
              {icon: token2.icon, label: token2.symbol},
            ]}
            {...restAssetProps("Create LP", iconSize)}
          />
        </Button>
      </Grid>

      <BuySingleTokenModal
        dismissible
        onClose={onCloseBuyModal}
        open={!isBuyLpToken && isOpenBuyModal}
        tokenId={tokenIdToBuy}
      />

      <BuyLPTokenModal
        dismissible
        onClose={onCloseBuyModal}
        open={isBuyLpToken && isOpenBuyModal}
        bond={bond}
      />
    </Grid>
  );
};
