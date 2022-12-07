import {
  setSelection,
  useAddLiquiditySlice,
} from "@/store/addLiquidity/addLiquidity.slice";
import { DEFAULT_NETWORK_ID } from "@/defi/utils/constants";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useParachainApi, useSelectedAccount } from "substrate-react";
import { toChainUnits } from "@/defi/utils";
import { useAsset } from "@/defi/hooks/assets/useAsset";
import { useFilteredAssetListDropdownOptions } from "@/defi/hooks/assets/useFilteredAssetListDropdownOptions";
import { useLiquidity } from "@/defi/hooks/useLiquidity";
import { useAssetBalance } from "@/defi/hooks";
import BigNumber from "bignumber.js";
import { TokenId } from "tokens";

export const useAddLiquidityForm = () => {
  const [valid, setValid] = useState<boolean>(false);
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);

  const {
    ui: { assetOne, assetTwo, assetOneAmount, assetTwoAmount },
    pool,
    findPoolManually,
  } = useAddLiquiditySlice();

  const [spotPrice, setSpotPrice] = useState(new BigNumber(0));
  useEffect(() => {
    if (pool) {
      pool.getSpotPrice().then(setSpotPrice);
    }
  }, [pool]);

  const setAmount =
    (key: "assetOneAmount" | "assetTwoAmount") => (v: BigNumber) => {
      const otherKey =
        key == "assetOneAmount" ? "assetTwoAmount" : "assetOneAmount";
      let otherValue = new BigNumber(0);
      if (spotPrice.gt(0)) {
        otherValue =
          key == "assetOneAmount"
            ? new BigNumber(1).div(spotPrice).times(v).dp(4)
            : spotPrice.times(v).dp(4);
      }

      setSelection({ [key]: v, [otherKey]: otherValue });
    };

  const _assetOne = useAsset(assetOne);
  const _assetTwo = useAsset(assetTwo);

  const { baseAmount, quoteAmount } = useLiquidity(pool);

  const assetList1 = useFilteredAssetListDropdownOptions(assetOne);
  const assetList2 = useFilteredAssetListDropdownOptions(assetTwo);

  const balanceOne = useAssetBalance(_assetOne, "picasso");
  const balanceTwo = useAssetBalance(_assetTwo, "picasso");

  const setToken = (key: "assetOne" | "assetTwo") => (v: TokenId) => {
    setSelection({ [key]: v });
  };

  const isValidToken1 = assetOne != "none";
  const isValidToken2 = assetTwo != "none";

  const needToSelectToken = () => {
    return !isValidToken1 && !isValidToken2;
  };

  const invalidTokenPair = () => {
    return (
      (!isValidToken1 && isValidToken2) || (isValidToken1 && !isValidToken2)
    );
  };

  const canSupply = useCallback(() => {
    return assetOneAmount.lte(balanceOne) && assetTwoAmount.lte(balanceTwo);
  }, [assetOneAmount, assetTwoAmount, balanceOne, balanceTwo]);

  useEffect(() => {
    setValid(true);
    assetOne == "none" && setValid(false);
    assetTwo == "none" && setValid(false);

    new BigNumber(0).eq(assetOneAmount) && setValid(false);
    new BigNumber(0).eq(assetTwoAmount) && setValid(false);

    balanceOne.lt(assetOneAmount) && setValid(false);
    balanceTwo.lt(assetTwoAmount) && setValid(false);

    !pool && setValid(false);
  }, [
    assetOne,
    assetTwo,
    assetOneAmount,
    assetTwoAmount,
    balanceOne,
    balanceTwo,
    pool,
  ]);

  const share = useMemo(() => {
    let netAum = new BigNumber(baseAmount).plus(quoteAmount);
    let netUser = new BigNumber(assetOneAmount).plus(assetTwoAmount);

    if (netAum.eq(0)) {
      return new BigNumber(100);
    } else {
      return new BigNumber(netUser)
        .div(new BigNumber(netAum).plus(netUser))
        .times(100);
    }
  }, [baseAmount, quoteAmount, assetOneAmount, assetTwoAmount]);

  const [lpReceiveAmount, setLpReceiveAmount] = useState(new BigNumber(0));

  useEffect(() => {
    if (
      parachainApi &&
      assetOne !== "none" &&
      assetTwo !== "none" &&
      pool &&
      selectedAccount
    ) {
      const pair = pool.getPair();
      let poolBase = pair.getBaseAsset().toString();
      let poolQuote = pair.getQuoteAsset().toString();
      let isReverse = poolBase !== assetOne;
      const bnBase = toChainUnits(isReverse ? assetTwoAmount : assetOneAmount);
      const bnQuote = toChainUnits(isReverse ? assetOneAmount : assetTwoAmount);

      if (bnBase.gte(0) && bnQuote.gte(0)) {
        let b = isReverse ? poolQuote : poolBase;
        let q = isReverse ? poolBase : poolQuote;

        // parachainApi.rpc.pablo
        //   .simulateAddLiquidity(
        //     parachainApi.createType("AccountId32", selectedAccount.address),
        //     parachainApi.createType("PalletPabloPoolId", pool.getPoolId() as string),
        //     parachainApi.createType(
        //       "BTreeMap<AssetId, Balance>",
        //       {
        //         [b]: bnBase.toFixed(0),
        //         [q]: bnQuote.toFixed(0),
        //       }
        //     )
        //   )
        //   .then((expectedLP: any) => {
        //     setLpReceiveAmount(fromChainUnits(expectedLP.toString()));
        //   })
        //   .catch((err: any) => {
        //     console.log(err);
        //   });
      }
    }
  }, [
    parachainApi,
    assetOneAmount,
    assetTwoAmount,
    assetOne,
    assetTwo,
    pool,
    selectedAccount,
  ]);

  return {
    assetOne: _assetOne,
    assetTwo: _assetTwo,
    balanceOne,
    balanceTwo,
    assetOneAmount,
    assetTwoAmount,
    assetList1,
    assetList2,
    share,
    lpReceiveAmount,
    valid,
    isValidToken1,
    isValidToken2,
    setValid,
    setToken,
    setAmount,
    needToSelectToken,
    invalidTokenPair,
    canSupply,
    findPoolManually,
    spotPrice,
    pool,
  };
};
