import { FC } from "react";
import { Grid } from "@mui/material";
import { BondBox } from "@/components";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import BigNumber from "bignumber.js";
import { useBondVestingInDays } from "@/defi/polkadot/hooks/useBondVestingInDays";
import { humanBalance, humanDate, LONG_HUMAN_DATE } from "@/utils/formatters";

type BoxPosition = 0 | 1 | 2 | 3;

type BoxItem = {
  [key in BoxPosition]: BoxData;
};

type BoxData = {
  title: string;
  description: string;
  discountColor?: number;
};

export const HighlightBoxes: FC<{
  bondOffer: BondOffer;
  roi: BigNumber;
}> = ({ bondOffer, roi }) => {
  const vesting = useBondVestingInDays(bondOffer);
  const bondBoxes: BoxItem = {
    0: {
      title: "Bond price",
      description: `$${humanBalance(bondOffer.price)}`,
    },
    1: {
      title: "Market price",
      description: `$${humanBalance(bondOffer.rewardPrice)}`,
    },
    2: {
      title: "Discount",
      description: `${humanBalance(roi)}%`,
      discountColor: Number(roi.toFixed(3)),
    },
    3: {
      title: "Vesting period",
      description: `${
        vesting === "Infinite"
          ? "Infinite"
          : humanDate(vesting, LONG_HUMAN_DATE)
      }`,
    },
  };
  return (
    <Grid item container spacing={3}>
      {Object.values(bondBoxes).map(({ title, description, discountColor }) => (
        <Grid item key={title} xs={3}>
          <BondBox
            title={title}
            description={description}
            discountColor={discountColor}
          />
        </Grid>
      ))}
    </Grid>
  );
};
