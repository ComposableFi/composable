import BigNumber from "bignumber.js";
import { TOKENS } from "@/defi/Tokens";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";
import { ApiPromise } from "@polkadot/api";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { currencyIdToAssetMap } from "@/stores/defi/polkadot/bonds/constants";
import { ComposableTraitsBondedFinanceBondOffer } from "@/defi/polkadot/interfaces";
import { Option, u128 } from "@polkadot/types-codec";
import { ITuple } from "@polkadot/types-codec/types";

export function createArrayOfLength(length: number): number[] {
  return Array.from(Array(length).keys());
}

export function stringToBigNumber(value: string): BigNumber {
  return new BigNumber(value.replaceAll(",", ""));
}

export async function fetchBondOfferCount(api: ApiPromise) {
  return await api.query.bondedFinance
    .bondOfferCount()
    .then((countBondOffers) => new BigNumber(countBondOffers.toHuman()));
}

export async function fetchBonds(api: ApiPromise) {
  // Count bonded offers
  const bondOfferCount = await fetchBondOfferCount(api);

  const bonds = await Promise.all(
    createArrayOfLength(bondOfferCount.toNumber()).map(
      (index) => api.query.bondedFinance.bondOffers(index + 1) // index + 1 is offerId
    )
  );

  const allBonds = await bonds.reduce(
    async (
      acc: Promise<BondOffer[]>,
      bond: Option<
        ITuple<[AccountId32, ComposableTraitsBondedFinanceBondOffer]>
      >
    ) => {
      const prev = await acc;
      if (bond.isNone) return prev;
      const [beneficiary, bondOffer]: [
        AccountId32,
        ComposableTraitsBondedFinanceBondOffer
      ] = bond.unwrap();
      const [price, rewardPrice] = await fetchBondPrice(api, bondOffer);
      const newBondOffer = {
        ...bondOffer,
        price,
        rewardPrice,
      };

      return [...prev, bondTransformer(beneficiary, newBondOffer)];
    },
    Promise.resolve<BondOffer[]>([])
  );

  console.table(allBonds);

  return {
    bonds: allBonds,
    bondOfferCount,
  };
}

async function fetchAssetPrice(assetId: u128, api: ApiPromise) {
  try {
    const prices: any = await api.query.oracle.prices(assetId); // TODO[type-gen]: replace any with proper type
    const obj = prices.toJSON();
    return obj ? new BigNumber(obj.price) : new BigNumber(0);
  } catch (e) {
    console.error("Defaulting to zero", e);
    return new BigNumber(0);
  }
}

async function fetchBondPrice(
  api: ApiPromise,
  bond: ComposableTraitsBondedFinanceBondOffer
) {
  const asset = bond.asset;
  const reward_asset = bond.reward.asset;

  const [assetPriceResult, rewardAssetPriceResult] = await Promise.allSettled([
    fetchAssetPrice(asset, api),
    fetchAssetPrice(reward_asset, api),
  ]);

  return [
    assetPriceResult.status === "fulfilled"
      ? assetPriceResult.value
      : new BigNumber(0),
    rewardAssetPriceResult.status === "fulfilled"
      ? rewardAssetPriceResult.value
      : new BigNumber(0),
  ];
}

function bondTransformer(beneficiary: AccountId32, bondOffer: any): BondOffer {
  return {
    beneficiary,
    asset: TOKENS[currencyIdToAssetMap[bondOffer.asset]],
    bondPrice: stringToBigNumber(bondOffer.bondPrice.toString()),
    nbOfBonds: bondOffer.nbOfBonds,
    maturity: bondOffer.maturity.Finite
      ? bondOffer.maturity.Finite.returnIn
      : "Infinite",
    reward: {
      asset: TOKENS[currencyIdToAssetMap[bondOffer.reward.asset]],
      amount: stringToBigNumber(bondOffer.reward.amount.toString()),
      maturity: new BigNumber(bondOffer.reward.maturity),
    },
    price: bondOffer.price,
    rewardPrice: bondOffer.rewardPrice,
  };
}

export function getROI(
  rewardAssetPrice: BigNumber,
  rewardAmount: BigNumber,
  oraclePrice: BigNumber,
  bondPrice: BigNumber
): BigNumber {
  const left = rewardAssetPrice.multipliedBy(rewardAmount).multipliedBy(100);

  const right = oraclePrice.multipliedBy(bondPrice);
  if (right.eq(0)) {
    return new BigNumber(0);
  }

  return left.dividedBy(right);
}
