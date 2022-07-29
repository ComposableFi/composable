
import { ApiPromise } from "@polkadot/api";
import { BondOffer } from "@bootstrap-pallets/types";

export function toBondOffer(api: ApiPromise, offer: any): BondOffer {
  const maturity =
    offer.maturity === "Infinite"
      ? offer.maturity
      : { Finite: { returnIn: api.createType("u32", offer.maturity.Finite.returnIn) } };

  return {
    asset: api.createType("u128", offer.asset),
    bondPrice: api.createType("u128", offer.bondPrice),
    nbOfBonds: api.createType("u128", offer.nbOfBonds),
    maturity: maturity,
    reward: {
      asset: api.createType("u128", offer.reward.asset),
      amount: api.createType("u128", offer.reward.amount),
      maturity: api.createType("u32", offer.reward.maturity)
    }
  };
}
