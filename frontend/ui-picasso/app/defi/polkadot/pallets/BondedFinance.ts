import BigNumber from "bignumber.js";
import { Token, TOKENS } from "@/defi/Tokens";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";
import { ApiPromise } from "@polkadot/api";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { currencyIdToAssetMap } from "@/stores/defi/polkadot/bonds/constants";
import { ComposableTraitsBondedFinanceBondOffer } from "@/defi/polkadot/interfaces";
import { Option } from "@polkadot/types-codec";
import { ITuple } from "@polkadot/types-codec/types";
import { fetchAssetPrice } from "./Oracle";

export function createArrayOfLength(length: number): number[] {
  return Array.from(Array(length).keys());
}

export function stringToBigNumber(value: string): BigNumber {
  return new BigNumber(value.replaceAll(",", ""));
}

export async function fetchBondOfferCount(api: ApiPromise) {
  const countBondOffers = await api.query.bondedFinance.bondOfferCount();

  return new BigNumber(countBondOffers.toHuman());
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

  return {
    bonds: allBonds,
    bondOfferCount,
  };
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
      ? assetPriceResult.value.multipliedBy(
          fromPica(stringToBigNumber(bond.bondPrice.toString()))
        )
      : new BigNumber(0),
    rewardAssetPriceResult.status === "fulfilled"
      ? rewardAssetPriceResult.value.multipliedBy(
          fromPica(stringToBigNumber(bond.reward.amount.toString()))
        )
      : new BigNumber(0),
  ];
}

function getAssets(asset: string): Token[] | Token {
  const mapped = currencyIdToAssetMap[asset];

  const tokens = Array.isArray(mapped)
    ? mapped.flatMap((tokenId: string) => currencyIdToAssetMap[tokenId])
    : mapped;

  return Array.isArray(tokens)
    ? tokens.map((token) => TOKENS[token])
    : TOKENS[tokens];
}

export function toPica(value: number | BigNumber) {
  const bigNumberValue =
    typeof value === "number" ? new BigNumber(value) : value;

  return bigNumberValue.multipliedBy(10 ** 12);
}

export function fromPica(value: number | BigNumber) {
  return (typeof value === "number" ? new BigNumber(value) : value).dividedBy(
    toPica(1)
  );
}

function bondTransformer(beneficiary: AccountId32, bondOffer: any): BondOffer {
  return {
    beneficiary,
    asset: getAssets(bondOffer.asset),
    bondPrice: fromPica(stringToBigNumber(bondOffer.bondPrice.toString())),
    nbOfBonds: bondOffer.nbOfBonds,
    maturity: bondOffer.maturity.Finite
      ? bondOffer.maturity.Finite.returnIn
      : "Infinite",
    reward: {
      asset: getAssets(bondOffer.reward.asset),
      amount: fromPica(stringToBigNumber(bondOffer.reward.amount.toString())),
      maturity: new BigNumber(bondOffer.reward.maturity),
    },
    price: bondOffer.price,
    rewardPrice: bondOffer.rewardPrice,
  };
}

export function getROI(
  rewardPrice: BigNumber,
  bondPrice: BigNumber
): BigNumber {
  if (rewardPrice.eq(0)) {
    return new BigNumber(0);
  }

  // calculate difference in percentage between bondPrice and rewardPrice
  const diff = rewardPrice.minus(bondPrice);
  const sum = rewardPrice.plus(bondPrice);
  const avg = sum.dividedBy(2);

  return diff.dividedBy(sum);
}
