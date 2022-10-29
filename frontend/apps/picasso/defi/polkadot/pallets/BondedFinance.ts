import BigNumber from "bignumber.js";
import { Token, TOKENS } from "tokens";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";
import { ApiPromise } from "@polkadot/api";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { currencyIdToAssetMap } from "@/stores/defi/polkadot/bonds/constants";
import { ComposableTraitsBondedFinanceBondOffer } from "defi-interfaces";
import { Option } from "@polkadot/types-codec";
import { ITuple } from "@polkadot/types-codec/types";
import { fetchAssetPrice } from "./Oracle";
import { Executor, getSigner } from "substrate-react";
import { APP_NAME } from "@/defi/polkadot/constants";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { fromChainIdUnit } from "shared";

export function createArrayOfLength(length: number): number[] {
  return Array.from(Array(length).keys());
}

export function stringToBigNumber(value: string): BigNumber {
  return new BigNumber(value.replaceAll(",", ""));
}

export async function fetchBondOfferCount(api: ApiPromise) {
  const countBondOffers = await api.query.bondedFinance.bondOfferCount();

  // @ts-ignore
  return new BigNumber(countBondOffers.toHuman());
}

export async function fetchBonds(api: ApiPromise) {
  // Count bonded offers
  const bondOfferCount = await fetchBondOfferCount(api);
  // @ts-ignore
  const bonds: Option<
    // @ts-ignore
    ITuple<[AccountId32, ComposableTraitsBondedFinanceBondOffer]>
  >[] = await Promise.all(
    createArrayOfLength(bondOfferCount.toNumber()).map(
      (index) => api.query.bondedFinance.bondOffers(index + 1) // index + 1 is offerId
    )
  );
  const allBonds = await bonds.reduce(
    async (
      acc: Promise<BondOffer[]>,
      bond: Option<
        // @ts-ignore
        ITuple<[AccountId32, ComposableTraitsBondedFinanceBondOffer]>
      >,
      index
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
        offerId: index + 1,
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

  const nbOfBonds = stringToBigNumber(bond.nbOfBonds.toString());
  return [
    assetPriceResult.status === "fulfilled"
      ? assetPriceResult.value.multipliedBy(
          fromChainIdUnit(stringToBigNumber(bond.bondPrice.toString()))
        )
      : new BigNumber(0),
    rewardAssetPriceResult.status === "fulfilled"
      ? rewardAssetPriceResult.value.multipliedBy(
          fromChainIdUnit(
            stringToBigNumber(bond.reward.amount.toString())
          ).dividedBy(nbOfBonds)
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

function bondTransformer(beneficiary: AccountId32, bondOffer: any): BondOffer {
  return {
    bondOfferId: bondOffer.offerId,
    beneficiary,
    assetId: bondOffer.asset.toString(),
    asset: getAssets(bondOffer.asset),
    bondPrice: fromChainIdUnit(
      stringToBigNumber(bondOffer.bondPrice.toString())
    ),
    nbOfBonds: bondOffer.nbOfBonds,
    maturity: bondOffer.maturity.isFinite
      ? bondOffer.maturity.asFinite.returnIn.toString()
      : "Infinite",
    reward: {
      assetId: bondOffer.reward.asset.toString(),
      asset: getAssets(bondOffer.reward.asset),
      amount: fromChainIdUnit(
        stringToBigNumber(bondOffer.reward.amount.toString())
      ),
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
    return new BigNumber(-100);
  }

  return rewardPrice
    .minus(bondPrice)
    .abs()
    .dividedBy(bondPrice)
    .multipliedBy(100)
    .multipliedBy(rewardPrice.lt(bondPrice) ? -1 : 1);
}

export type PurchaseBond = {
  parachainApi: ApiPromise | undefined;
  account: InjectedAccountWithMeta | undefined;
  executor: Executor | undefined;
  offerId: string;
  bondInput: BigNumber;
  enqueueSnackbar: (str: string, options: any) => void;
  setOpen: (value: ((prevState: boolean) => boolean) | boolean) => void;
  setOpen2nd: (value: ((prevState: boolean) => boolean) | boolean) => void;
  handleFormReset: () => void;
};
export type ClaimType = {
  parachainApi: ApiPromise | undefined;
  vestingScheduleId?: string;
  account: InjectedAccountWithMeta | undefined;
  executor: Executor | undefined;
  assetId: string;
};

export async function claim(
  { parachainApi, account, executor, assetId, vestingScheduleId }: ClaimType,
  onSuccess: (txHash: string) => void,
  onError: (msg: string) => void,
  onStart: (txHash: string) => void
) {
  if (parachainApi && account && executor && vestingScheduleId) {
    const signer = await getSigner(APP_NAME, account.address);
    await executor.execute(
      parachainApi.tx.vesting.claim(assetId, { One: vestingScheduleId }),
      account.address,
      parachainApi,
      signer,
      (txHash) => onStart(txHash),
      (txHash) => onSuccess(txHash),
      (errorMessage) => onError(errorMessage)
    );
  }
}

export async function purchaseBond({
  parachainApi,
  account,
  executor,
  offerId,
  bondInput,
  enqueueSnackbar,
  setOpen,
  setOpen2nd,
  handleFormReset,
}: PurchaseBond) {
  if (parachainApi && account && executor) {
    try {
      const signer = await getSigner(APP_NAME, account.address);
      await executor
        .execute(
          parachainApi.tx.bondedFinance.bond(
            offerId.toString(),
            bondInput.toString(),
            true
          ),
          account.address,
          parachainApi,
          signer,
          (txHash: string) => {
            enqueueSnackbar("Processing transaction", {
              variant: "info",
              isClosable: true,
              persist: true,
              url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
            });
            setOpen(false);
            setOpen2nd(false);
          },
          (txHash: string) => {
            enqueueSnackbar("Bond transaction successful", {
              variant: "success",
              isClosable: true,
              persist: true,
              url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
            });
            handleFormReset();
          }
        )
        .catch((err) => {
          enqueueSnackbar("Bond transaction failed", {
            variant: "error",
            isClosable: true,
            description: err.message,
            persist: true,
          });
        });
    } catch (e) {
      console.log(e);
    }
  } else {
    console.log("Purchasing... no parachainAPI");
  }
}
