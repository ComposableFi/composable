import { u128, u32 } from "@polkadot/types";

export type BondOffer = {
  asset: u128;
  bondPrice: u128;
  nbOfBonds: u128;
  maturity: "Infinite" | { Finite: { returnIn: u32 } };
  reward: {
    asset: u128;
    amount: u128;
    maturity: u32;
  };
};
