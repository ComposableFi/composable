import { useEffect, VoidFunctionComponent } from "react";
import { usePicassoProvider } from "@/defi/polkadot/hooks";
import BigNumber from "bignumber.js";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";
import { Token, TokenId, TOKENS } from "@/defi/Tokens";
import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { currencyIdToAssetMap } from "@/stores/defi/polkadot/bonds/constants";
import {
  createArrayOfLength,
  fetchBonds,
  stringToBigNumber,
} from "@/defi/polkadot/pallets/BondedFinance";
import { useAppDispatch } from "@/hooks/store";
import {
  setBondOfferCount,
  setBonds,
} from "@/stores/defi/polkadot/bonds/slice";

export const Updater: VoidFunctionComponent = () => {
  const { parachainApi: api, accounts } = usePicassoProvider();
  const dispatch = useAppDispatch();

  const updateBonds = async () => {
    if (!api) return;
    const { bonds, bondOfferCount } = await fetchBonds(api);
    dispatch(setBonds(bonds));
    dispatch(setBondOfferCount(bondOfferCount));
  };

  useEffect(() => {
    updateBonds();
  }, [accounts]);

  return null;
};
